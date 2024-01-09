use crate::tracked_resources::TrackedResourceList;
use bevy::{
    ecs::component::{ComponentId, Tick},
    prelude::*,
    utils::HashSet,
};

pub(crate) struct TrackingContext {
    pub(crate) resources: TrackedResourceList,
    pub(crate) components: HashSet<(Entity, ComponentId)>,
    pub(crate) next_entity_index: usize,
    pub(crate) owned_entities: Vec<Entity>,
}

/// Tracks components used by each View tree entity
#[derive(Component)]
pub(crate) struct TrackedComponents {
    pub(crate) data: HashSet<(Entity, ComponentId)>,
    pub(crate) tick: Tick,
}

/// Tracks entities which were explicitly allocated by a presenter.
#[derive(Component, Default)]
pub(crate) struct OwnedEntities(pub(crate) Vec<Entity>);
