use bevy::prelude::*;

use crate::TrackedResources;

use super::{
    local::{LocalData, TrackedLocals},
    resource::AnyRes,
};

/// Passed to `build` and `raze` methods to give access to the world and the view entity.
pub struct ElementContext<'w> {
    pub(crate) world: &'w mut World,
    pub(crate) entity: Entity,
}

/// Cx is a context parameter that is passed to presenters. It contains the presenter's
/// properties (passed from the parent presenter), plus other context information needed
/// in building the view state graph.
// TODO: Move this to it's own file once it's stable.
pub struct Cx<'w, 'p, Props = ()> {
    pub props: &'p Props,
    pub sys: &'p mut ElementContext<'w>,
    pub(crate) local_index: usize,
}

impl<'w, 'p, Props> Cx<'w, 'p, Props> {
    fn add_tracked_resource<T: Resource>(&mut self) {
        if let Some(mut tracked) = self.sys.world.get_mut::<TrackedResources>(self.sys.entity) {
            tracked.data.push(Box::new(AnyRes::<T>::new()));
        } else {
            let mut tracked = TrackedResources::default();
            tracked.data.push(Box::new(AnyRes::<T>::new()));
            self.sys.world.entity_mut(self.sys.entity).insert(tracked);
        }
    }

    pub fn use_resource<T: Resource>(&mut self) -> &T {
        self.add_tracked_resource::<T>();
        self.sys.world.resource::<T>()
    }

    pub fn use_resource_mut<T: Resource>(&mut self) -> Mut<T> {
        self.add_tracked_resource::<T>();
        self.sys.world.resource_mut::<T>()
    }

    pub fn use_local<T: Send + Sync + Clone>(&mut self, init: impl FnOnce() -> T) -> LocalData<T> {
        let index = self.local_index;
        self.local_index += 1;
        if let Some(mut tracked) = self.sys.world.get_mut::<TrackedLocals>(self.sys.entity) {
            tracked.get::<T>(index, init)
        } else {
            self.sys
                .world
                .entity_mut(self.sys.entity)
                .insert(TrackedLocals::default());
            let mut tracked = self
                .sys
                .world
                .get_mut::<TrackedLocals>(self.sys.entity)
                .unwrap();
            tracked.get::<T>(index, init)
        }
    }
}
