use bevy::prelude::*;

use crate::{View, ViewContext};

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

    fn nodes(&self, ecx: &ViewContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(ecx, state)
    }

    fn build(&self, ecx: &mut ViewContext) -> Self::State {
        let state = self.inner.build(ecx);
        Self::with_entity(&self.callback, &mut self.nodes(ecx, &state), ecx.world);
        state
    }

    fn update(&self, ecx: &mut ViewContext, state: &mut Self::State) {
        self.inner.update(ecx, state);
        if !self.once {
            Self::with_entity(&self.callback, &mut self.nodes(ecx, state), ecx.world);
        }
    }

    fn assemble(&self, ecx: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(ecx, state)
    }

    fn raze(&self, ecx: &mut ViewContext, state: &mut Self::State) {
        self.inner.raze(ecx, state);
    }
}
