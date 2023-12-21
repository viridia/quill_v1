use bevy::prelude::*;

use crate::{View, ViewContext};

use crate::node_span::NodeSpan;

/// A View which renders a NodeBundle that can have multiple children, with no inherent style
/// or behavior. Basically the equivalent of an HTML 'div'.
pub struct Element {}

impl Element {
    /// Construct a new, empty `Element`.
    pub fn new() -> Self {
        Self {}
    }
}

impl View for Element {
    type State = Entity;

    fn nodes(&self, _vc: &ViewContext, state: &Self::State) -> NodeSpan {
        // Return just the parent node.
        return NodeSpan::Node(*state);
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        let new_entity = vc
            .world
            .spawn((
                NodeBundle {
                    visibility: Visibility::Visible,
                    ..default()
                },
                Name::new("element"),
            ))
            .id();
        new_entity
    }

    fn update(&self, _vc: &mut ViewContext, _state: &mut Self::State) {}

    fn assemble(&self, _vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        return NodeSpan::Node(*state);
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        let mut entt = vc.entity_mut(*state);
        entt.remove_parent();
        entt.despawn();
    }
}

impl Clone for Element {
    fn clone(&self) -> Self {
        Self {}
    }
}

impl PartialEq for Element {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
