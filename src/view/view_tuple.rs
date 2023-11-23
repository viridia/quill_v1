use crate::{View, ViewContext};

use crate::node_span::NodeSpan;

// ViewTuple

// TODO: Turn this into a macro once it's stable.
pub trait ViewTuple: Send + Sync {
    type State: Send + Sync;

    /// Return the number of child views.
    fn len(&self) -> usize;

    /// Return the output nodes for all spans.
    fn span_nodes(&self, cx: &ViewContext, state: &Self::State) -> NodeSpan;

    /// Build the child views.
    fn build_spans(&self, cx: &mut ViewContext) -> Self::State;

    /// Update the child views.
    fn update_spans(&self, cx: &mut ViewContext, state: &mut Self::State);

    /// Assemble the child views.
    fn assemble_spans(&self, cx: &mut ViewContext, state: &mut Self::State) -> NodeSpan;

    /// Despawn the child views.
    fn raze_spans(&self, vc: &mut ViewContext, state: &mut Self::State);
}

impl<A: View> ViewTuple for A {
    type State = A::State;

    fn len(&self) -> usize {
        1
    }

    fn span_nodes(&self, cx: &ViewContext, state: &Self::State) -> NodeSpan {
        self.nodes(cx, state)
    }

    fn build_spans(&self, cx: &mut ViewContext) -> Self::State {
        self.build(cx)
    }

    fn update_spans(&self, cx: &mut ViewContext, state: &mut Self::State) {
        self.update(cx, state)
    }

    fn assemble_spans(&self, cx: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.assemble(cx, state)
    }

    fn raze_spans(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.raze(vc, state)
    }
}

impl<A: View> ViewTuple for (A,) {
    type State = (A::State,);

    fn len(&self) -> usize {
        1
    }

    fn span_nodes(&self, cx: &ViewContext, state: &Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([self.0.nodes(cx, &state.0)]))
    }

    fn build_spans(&self, cx: &mut ViewContext) -> Self::State {
        (self.0.build(cx),)
    }

    fn update_spans(&self, cx: &mut ViewContext, state: &mut Self::State) {
        self.0.update(cx, &mut state.0)
    }

    fn assemble_spans(&self, cx: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([self.0.assemble(cx, &mut state.0)]))
    }

    fn raze_spans(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.0.raze(vc, &mut state.0);
    }
}

impl<A0: View, A1: View> ViewTuple for (A0, A1) {
    type State = (A0::State, A1::State);

    fn len(&self) -> usize {
        2
    }

    fn span_nodes(&self, cx: &ViewContext, state: &Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            self.0.nodes(cx, &state.0),
            self.1.nodes(cx, &state.1),
        ]))
    }

    fn build_spans(&self, cx: &mut ViewContext) -> Self::State {
        (self.0.build(cx), self.1.build(cx))
    }

    fn update_spans(&self, cx: &mut ViewContext, state: &mut Self::State) {
        self.0.update(cx, &mut state.0);
        self.1.update(cx, &mut state.1);
    }

    fn assemble_spans(&self, cx: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            self.0.assemble(cx, &mut state.0),
            self.1.assemble(cx, &mut state.1),
        ]))
    }

    fn raze_spans(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.0.raze(vc, &mut state.0);
        self.1.raze(vc, &mut state.1);
    }
}

impl<A0: View, A1: View, A2: View> ViewTuple for (A0, A1, A2) {
    type State = (A0::State, A1::State, A2::State);

    fn len(&self) -> usize {
        3
    }

    fn span_nodes(&self, cx: &ViewContext, state: &Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            self.0.nodes(cx, &state.0),
            self.1.nodes(cx, &state.1),
            self.2.nodes(cx, &state.2),
        ]))
    }

    fn build_spans(&self, cx: &mut ViewContext) -> Self::State {
        (self.0.build(cx), self.1.build(cx), self.2.build(cx))
    }

    fn update_spans(&self, cx: &mut ViewContext, state: &mut Self::State) {
        self.0.update(cx, &mut state.0);
        self.1.update(cx, &mut state.1);
        self.2.update(cx, &mut state.2);
    }

    fn assemble_spans(&self, cx: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            self.0.assemble(cx, &mut state.0),
            self.1.assemble(cx, &mut state.1),
            self.2.assemble(cx, &mut state.2),
        ]))
    }

    fn raze_spans(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.0.raze(vc, &mut state.0);
        self.1.raze(vc, &mut state.1);
        self.2.raze(vc, &mut state.2);
    }
}

impl<A0: View, A1: View, A2: View, A3: View> ViewTuple for (A0, A1, A2, A3) {
    type State = (A0::State, A1::State, A2::State, A3::State);

    fn len(&self) -> usize {
        4
    }

    fn span_nodes(&self, cx: &ViewContext, state: &Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            self.0.nodes(cx, &state.0),
            self.1.nodes(cx, &state.1),
            self.2.nodes(cx, &state.2),
            self.3.nodes(cx, &state.3),
        ]))
    }

    fn build_spans(&self, cx: &mut ViewContext) -> Self::State {
        (
            self.0.build(cx),
            self.1.build(cx),
            self.2.build(cx),
            self.3.build(cx),
        )
    }

    fn update_spans(&self, cx: &mut ViewContext, state: &mut Self::State) {
        self.0.update(cx, &mut state.0);
        self.1.update(cx, &mut state.1);
        self.2.update(cx, &mut state.2);
        self.3.update(cx, &mut state.3);
    }

    fn assemble_spans(&self, cx: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            self.0.assemble(cx, &mut state.0),
            self.1.assemble(cx, &mut state.1),
            self.2.assemble(cx, &mut state.2),
            self.3.assemble(cx, &mut state.3),
        ]))
    }

    fn raze_spans(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.0.raze(vc, &mut state.0);
        self.1.raze(vc, &mut state.1);
        self.2.raze(vc, &mut state.2);
        self.3.raze(vc, &mut state.3);
    }
}
