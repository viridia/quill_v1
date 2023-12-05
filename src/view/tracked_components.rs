use bevy::{
    ecs::{
        component::{Component, ComponentId},
        entity::Entity,
    },
    utils::HashSet,
};

/// Tracks components used by each View tree entity
#[derive(Component, Default)]
pub struct TrackedComponents {
    pub data: HashSet<(Entity, ComponentId)>,
}

// impl TrackedComponents {
//     pub(crate) fn add_component(&mut self, entity: Entity, cid: ComponentId) {
//         self.data.insert((entity, cid));
//     }
// }
