use bevy::prelude::*;

use crate::{BuildContext, View};

use crate::node_span::NodeSpan;

/// Portal behaves just like Element, except that the generated UI nodes are unparented,
/// making them roots.
#[derive(Default)]
pub struct Portal {}

impl Portal {
    /// Construct a new, empty `Element`.
    pub fn new() -> Self {
        Self {}
    }
}

impl View for Portal {
    type State = Entity;

    fn nodes(&self, _vc: &BuildContext, _state: &Self::State) -> NodeSpan {
        NodeSpan::Empty
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        let new_entity = bc
            .world
            .spawn((
                NodeBundle {
                    visibility: Visibility::Visible,
                    ..default()
                },
                Name::new("Portal"),
            ))
            .id();
        new_entity
    }

    fn update(&self, _vc: &mut BuildContext, _state: &mut Self::State) {}

    fn assemble(&self, _vc: &mut BuildContext, _state: &mut Self::State) -> NodeSpan {
        NodeSpan::Empty
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        let mut entt = world.entity_mut(*state);
        entt.remove_parent();
        entt.despawn();
    }
}

impl Clone for Portal {
    fn clone(&self) -> Self {
        Self {}
    }
}

impl PartialEq for Portal {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
