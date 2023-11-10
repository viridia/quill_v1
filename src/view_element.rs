use std::mem::swap;

use bevy::prelude::*;

use crate::{ElementContext, View};

use super::node_span::NodeSpan;

/// A Presenter type which renders a NodeBundle that can have multiple children.
pub struct Element<A: ViewTuple> {
    items: A,
}

impl<A: ViewTuple> Element<A> {
    pub fn new(items: A) -> Self {
        Self { items }
    }
}

impl<A: ViewTuple> View for Element<A> {
    type State = (A::State, Vec<NodeSpan>);

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        let count_spans = self.items.len();
        let mut child_spans = state.1.clone();
        child_spans.resize(count_spans, NodeSpan::Empty);

        // Rebuild span array, replacing ones that changed.
        self.items.build_spans(ecx, &mut state.0, &mut child_spans);
        let mut count_children: usize = 0;
        for node in child_spans.iter() {
            count_children += node.count()
        }
        let mut flat: Vec<Entity> = Vec::with_capacity(count_children);
        for node in child_spans.iter() {
            node.flatten(&mut flat);
        }

        if let NodeSpan::Node(entity) = prev {
            let mut em = ecx.world.entity_mut(*entity);
            if state.1 != child_spans {
                swap(&mut state.1, &mut child_spans);
                em.replace_children(&flat);
            }
            return NodeSpan::Node(*entity);
        }

        // Remove previous entity
        prev.despawn_recursive(ecx.world);

        let new_entity = ecx
            .world
            .spawn((NodeBundle {
                // focus_policy: FocusPolicy::Pass,
                visibility: Visibility::Visible,
                ..default()
            },))
            .push_children(&flat)
            .id();

        state.1 = child_spans;
        NodeSpan::Node(new_entity)
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, prev: &NodeSpan) {
        self.items.raze_spans(ecx, &mut state.0, &state.1);
        prev.despawn_recursive(ecx.world);
    }
}

// ViewTuple

// TODO: Turn this into a macro once it's stable.
pub trait ViewTuple: Send + Sync {
    type State: Send + Sync + Default;

    fn len(&self) -> usize;

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]);

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State, out: &[NodeSpan]);
}

impl<A: View> ViewTuple for A {
    type State = A::State;

    fn len(&self) -> usize {
        1
    }

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]) {
        out[0] = self.build(cx, state, &out[0])
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State, out: &[NodeSpan]) {
        self.raze(ecx, state, &out[0])
    }
}

impl<A: View> ViewTuple for (A,) {
    type State = (A::State,);

    fn len(&self) -> usize {
        1
    }

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]) {
        out[0] = self.0.build(cx, &mut state.0, &out[0])
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State, out: &[NodeSpan]) {
        self.0.raze(ecx, &mut state.0, &out[0]);
    }
}

impl<A0: View, A1: View> ViewTuple for (A0, A1) {
    type State = (A0::State, A1::State);

    fn len(&self) -> usize {
        2
    }

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]) {
        out[0] = self.0.build(cx, &mut state.0, &out[0]);
        out[1] = self.1.build(cx, &mut state.1, &out[1]);
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State, out: &[NodeSpan]) {
        self.0.raze(ecx, &mut state.0, &out[0]);
        self.1.raze(ecx, &mut state.1, &out[1]);
    }
}

impl<A0: View, A1: View, A2: View> ViewTuple for (A0, A1, A2) {
    type State = (A0::State, A1::State, A2::State);

    fn len(&self) -> usize {
        3
    }

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]) {
        out[0] = self.0.build(cx, &mut state.0, &out[0]);
        out[1] = self.1.build(cx, &mut state.1, &out[1]);
        out[2] = self.2.build(cx, &mut state.2, &out[2]);
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State, out: &[NodeSpan]) {
        self.0.raze(ecx, &mut state.0, &out[0]);
        self.1.raze(ecx, &mut state.1, &out[1]);
        self.2.raze(ecx, &mut state.2, &out[2]);
    }
}
