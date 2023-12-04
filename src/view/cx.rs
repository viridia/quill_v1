use std::cell::Cell;

use bevy::prelude::*;

use crate::ViewContext;

use super::local::{LocalData, TrackedLocals};

/// Cx is a context parameter that is passed to presenters. It contains the presenter's
/// properties (passed from the parent presenter), plus other context information needed
/// in building the view state graph.
pub struct Cx</*'w, */ 'p, Props = ()> {
    /// The properties that were passed to the presenter from it's parent.
    pub props: &'p Props,
    // pub(crate) vc: &'p mut ViewContext<'w>,
    entity: Entity,
    local_index: Cell<usize>,
}

impl<'w, 'p, Props> Cx</*'w, */ 'p, Props> {
    pub(crate) fn new(
        props: &'p Props,
        entity: Entity, /*, vc: &'p mut ViewContext<'w> */
    ) -> Self {
        Self {
            props,
            entity,
            // vc,
            local_index: Cell::new(0),
        }
    }

    // /// Return a reference to the resource of the given type. Calling this function
    // /// adds the resource as a dependency of the current presenter invocation.
    // pub fn use_resource<T: Resource>(&mut self) -> &T {
    //     self.vc.add_tracked_resource::<T>();
    //     self.vc.world.resource::<T>()
    // }

    // /// Return a mutable reference to the resource of the given type. Calling this function
    // /// adds the resource as a dependency of the current presenter invocation.
    // pub fn use_resource_mut<T: Resource>(&mut self) -> Mut<T> {
    //     self.vc.add_tracked_resource::<T>();
    //     self.vc.world.resource_mut::<T>()
    // }

    // /// Return a reference to the Component `C` on the given entity.
    // pub fn use_component<C: Component>(&mut self, entity: Entity) -> Option<&C> {
    //     self.vc.add_tracked_component::<C>(entity);
    //     self.vc.world.entity(entity).get::<C>()
    // }

    // /// Return a reference to the Component `C` on the entity that contains the current
    // /// presenter invocation.
    // pub fn use_view_component<C: Component>(&mut self) -> Option<&C> {
    //     self.vc.add_tracked_component::<C>(self.vc.entity);
    //     self.vc.world.entity(self.vc.entity).get::<C>()
    // }

    // /// Return a reference to the entity that holds the current presenter invocation.
    // pub fn use_view_entity_mut(&mut self) -> EntityWorldMut<'_> {
    //     self.vc.world.entity_mut(self.vc.entity)
    // }

    // /// Return a local state variable. Calling this function also adds the state variable as
    // /// a dependency of the current presenter invocation.
    // pub fn use_local<T: Send + Clone>(&mut self, init: impl FnOnce() -> T) -> LocalData<T> {
    //     let index = self.local_index.get();
    //     self.local_index.set(index + 1);
    //     if let Some(mut tracked) = self.vc.world.get_mut::<TrackedLocals>(self.vc.entity) {
    //         tracked.get::<T>(index, init)
    //     } else {
    //         self.vc
    //             .world
    //             .entity_mut(self.vc.entity)
    //             .insert(TrackedLocals::default());
    //         let mut tracked = self
    //             .vc
    //             .world
    //             .get_mut::<TrackedLocals>(self.vc.entity)
    //             .unwrap();
    //         tracked.get::<T>(index, init)
    //     }
    // }

    // / Return an object which can be used to send a message to the current presenter.
    // pub fn use_callback<In, Marker>(&mut self, sys: impl IntoSystem<In, (), Marker>) {
    //     todo!()
    // }
}
