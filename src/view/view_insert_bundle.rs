use std::cell::Cell;

use bevy::prelude::*;

use crate::{BuildContext, View};

use crate::node_span::NodeSpan;

/// An implementtion of [`View`] that inserts an ECS Component on the generated display entities.
///
/// The Component will only be inserted once on an entity. This happens when the entity is
/// first created, and also will happen if the output entity is replaced by a different entity.
pub struct ViewInsertBundle<V: View, B: Bundle> {
    pub(crate) inner: V,
    pub(crate) bundle: Cell<Option<B>>,
}

impl<V: View, B: Bundle> ViewInsertBundle<V, B> {
    fn insert_bundle(&self, nodes: &NodeSpan, bc: &mut BuildContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut bc.entity_mut(*entity);
                if let Some(bundle) = self.bundle.take() {
                    em.insert(bundle);
                } else {
                    panic!("No bundle to insert");
                }
            }
            NodeSpan::Fragment(ref _nodes) => {
                panic!("Can only insert into a singular node")
            }
        }
    }
}

impl<V: View, B: Bundle> View for ViewInsertBundle<V, B> {
    type State = (V::State, NodeSpan);

    fn nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(bc, &state.0)
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        let state = self.inner.build(bc);
        let nodes = self.inner.nodes(bc, &state);
        self.insert_bundle(&nodes, bc);
        (state, nodes)
    }

    fn update(&self, bc: &mut BuildContext, state: &mut Self::State) {
        self.inner.update(bc, &mut state.0);
        let nodes = self.inner.nodes(bc, &state.0);
        // Only insert the component when the output entity has changed.
        if state.1 != nodes {
            state.1 = nodes;
            self.insert_bundle(&state.1, bc);
        }
    }

    fn assemble(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(bc, &mut state.0)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.inner.raze(world, &mut state.0);
    }
}
