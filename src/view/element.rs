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
    type State = (A::State, Entity);

    fn nodes(&self, _ecx: &ElementContext, state: &Self::State) -> NodeSpan {
        return NodeSpan::Node(state.1);
    }

    fn build(&self, ecx: &mut ElementContext) -> Self::State {
        let state = self.items.build_spans(ecx);
        let new_entity = ecx
            .world
            .spawn((NodeBundle {
                visibility: Visibility::Visible,
                ..default()
            },))
            .id();
        (state, new_entity)
    }

    fn rebuild(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        self.items.rebuild_spans(ecx, &mut state.0);
        // self.items.rebuild_child_views(ecx, &mut state.0);
        // if changed {
        //     ecx.world
        //         .entity_mut(ecx.entity)
        //         .insert(PresenterGraphChanged);
        // }
    }

    fn collect(&self, ecx: &mut ElementContext, state: &mut Self::State) -> NodeSpan {
        // Rebuild span array, replacing ones that changed.
        let children = self.items.collect_spans(ecx, &mut state.0);
        let mut flat: Vec<Entity> = Vec::with_capacity(children.count());
        children.flatten(&mut flat);

        let mut em = ecx.world.entity_mut(state.1);
        if let Some(children) = em.get::<Children>() {
            // See if children changed
            if !children.eq(&flat) {
                em.replace_children(&flat);
            }
        } else {
            // No children, unconditional replace
            em.replace_children(&flat);
        }
        return NodeSpan::Node(state.1);
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        self.items.raze_spans(ecx, &mut state.0);
        let mut entt = ecx.world.entity_mut(state.1);
        entt.remove_parent();
        entt.despawn();
    }
}

// ViewTuple

// TODO: Turn this into a macro once it's stable.
pub trait ViewTuple: Send + Sync {
    type State: Send + Sync;

    fn len(&self) -> usize;

    fn build_spans(&self, cx: &mut ElementContext) -> Self::State;

    fn rebuild_spans(&self, cx: &mut ElementContext, state: &mut Self::State);

    fn collect_spans(&self, cx: &mut ElementContext, state: &mut Self::State) -> NodeSpan;

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State);

    // Helper function to build child views for a view.
    // fn rebuild_child_views(
    //     &self,
    //     ecx: &mut ElementContext,
    //     state_child_views: &mut Self::State,
    // ) -> bool {
    //     // Rebuild span array, replacing ones that changed.
    //     self.rebuild_spans(ecx, state_child_views);
    //     let changed = state_child_nodes.as_ref() != next_state;
    //     *state_child_nodes = next_state;
    //     changed
    // }
}

impl<A: View> ViewTuple for A {
    type State = A::State;

    fn len(&self) -> usize {
        1
    }

    fn build_spans(&self, cx: &mut ElementContext) -> Self::State {
        self.build(cx)
    }

    fn rebuild_spans(&self, cx: &mut ElementContext, state: &mut Self::State) {
        self.rebuild(cx, state)
    }

    fn collect_spans(&self, cx: &mut ElementContext, state: &mut Self::State) -> NodeSpan {
        self.collect(cx, state)
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        self.raze(ecx, state)
    }
}

impl<A: View> ViewTuple for (A,) {
    type State = (A::State,);

    fn len(&self) -> usize {
        1
    }

    fn build_spans(&self, cx: &mut ElementContext) -> Self::State {
        (self.0.build(cx),)
    }

    fn rebuild_spans(&self, cx: &mut ElementContext, state: &mut Self::State) {
        self.0.rebuild(cx, &mut state.0)
    }

    fn collect_spans(&self, cx: &mut ElementContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([self.0.collect(cx, &mut state.0)]))
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        self.0.raze(ecx, &mut state.0);
    }
}

impl<A0: View, A1: View> ViewTuple for (A0, A1) {
    type State = (A0::State, A1::State);

    fn len(&self) -> usize {
        2
    }

    fn build_spans(&self, cx: &mut ElementContext) -> Self::State {
        (self.0.build(cx), self.1.build(cx))
    }

    fn rebuild_spans(&self, cx: &mut ElementContext, state: &mut Self::State) {
        self.0.rebuild(cx, &mut state.0);
        self.1.rebuild(cx, &mut state.1);
    }

    fn collect_spans(&self, cx: &mut ElementContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            self.0.collect(cx, &mut state.0),
            self.1.collect(cx, &mut state.1),
        ]))
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        self.0.raze(ecx, &mut state.0);
        self.1.raze(ecx, &mut state.1);
    }
}

impl<A0: View, A1: View, A2: View> ViewTuple for (A0, A1, A2) {
    type State = (A0::State, A1::State, A2::State);

    fn len(&self) -> usize {
        3
    }

    fn build_spans(&self, cx: &mut ElementContext) -> Self::State {
        (self.0.build(cx), self.1.build(cx), self.2.build(cx))
    }

    fn rebuild_spans(&self, cx: &mut ElementContext, state: &mut Self::State) {
        self.0.rebuild(cx, &mut state.0);
        self.1.rebuild(cx, &mut state.1);
        self.2.rebuild(cx, &mut state.2);
    }

    fn collect_spans(&self, cx: &mut ElementContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            self.0.collect(cx, &mut state.0),
            self.1.collect(cx, &mut state.1),
            self.2.collect(cx, &mut state.2),
        ]))
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        self.0.raze(ecx, &mut state.0);
        self.1.raze(ecx, &mut state.1);
        self.2.raze(ecx, &mut state.2);
    }
}

impl<A0: View, A1: View, A2: View, A3: View> ViewTuple for (A0, A1, A2, A3) {
    type State = (A0::State, A1::State, A2::State, A3::State);

    fn len(&self) -> usize {
        4
    }

    fn build_spans(&self, cx: &mut ElementContext) -> Self::State {
        (
            self.0.build(cx),
            self.1.build(cx),
            self.2.build(cx),
            self.3.build(cx),
        )
    }

    fn rebuild_spans(&self, cx: &mut ElementContext, state: &mut Self::State) {
        self.0.rebuild(cx, &mut state.0);
        self.1.rebuild(cx, &mut state.1);
        self.2.rebuild(cx, &mut state.2);
        self.3.rebuild(cx, &mut state.3);
    }

    fn collect_spans(&self, cx: &mut ElementContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            self.0.collect(cx, &mut state.0),
            self.1.collect(cx, &mut state.1),
            self.2.collect(cx, &mut state.2),
            self.3.collect(cx, &mut state.3),
        ]))
    }

    fn raze_spans(&self, ecx: &mut ElementContext, state: &mut Self::State) {
        self.0.raze(ecx, &mut state.0);
        self.1.raze(ecx, &mut state.1);
        self.2.raze(ecx, &mut state.2);
        self.3.raze(ecx, &mut state.3);
    }
}
