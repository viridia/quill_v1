use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{ElementContext, View};

use super::node_span::NodeSpan;

/// An implementtion of View that inserts an ECS Component on the generated elements.
pub struct ViewInsert<V: View, C: Component + Default> {
    pub(crate) inner: V,
    pub(crate) marker: PhantomData<C>,
}

impl<V: View, C: Component + Default> ViewInsert<V, C> {
    fn insert_component(nodes: &NodeSpan, ecx: &mut ElementContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut ecx.world.entity_mut(*entity);
                match em.get::<C>() {
                    Some(_) => (),
                    None => {
                        em.insert::<C>(Default::default());
                    }
                }
            }
            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    Self::insert_component(node, ecx);
                }
            }
        }
    }
}

impl<V: View, C: Component + Default> View for ViewInsert<V, C> {
    type State = V::State;

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        let mut nodes = self.inner.build(ecx, state, prev);
        Self::insert_component(&mut nodes, ecx);
        nodes
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, prev: &NodeSpan) {
        self.inner.raze(ecx, state, prev);
    }
}
