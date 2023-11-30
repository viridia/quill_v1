use crate::node_span::NodeSpan;
use crate::{View, ViewContext};
use impl_trait_for_tuples::*;

// ViewTuple

#[doc(hidden)]
pub trait ViewTuple: Send {
    /// Aggregate View::State for all tuple members.
    type State: Send;

    /// Return the number of child views.
    fn len(&self) -> usize;

    /// Return the output nodes for all spans.
    fn span_nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan;

    /// Build the child views.
    fn build_spans(&self, vc: &mut ViewContext) -> Self::State;

    /// Update the child views.
    fn update_spans(&self, vc: &mut ViewContext, state: &mut Self::State);

    /// Assemble the child views.
    fn assemble_spans(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan;

    /// Despawn the child views.
    fn raze_spans(&self, vc: &mut ViewContext, state: &mut Self::State);
}

impl<A: View> ViewTuple for A {
    type State = A::State;

    fn len(&self) -> usize {
        1
    }

    fn span_nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        self.nodes(vc, state)
    }

    fn build_spans(&self, vc: &mut ViewContext) -> Self::State {
        self.build(vc)
    }

    fn update_spans(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.update(vc, state)
    }

    fn assemble_spans(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.assemble(vc, state)
    }

    fn raze_spans(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.raze(vc, state)
    }
}

#[impl_for_tuples(1, 16)]
#[tuple_types_custom_trait_bound(View)]
impl ViewTuple for Tuple {
    for_tuples!( type State = ( #( Tuple::State ),* ); );

    fn len(&self) -> usize {
        for_tuples!((#( 1 )+*))
    }

    #[rustfmt::skip]
    fn span_nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            for_tuples!(#( self.Tuple.nodes(vc, &state.Tuple) ),*)
        ]))
    }

    fn build_spans(&self, vc: &mut ViewContext) -> Self::State {
        for_tuples!((#( self.Tuple.build(vc) ),*))
    }

    fn update_spans(&self, vc: &mut ViewContext, state: &mut Self::State) {
        for_tuples!(#( self.Tuple.update(vc, &mut state.Tuple); )*)
    }

    #[rustfmt::skip]
    fn assemble_spans(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        NodeSpan::Fragment(Box::new([
            for_tuples!(#( self.Tuple.assemble(vc, &mut state.Tuple) ),*)
        ]))
    }

    fn raze_spans(&self, vc: &mut ViewContext, state: &mut Self::State) {
        for_tuples!(#( self.Tuple.raze(vc, &mut state.Tuple); )*)
    }
}
