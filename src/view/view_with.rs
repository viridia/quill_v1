use bevy::prelude::*;

use crate::{BuildContext, View};

use crate::node_span::NodeSpan;

/// An implementtion of View that allows a callback to modify the generated elements.
pub struct ViewWith<V: View, F: Fn(EntityWorldMut) -> () + Send> {
    /// Inner view that we're going to modify
    pub(crate) inner: V,

    /// Callback function called for each output entity
    pub(crate) callback: F,
}

impl<V: View, F: Fn(EntityWorldMut) -> () + Send> ViewWith<V, F> {
    fn with_entity(callback: &F, nodes: &NodeSpan, world: &mut World) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => callback(world.entity_mut(*entity)),
            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    Self::with_entity(callback, node, world);
                }
            }
        }
    }
}

impl<V: View, F: Fn(EntityWorldMut) -> () + Send> View for ViewWith<V, F> {
    type State = V::State;

    fn nodes(&self, vc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(vc, state)
    }

    fn build(&self, vc: &mut BuildContext) -> Self::State {
        let state = self.inner.build(vc);
        Self::with_entity(&self.callback, &mut self.nodes(vc, &state), vc.world);
        state
    }

    fn update(&self, vc: &mut BuildContext, state: &mut Self::State) {
        self.inner.update(vc, state);
        Self::with_entity(&self.callback, &mut self.nodes(vc, state), vc.world);
    }

    fn assemble(&self, vc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(vc, state)
    }

    fn raze(&self, vc: &mut BuildContext, state: &mut Self::State) {
        self.inner.raze(vc, state);
    }
}
