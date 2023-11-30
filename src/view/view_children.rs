use bevy::prelude::*;

use crate::{View, ViewContext, ViewTuple};

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

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(vc, &state.0)
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        // Build state for inner view
        let st = self.inner.build(vc);
        // Build Views for each child element
        let ch = self.items.build_spans(vc);
        (st, ch)
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.update(vc, &mut state.0);
        self.items.update_spans(vc, &mut state.1);
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        let nodes = self.inner.assemble(vc, &mut state.0);
        let children = self.items.assemble_spans(vc, &mut state.1);
        if let NodeSpan::Node(parent) = nodes {
            // Attach child view outputs to parent.
            let mut flat: Vec<Entity> = Vec::with_capacity(children.count());
            children.flatten(&mut flat);

            let mut em = vc.entity_mut(parent);
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

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.items.raze_spans(vc, &mut state.1);
        self.inner.raze(vc, &mut state.0);
    }
}
