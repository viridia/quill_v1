use bevy::prelude::*;

use crate::{BuildContext, View};

use crate::node_span::NodeSpan;

/// An implementtion of View that allows a callback to modify the generated elements.
pub struct ViewWithMemo<V: View, D: Clone + PartialEq + Send, F: Fn(EntityWorldMut) + Send> {
    /// Inner view that we're going to modify
    pub(crate) inner: V,

    /// Callback function called for each output entity
    pub(crate) callback: F,

    /// Callback function called for each output entity
    pub(crate) deps: D,
}

impl<V: View, D: Clone + PartialEq + Send, F: Fn(EntityWorldMut) + Send> ViewWithMemo<V, D, F> {
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

impl<V: View, D: Clone + PartialEq + Send, F: Fn(EntityWorldMut) + Send> View
    for ViewWithMemo<V, D, F>
{
    type State = (V::State, D, NodeSpan);

    fn nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(bc, &state.0)
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        let state = self.inner.build(bc);
        let nodes = self.inner.nodes(bc, &state);
        Self::with_entity(&self.callback, &nodes, bc.world);
        (state, self.deps.clone(), nodes)
    }

    fn update(&self, bc: &mut BuildContext, state: &mut Self::State) {
        self.inner.update(bc, &mut state.0);
        let nodes = self.inner.nodes(bc, &state.0);
        if state.1 != self.deps || state.2 != nodes {
            state.1 = self.deps.clone();
            state.2 = nodes;
            Self::with_entity(&self.callback, &self.nodes(bc, state), bc.world);
        }
    }

    fn assemble(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(bc, &mut state.0)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.inner.raze(world, &mut state.0);
    }
}
