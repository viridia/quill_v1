use std::{cell::RefCell, marker::PhantomData};

use bevy::prelude::*;

use crate::{tracked_resources::TrackedResource, TrackingContext, ViewContext};

use super::atom::{AtomCell, AtomHandle, AtomMethods};

/// Cx is a context parameter that is passed to presenters. It contains the presenter's
/// properties (passed from the parent presenter), plus other context information needed
/// in building the view state graph.
pub struct Cx<'w, 'p, Props = ()> {
    /// The properties that were passed to the presenter from it's parent.
    pub props: &'p Props,
    pub(crate) vc: &'p mut ViewContext<'w>,
    /// Set of reactive resources referenced by the presenter.
    pub(crate) tracking: RefCell<&'p mut TrackingContext>,
}

impl<'w, 'p, Props> Cx<'w, 'p, Props> {
    pub(crate) fn new(
        props: &'p Props,
        vc: &'p mut ViewContext<'w>,
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

    /// Return a mutable reference to the resource of the given type. Calling this function
    /// adds the resource as a dependency of the current presenter invocation.
    pub fn use_resource_mut<T: Resource>(&mut self) -> Mut<T> {
        self.add_tracked_resource::<T>();
        self.vc.world.resource_mut::<T>()
    }

    /// Return a reference to the Component `C` on the given entity.
    pub fn use_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        self.add_tracked_component::<C>(entity);
        self.vc.world.entity(entity).get::<C>()
    }

    /// Return a reference to the Component `C` on the entity that contains the current
    /// presenter invocation.
    pub fn use_view_component<C: Component>(&self) -> Option<&C> {
        self.add_tracked_component::<C>(self.vc.entity);
        self.vc.world.entity(self.vc.entity).get::<C>()
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
        if index < tracking.owned_entities.len() {
            return tracking.owned_entities[index];
        } else if index == tracking.owned_entities.len() {
            let id = self.vc.world.spawn_empty().id();
            tracking.owned_entities.push(id);
            return id;
        } else {
            panic!("Invalid presenter entity index");
        }
    }

    /// Create an [`AtomHandle`]. This can be used to read and write the content of an atom.
    /// The handle is owned by the current context, and will be deleted when the presenter
    /// invocation is razed.
    pub fn create_atom<T: Clone + Sync + Send + 'static>(&mut self) -> AtomHandle<T> {
        let id = self.create_entity();
        AtomHandle {
            id,
            marker: PhantomData,
        }
    }

    /// Create an [`AtomHandle`] with an initial value.
    /// The handle is owned by the current context, and will be deleted when the presenter
    /// invocation is razed.
    pub fn create_atom_init<T: Clone + Sync + Send + 'static>(
        &mut self,
        init: impl FnOnce() -> T,
    ) -> AtomHandle<T> {
        let handle = self.create_atom::<T>();
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

    // / Return an object which can be used to send a message to the current presenter.
    // pub fn use_callback<In, Marker>(&mut self, sys: impl IntoSystem<In, (), Marker>) {
    //     todo!()
    // }

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
