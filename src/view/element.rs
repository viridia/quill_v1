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

    fn build(&self, ecx: &mut ElementContext) -> (Self::State, NodeSpan) {
        let (state, nodes) = self.items.build_spans(ecx);
        let new_entity = ecx
            .world
            .spawn((NodeBundle {
                visibility: Visibility::Visible,
                ..default()
            },))
            .id();
        ((state, nodes), NodeSpan::Node(new_entity))
    }

    fn rebuild(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        let changed = self
            .items
            .rebuild_child_views(ecx, &mut state.0, &mut state.1);
        if changed {
            ecx.set_nodes_changed();
        }
        return prev.clone();
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, prev: &NodeSpan) {
        self.items.raze_spans(ecx, &mut state.0, &state.1);
        prev.despawn(ecx.world);
    }

    fn collect(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        nodes: &NodeSpan,
    ) -> NodeSpan {
        self.items.collect_spans(ecx, &mut state.0, &mut state.1);
        // Rebuild span array, replacing ones that changed.
        let mut count_children: usize = 0;
        for node in state.1.iter() {
            count_children += node.count()
        }
        let mut flat: Vec<Entity> = Vec::with_capacity(count_children);
        for node in state.1.iter() {
            node.flatten(&mut flat);
        }

        if let NodeSpan::Node(entity) = nodes {
            let mut em = ecx.world.entity_mut(*entity);
            if let Some(children) = em.get::<Children>() {
                // See if children changed
                if !children.eq(&flat) {
                    em.replace_children(&flat);
                }
            } else {
                // No children, unconditional replace
                em.replace_children(&flat);
            }
            return NodeSpan::Node(*entity);
        }

        panic!(
            "Expected Element NodeSpan to be a single node! {:?}",
            ecx.entity
        );
    }
}

// ViewTuple

// TODO: Turn this into a macro once it's stable.
pub trait ViewTuple: Send + Sync {
    type State: Send + Sync;

    fn len(&self) -> usize;

    fn build_spans(&self, cx: &mut ElementContext) -> (Self::State, Vec<NodeSpan>);

    fn rebuild_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]);

    fn collect_spans(&self, cx: &mut ElementContext, state: &mut Self::State, out: &mut [NodeSpan]);

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State, out: &[NodeSpan]);

    // Helper function to build child views for a view.
    fn rebuild_child_views(
        &self,
        ecx: &mut ElementContext,
        state_child_views: &mut Self::State,
        state_child_nodes: &mut Vec<NodeSpan>,
    ) -> bool {
        let mut next_state = state_child_nodes.clone();
        next_state.resize(self.len(), NodeSpan::Empty);

        // Rebuild span array, replacing ones that changed.
        self.rebuild_spans(ecx, state_child_views, &mut next_state);
        let changed = state_child_nodes.as_ref() != next_state;
        *state_child_nodes = next_state;
        changed
    }

    // Helper function to build child views for a view.
    fn collect_child_views(
        &self,
        ecx: &mut ElementContext,
        state_child_views: &mut Self::State,
        state_child_nodes: &mut Vec<NodeSpan>,
    ) -> (Vec<Entity>, bool) {
        let mut next_state = state_child_nodes.clone();
        next_state.resize(self.len(), NodeSpan::Empty);

        // Rebuild span array, replacing ones that changed.
        self.rebuild_spans(ecx, state_child_views, &mut next_state);
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

    fn build_spans(&self, cx: &mut ElementContext) -> (Self::State, Vec<NodeSpan>) {
        let (state, nodes) = self.build(cx);
        (state, vec![nodes])
    }

    fn rebuild_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.rebuild(cx, state, &out[0])
    }

    fn collect_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.collect(cx, state, &out[0])
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

    fn build_spans(&self, cx: &mut ElementContext) -> (Self::State, Vec<NodeSpan>) {
        let (state_0, out_0) = self.0.build(cx);
        ((state_0,), vec![out_0])
    }

    fn rebuild_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.0.rebuild(cx, &mut state.0, &out[0])
    }

    fn collect_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.0.collect(cx, &mut state.0, &out[0]);
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

    fn build_spans(&self, cx: &mut ElementContext) -> (Self::State, Vec<NodeSpan>) {
        let (state_0, out_0) = self.0.build(cx);
        let (state_1, out_1) = self.1.build(cx);
        ((state_0, state_1), vec![out_0, out_1])
    }

    fn rebuild_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.0.rebuild(cx, &mut state.0, &out[0]);
        out[1] = self.1.rebuild(cx, &mut state.1, &out[1]);
    }

    fn collect_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.0.collect(cx, &mut state.0, &out[0]);
        out[1] = self.1.collect(cx, &mut state.1, &out[1]);
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

    fn build_spans(&self, cx: &mut ElementContext) -> (Self::State, Vec<NodeSpan>) {
        let (state_0, out_0) = self.0.build(cx);
        let (state_1, out_1) = self.1.build(cx);
        let (state_2, out_2) = self.2.build(cx);
        ((state_0, state_1, state_2), vec![out_0, out_1, out_2])
    }

    fn rebuild_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.0.rebuild(cx, &mut state.0, &out[0]);
        out[1] = self.1.rebuild(cx, &mut state.1, &out[1]);
        out[2] = self.2.rebuild(cx, &mut state.2, &out[2]);
    }

    fn collect_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.0.collect(cx, &mut state.0, &out[0]);
        out[1] = self.1.collect(cx, &mut state.1, &out[1]);
        out[2] = self.2.collect(cx, &mut state.2, &out[2]);
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

    fn build_spans(&self, cx: &mut ElementContext) -> (Self::State, Vec<NodeSpan>) {
        let (state_0, out_0) = self.0.build(cx);
        let (state_1, out_1) = self.1.build(cx);
        let (state_2, out_2) = self.2.build(cx);
        let (state_3, out_3) = self.3.build(cx);
        (
            (state_0, state_1, state_2, state_3),
            vec![out_0, out_1, out_2, out_3],
        )
    }

    fn rebuild_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.0.rebuild(cx, &mut state.0, &out[0]);
        out[1] = self.1.rebuild(cx, &mut state.1, &out[1]);
        out[2] = self.2.rebuild(cx, &mut state.2, &out[2]);
        out[3] = self.3.rebuild(cx, &mut state.3, &out[3]);
    }

    fn collect_spans(
        &self,
        cx: &mut ElementContext,
        state: &mut Self::State,
        out: &mut [NodeSpan],
    ) {
        out[0] = self.0.collect(cx, &mut state.0, &out[0]);
        out[1] = self.1.collect(cx, &mut state.1, &out[1]);
        out[2] = self.2.collect(cx, &mut state.2, &out[2]);
        out[3] = self.3.collect(cx, &mut state.3, &out[3]);
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State, out: &[NodeSpan]) {
        self.0.raze(ecx, &mut state.0, &out[0]);
        self.1.raze(ecx, &mut state.1, &out[1]);
        self.2.raze(ecx, &mut state.2, &out[2]);
        self.3.raze(ecx, &mut state.3, &out[3]);
    }
}
