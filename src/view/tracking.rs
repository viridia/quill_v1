use crate::tracked_resources::TrackedResourceList;
use bevy::{ecs::component::ComponentId, prelude::*, utils::HashSet};

pub(crate) struct TrackingContext {
    pub(crate) resources: TrackedResourceList,
    pub(crate) components: HashSet<(Entity, ComponentId)>,
    pub(crate) next_atom_index: usize,
    pub(crate) atom_handles: Vec<Entity>,
}

/// Tracks components used by each View tree entity
#[derive(Component, Default)]
pub(crate) struct TrackedComponents {
    pub(crate) data: HashSet<(Entity, ComponentId)>,
}

/// Tracks local vars used by each presenter invocation.
#[derive(Component, Default)]
pub(crate) struct OwnedAtomHandles(pub(crate) Vec<Entity>);
