use std::{cell::RefCell, cmp::Ordering, marker::PhantomData};

use bevy::prelude::*;

use crate::{tracked_resources::TrackedResource, BuildContext, ScopedValueKey, TrackingContext};

use super::{
    atom::{AtomCell, AtomHandle, AtomMethods},
    scoped_values::ScopedValueMap,
};

/// Cx is a context parameter that is passed to presenters. It contains the presenter's
/// properties (passed from the parent presenter), plus other context information needed
/// in building the view state graph.
pub struct Cx<'w, 'p, Props = ()> {
    /// The properties that were passed to the presenter from it's parent.
    pub props: &'p Props,
    pub(crate) vc: &'p mut BuildContext<'w>,
    /// Set of reactive resources referenced by the presenter.
    pub(crate) tracking: RefCell<&'p mut TrackingContext>,
}

impl<'w, 'p, Props> Cx<'w, 'p, Props> {
    pub(crate) fn new(
        props: &'p Props,
        vc: &'p mut BuildContext<'w>,
        tracking: &'p mut TrackingContext,
    ) -> Self {
        Self {
            props,
            vc,
            tracking: RefCell::new(tracking),
        }
    }

    /// Return a reference to the resource of the given type. Calling this function
    /// adds the resource as a dependency of the current presenter invocation.
    pub fn use_resource<T: Resource>(&self) -> &T {
        self.add_tracked_resource::<T>();
        self.vc.world.resource::<T>()
    }

    /// Return a reference to the Component `C` on the given entity.
    pub fn use_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        match self.vc.world.get_entity(entity) {
            Some(c) => {
                self.add_tracked_component::<C>(entity);
                c.get::<C>()
            }
            None => None,
        }
    }

    /// Return a reference to the Component `C` on the given entity. This version does not
    /// add the component to the tracking scope, and is intended for components that update
    /// frequently.
    pub fn use_component_untracked<C: Component>(&self, entity: Entity) -> Option<&C> {
        match self.vc.world.get_entity(entity) {
            Some(c) => c.get::<C>(),
            None => None,
        }
    }

    /// Return a reference to the Component `C` on the entity that contains the current
    /// presenter invocation.
    pub fn use_view_component<C: Component>(&self) -> Option<&C> {
        self.add_tracked_component::<C>(self.vc.entity);
        self.vc.world.entity(self.vc.entity).get::<C>()
    }

    /// Run a function on the view entity. Will only re-run when [`deps`] changes.
    pub fn use_effect<F: FnOnce(EntityWorldMut), D: Clone + PartialEq + Send + Sync + 'static>(
        &mut self,
        effect: F,
        deps: D,
    ) {
        let handle = self.create_atom_handle::<D>();
        let mut entt = self.vc.world.entity_mut(handle.id);
        match entt.get_mut::<AtomCell>() {
            Some(mut cell) => {
                let deps_old = cell.0.downcast_mut::<D>().expect("Atom is incorrect type");
                if *deps_old != deps {
                    *deps_old = deps;
                    (effect)(self.vc.world.entity_mut(self.vc.entity));
                }
            }
            None => {
                entt.insert(AtomCell(Box::new(deps)));
                (effect)(self.vc.world.entity_mut(self.vc.entity));
            }
        }
    }

    /// Return a reference to the entity that holds the current presenter invocation.
    pub fn use_view_entity(&self) -> EntityRef<'_> {
        self.vc.world.entity(self.vc.entity)
    }

    /// Return a mutable reference to the entity that holds the current presenter invocation.
    pub fn use_view_entity_mut(&mut self) -> EntityWorldMut<'_> {
        self.vc.world.entity_mut(self.vc.entity)
    }

    /// Spawn an empty [`Entity`] which is owned by this presenter. The entity will be
    /// despawned when the presenter state is razed.
    pub fn create_entity(&mut self) -> Entity {
        let mut tracking = self.tracking.borrow_mut();
        let index = tracking.next_entity_index;
        tracking.next_entity_index = index + 1;
        match index.cmp(&tracking.owned_entities.len()) {
            Ordering::Less => tracking.owned_entities[index],
            Ordering::Equal => {
                let id = self.vc.world.spawn_empty().id();
                tracking.owned_entities.push(id);
                id
            }
            Ordering::Greater => panic!("Invalid presenter entity index"),
        }
    }

    /// Create an [`AtomHandle`]. This can be used to read and write the content of an atom.
    /// The handle is owned by the current context, and will be deleted when the presenter
    /// invocation is razed.
    pub fn create_atom<T: Clone + Sync + Send + Default + 'static>(&mut self) -> AtomHandle<T> {
        let handle = self.create_atom_handle::<T>();
        let mut entt = self.vc.world.entity_mut(handle.id);
        match entt.get_mut::<AtomCell>() {
            Some(_) => {}
            None => {
                entt.insert(AtomCell(Box::<T>::default()));
            }
        }
        handle
    }

    /// Create an [`AtomHandle`] with an initial value.
    /// The handle is owned by the current context, and will be deleted when the presenter
    /// invocation is razed.
    pub fn create_atom_init<T: Clone + Sync + Send + 'static>(
        &mut self,
        init: impl FnOnce() -> T,
    ) -> AtomHandle<T> {
        let handle = self.create_atom_handle::<T>();
        let mut entt = self.vc.world.entity_mut(handle.id);
        match entt.get_mut::<AtomCell>() {
            Some(_) => {}
            None => {
                entt.insert(AtomCell(Box::new(init())));
            }
        }
        handle
    }

    /// Read the value of an atom. This adds the atom to the tracking list for this
    /// presenter, so that it will re-render when the atom changes.
    pub fn read_atom<T: Clone + Sync + Send + 'static>(&self, handle: AtomHandle<T>) -> T {
        let cid = self
            .vc
            .world
            .component_id::<AtomCell>()
            .expect("Unregistered component type");
        self.tracking
            .borrow_mut()
            .components
            .insert((handle.id, cid));
        self.vc.world.get_atom(handle)
    }

    /// Write the value of an atom. Panics if the atom handle is invalid.
    pub fn write_atom<T: Clone + Sync + Send + 'static>(
        &mut self,
        handle: AtomHandle<T>,
        value: T,
    ) {
        self.vc.world.set_atom(handle, value);
    }

    /// Create a scoped value. This can be used to pass data to child presenters.
    /// The value is accessible by all child presenters.
    pub fn define_scoped_value<T: Clone + Send + Sync + PartialEq + 'static>(
        &mut self,
        key: ScopedValueKey<T>,
        value: T,
    ) {
        let mut ec = self.vc.world.entity_mut(self.vc.entity);
        match ec.get_mut::<ScopedValueMap>() {
            Some(mut ctx) => {
                if let Some(v) = ctx.0.get(&key.id()) {
                    // Don't update if value hasn't changed
                    if v.downcast_ref::<T>() == Some(&value) {
                        return;
                    }
                }
                ctx.0.insert(key.id(), Box::new(value));
            }
            None => {
                let mut map = ScopedValueMap::default();
                map.0.insert(key.id(), Box::new(value));
                ec.insert(map);
            }
        }
    }

    /// Retrieve the value of a context variable.
    pub fn get_scoped_value<T: Clone + Send + Sync + 'static>(
        &self,
        key: ScopedValueKey<T>,
    ) -> Option<T> {
        let mut entity = self.vc.entity;
        loop {
            let ec = self.vc.world.entity(entity);
            if let Some(ctx) = ec.get::<ScopedValueMap>() {
                if let Some(val) = ctx.0.get(&key.id()) {
                    let cid = self
                        .vc
                        .world
                        .component_id::<ScopedValueMap>()
                        .expect("ScopedValueMap component type is not registered");
                    self.tracking.borrow_mut().components.insert((entity, cid));
                    return val.downcast_ref::<T>().cloned();
                }
            }
            match ec.get::<Parent>() {
                Some(parent) => entity = **parent,
                _ => return None,
            }
        }
    }

    // / Return an object which can be used to send a message to the current presenter.
    // pub fn use_callback<In, Marker>(&mut self, sys: impl IntoSystem<In, (), Marker>) {
    //     todo!()
    // }

    /// Create an [`AtomHandle`]. This can be used to read and write the content of an atom.
    /// The handle is owned by the current context, and will be deleted when the presenter
    /// invocation is razed.
    fn create_atom_handle<T: Clone + Sync + Send + 'static>(&mut self) -> AtomHandle<T> {
        let id = self.create_entity();
        AtomHandle {
            id,
            marker: PhantomData,
        }
    }

    fn add_tracked_resource<T: Resource>(&self) {
        self.tracking
            .borrow_mut()
            .resources
            .push(Box::new(TrackedResource::<T>::new()));
    }

    fn add_tracked_component<C: Component>(&self, entity: Entity) {
        let cid = self
            .vc
            .world
            .component_id::<C>()
            .expect("Unregistered component type");
        self.tracking.borrow_mut().components.insert((entity, cid));
    }
}
