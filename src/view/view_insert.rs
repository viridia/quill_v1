use bevy::prelude::*;

use crate::{ElementContext, View};

use crate::node_span::NodeSpan;

/// An implementtion of View that inserts an ECS Component on the generated elements.
pub struct ViewInsert<V: View, C: Component> {
    pub(crate) inner: V,
    pub(crate) component: C,
}

impl<V: View, C: Component + Clone> ViewInsert<V, C> {
    fn insert_component(component: &C, nodes: &NodeSpan, ecx: &mut ElementContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut ecx.world.entity_mut(*entity);
                match em.get::<C>() {
                    Some(_) => {
                        // TODO: Compare and see if changed.
                    }
                    None => {
                        em.insert(component.clone());
                    }
                }
            }
            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    Self::insert_component(component, node, ecx);
                }
            }
        }
    }
}

impl<V: View, C: Component + Clone> View for ViewInsert<V, C> {
    type State = V::State;

    fn nodes(&self, ecx: &ElementContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(ecx, state)
    }

    fn build(&self, ecx: &mut ElementContext) -> Self::State {
        let state = self.inner.build(ecx);
        Self::insert_component(&self.component, &mut self.inner.nodes(ecx, &state), ecx);
        state
    }

    fn rebuild(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        self.inner.rebuild(ecx, state);
        Self::insert_component(&self.component, &mut self.nodes(ecx, state), ecx);
    }

    fn collect(&self, ecx: &mut ElementContext, state: &mut Self::State) -> NodeSpan {
        self.inner.collect(ecx, state)
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        self.inner.raze(ecx, state);
    }
}
