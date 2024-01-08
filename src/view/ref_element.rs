use bevy::prelude::*;

use crate::{BuildContext, View};

use crate::node_span::NodeSpan;

/// A View which renders a NodeBundle that can have multiple children, with no inherent style
/// or behavior. Basically the equivalent of an HTML 'div'.
///
/// Unlike [`Element`], [`RefElement`] accepts as a parameter a pre-allocated entity id, which
/// is used for the output display node. This is intended for cases where widgets contain references
/// to other widgets. By allowing the id to be generated before the UI is constructed, the id
/// can be passed to other widgets as a parameter.
pub struct RefElement {
    id: Entity,
}

impl RefElement {
    /// Construct a new, empty `Element`.
    ///
    /// Arguments:
    /// * `id` - The entity id for this node.
    pub fn new(id: Entity) -> Self {
        Self { id }
    }
}

impl View for RefElement {
    type State = ();

    fn nodes(&self, _vc: &BuildContext, _state: &Self::State) -> NodeSpan {
        // Return just the parent node.
        NodeSpan::Node(self.id)
    }

    fn build(&self, vc: &mut BuildContext) -> Self::State {
        vc.world.entity_mut(self.id).insert((NodeBundle {
            visibility: Visibility::Visible,
            ..default()
        },));
    }

    fn update(&self, _vc: &mut BuildContext, _state: &mut Self::State) {}

    fn assemble(&self, _vc: &mut BuildContext, _state: &mut Self::State) -> NodeSpan {
        NodeSpan::Node(self.id)
    }

    fn raze(&self, world: &mut World, _state: &mut Self::State) {
        let mut entt = world.entity_mut(self.id);
        entt.remove_parent();
        // We want to remove the components but keep the id
        entt.remove::<Node>().remove::<Visibility>();
    }
}

impl Clone for RefElement {
    fn clone(&self) -> Self {
        Self { id: self.id }
    }
}

impl PartialEq for RefElement {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
