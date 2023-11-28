use bevy::prelude::*;

use crate::{View, ViewContext};

use crate::node_span::NodeSpan;

/// An implementtion of View that allows a callback to modify the generated elements.
pub struct ViewWith<V: View, F: Fn(Entity, &mut World) -> () + 'static + Send> {
    /// Inner view that we're going to modify
    pub(crate) inner: V,

    /// Callback function called for each output entity
    pub(crate) callback: F,

    /// Whether the callback should only be called once when nodes are first created, or
    /// on every rebuild.
    pub(crate) once: bool,
}

impl<V: View, F: Fn(Entity, &mut World) -> () + 'static + Send> ViewWith<V, F> {
    fn with_entity(callback: &F, nodes: &NodeSpan, vc: &mut World) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => callback(*entity, vc),
            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    Self::with_entity(callback, node, vc);
                }
            }
        }
    }
}

impl<V: View, F: Fn(Entity, &mut World) -> () + 'static + Send> View for ViewWith<V, F> {
    type State = V::State;

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(vc, state)
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        let state = self.inner.build(vc);
        Self::with_entity(&self.callback, &mut self.nodes(vc, &state), vc.world);
        state
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.update(vc, state);
        if !self.once {
            Self::with_entity(&self.callback, &mut self.nodes(vc, state), vc.world);
        }
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(vc, state)
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.raze(vc, state);
    }
}
