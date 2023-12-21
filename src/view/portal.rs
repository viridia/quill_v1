use bevy::prelude::*;

use crate::{View, ViewContext};

use crate::node_span::NodeSpan;

/// Portal behaves just like Element, except that the generated UI nodes are unparented,
/// making them roots.
pub struct Portal {}

impl Portal {
    /// Construct a new, empty `Element`.
    pub fn new() -> Self {
        Self {}
    }
}

impl View for Portal {
    type State = Entity;

    fn nodes(&self, _vc: &ViewContext, _state: &Self::State) -> NodeSpan {
        return NodeSpan::Empty;
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        let new_entity = vc
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

    fn update(&self, _vc: &mut ViewContext, _state: &mut Self::State) {}

    fn assemble(&self, _vc: &mut ViewContext, _state: &mut Self::State) -> NodeSpan {
        return NodeSpan::Empty;
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        let mut entt = vc.entity_mut(*state);
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
