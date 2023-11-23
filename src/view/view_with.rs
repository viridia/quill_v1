use bevy::prelude::*;

use crate::{ElementContext, View};

use crate::node_span::NodeSpan;

/// An implementtion of View that allows a callback to modify the generated elements.
pub struct ViewWith<V: View, F: Fn(Entity, &mut World) -> () + 'static + Send + Sync> {
    /// Inner view that we're going to modify
    pub(crate) inner: V,

    /// Callback function called for each output entity
    pub(crate) callback: F,

    /// Whether the callback should only be called once when nodes are first created, or
    /// on every rebuild.
    pub(crate) once: bool,
}

impl<V: View, F: Fn(Entity, &mut World) -> () + 'static + Send + Sync> ViewWith<V, F> {
    fn with_entity(callback: &F, nodes: &NodeSpan, ecx: &mut World) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => callback(*entity, ecx),
            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    Self::with_entity(callback, node, ecx);
                }
            }
        }
    }
}

impl<V: View, F: Fn(Entity, &mut World) -> () + 'static + Send + Sync> View for ViewWith<V, F> {
    type State = V::State;

    fn build(&self, ecx: &mut ElementContext) -> (Self::State, NodeSpan) {
        let (state, mut nodes) = self.inner.build(ecx);
        Self::with_entity(&self.callback, &mut nodes, ecx.world);
        (state, nodes)
    }

    fn rebuild(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        nodes_prev: &NodeSpan,
    ) -> NodeSpan {
        let mut nodes = self.inner.rebuild(ecx, state, nodes_prev);
        if !self.once || nodes != *nodes_prev {
            Self::with_entity(&self.callback, &mut nodes, ecx.world);
        }
        nodes
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, nodes: &NodeSpan) {
        self.inner.raze(ecx, state, nodes);
        // *nodes = NodeSpan::Empty;
    }

    fn collect(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        nodes: &NodeSpan,
    ) -> NodeSpan {
        self.inner.collect(ecx, state, nodes)
    }
}
