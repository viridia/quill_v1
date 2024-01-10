use bevy::prelude::*;

use crate::{BuildContext, View, ViewTuple};

use crate::node_span::NodeSpan;

/// An implementtion of View that allows a callback to modify the generated elements.
pub struct ViewChildren<V: View, A: ViewTuple> {
    /// Inner view that we're going to modify
    pub(crate) inner: V,

    /// List of child views.
    pub(crate) items: A,
}

impl<V: View, A: ViewTuple> View for ViewChildren<V, A> {
    type State = (V::State, A::State);

    fn nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(bc, &state.0)
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        // Build state for inner view
        let st = self.inner.build(bc);
        // Build Views for each child element
        let ch = self.items.build_spans(bc);
        (st, ch)
    }

    fn update(&self, bc: &mut BuildContext, state: &mut Self::State) {
        self.inner.update(bc, &mut state.0);
        self.items.update_spans(bc, &mut state.1);
    }

    fn assemble(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        let nodes = self.inner.assemble(bc, &mut state.0);
        let children = self.items.assemble_spans(bc, &mut state.1);
        if let NodeSpan::Node(parent) = nodes {
            // Attach child view outputs to parent.
            let mut flat: Vec<Entity> = Vec::with_capacity(children.count());
            children.flatten(&mut flat);

            let mut em = bc.entity_mut(parent);
            if let Some(children) = em.get::<Children>() {
                // See if children changed
                if !children.eq(&flat) {
                    em.replace_children(&flat);
                }
            } else {
                // No children, unconditional replace
                em.replace_children(&flat);
            }
        } else if nodes != NodeSpan::Empty {
            panic!("Children can only be parented to a single node");
        }
        nodes
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.items.raze_spans(world, &mut state.1);
        self.inner.raze(world, &mut state.0);
    }
}

impl<V: View + PartialEq, A: ViewTuple + PartialEq> PartialEq for ViewChildren<V, A> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.items == other.items
    }
}

impl<V: View + Clone, A: ViewTuple + Clone> Clone for ViewChildren<V, A> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            items: self.items.clone(),
        }
    }
}
