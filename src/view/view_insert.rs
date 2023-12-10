use bevy::prelude::*;

use crate::{View, ViewContext};

use crate::node_span::NodeSpan;

/// An implementtion of [`View`] that inserts an ECS Component on the generated display entities.
///
/// The Component will only be inserted once on an entity. This happens when the entity is
/// first created, and also will happen if the output entity is replaced by a different entity.
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
                em.insert(component.clone());
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
    type State = (V::State, NodeSpan);

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(vc, &state.0)
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        let state = self.inner.build(vc);
        let mut nodes = self.inner.nodes(vc, &state);
        Self::insert_component(&self.component, &mut nodes, vc);
        (state, nodes)
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.update(vc, &mut state.0);
        let nodes = self.inner.nodes(vc, &state.0);
        // Only insert the component when the output entity has changed.
        if state.1 != nodes {
            state.1 = nodes;
            Self::insert_component(&self.component, &mut state.1, vc);
        }
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(vc, &mut state.0)
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.raze(vc, &mut state.0);
    }
}
