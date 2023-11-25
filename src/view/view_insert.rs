use bevy::prelude::*;

use crate::{View, ViewContext};

use crate::node_span::NodeSpan;

/// An implementtion of View that inserts an ECS Component on the generated elements.
pub struct ViewInsert<V: View, C: Component> {
    pub(crate) inner: V,
    pub(crate) component: C,
}

impl<V: View, C: Component + Clone> ViewInsert<V, C> {
    fn insert_component(component: &C, nodes: &NodeSpan, vc: &mut ViewContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut vc.entity_mut(*entity);
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
                    Self::insert_component(component, node, vc);
                }
            }
        }
    }
}

impl<V: View, C: Component + Clone> View for ViewInsert<V, C> {
    type State = V::State;

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(vc, state)
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        let state = self.inner.build(vc);
        Self::insert_component(&self.component, &mut self.inner.nodes(vc, &state), vc);
        state
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.update(vc, state);
        Self::insert_component(&self.component, &mut self.nodes(vc, state), vc);
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(vc, state)
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.raze(vc, state);
    }
}
