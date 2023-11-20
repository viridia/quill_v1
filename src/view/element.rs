use bevy::prelude::*;

use crate::{ElementContext, View};

use crate::node_span::NodeSpan;

/// A View which renders a NodeBundle that can have multiple children, with no inherent style
/// or behavior. Basically the equivalent of an HTML 'div'.
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
        let (flat, changed) = self
            .items
            .build_child_views(ecx, &mut state.0, &mut state.1);
        if let NodeSpan::Node(entity) = prev {
            let mut em = ecx.world.entity_mut(*entity);
            if changed {
                em.replace_children(&flat);
            }
            return NodeSpan::Node(*entity);
        }

        let new_entity = ecx
            .world
            .spawn((NodeBundle {
                visibility: Visibility::Visible,
                ..default()
            },))
            .replace_children(&flat)
            .id();

        // Remove previous entity and any remaining children
        prev.despawn_recursive(ecx.world);

        // state.1 = next_state;
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

    // Helper function to build child views for a view.
    fn build_child_views(
        &self,
        ecx: &mut ElementContext,
        state_child_views: &mut Self::State,
        state_child_nodes: &mut Vec<NodeSpan>,
    ) -> (Vec<Entity>, bool) {
        let mut next_state = state_child_nodes.clone();
        next_state.resize(self.len(), NodeSpan::Empty);

        // Rebuild span array, replacing ones that changed.
        self.build_spans(ecx, state_child_views, &mut next_state);
        let mut count_children: usize = 0;
        for node in next_state.iter() {
            count_children += node.count()
        }
        let mut flat: Vec<Entity> = Vec::with_capacity(count_children);
        for node in next_state.iter() {
            node.flatten(&mut flat);
        }

        let changed = state_child_nodes.as_ref() != next_state;
        *state_child_nodes = next_state;
        (flat, changed)
    }
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

impl<A0: View, A1: View, A2: View, A3: View> ViewTuple for (A0, A1, A2, A3) {
    type State = (A0::State, A1::State, A2::State, A3::State);

    fn len(&self) -> usize {
        4
    }

    fn build_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]) {
        out[0] = self.0.build(cx, &mut state.0, &out[0]);
        out[1] = self.1.build(cx, &mut state.1, &out[1]);
        out[2] = self.2.build(cx, &mut state.2, &out[2]);
        out[3] = self.3.build(cx, &mut state.3, &out[3]);
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State, out: &[NodeSpan]) {
        self.0.raze(ecx, &mut state.0, &out[0]);
        self.1.raze(ecx, &mut state.1, &out[1]);
        self.2.raze(ecx, &mut state.2, &out[2]);
        self.3.raze(ecx, &mut state.3, &out[3]);
    }
}
