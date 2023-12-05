use crate::tracked_resources::TrackedResourceList;
use bevy::{ecs::component::ComponentId, prelude::*, utils::HashSet};

pub(crate) struct TrackingContext {
    pub(crate) resources: TrackedResourceList,
    pub(crate) components: HashSet<(Entity, ComponentId)>,
    pub(crate) local_index: usize,
}

/// Tracks components used by each View tree entity
#[derive(Component, Default)]
pub struct TrackedComponents {
    pub data: HashSet<(Entity, ComponentId)>,
}
