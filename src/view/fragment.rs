use bevy::ecs::world::World;

use crate::{BuildContext, View, ViewTuple};

use crate::node_span::NodeSpan;

use super::view_tuple::ViewTupleClone;

/// A View which renders a sequence of nodes which are inserted into the parent view.
pub struct Fragment<A: ViewTuple> {
    items: A,
}

impl<A: ViewTuple> Fragment<A> {
    /// Construct a new [`Fragment`] from a tuple of views.
    pub fn new(items: A) -> Self {
        Self { items }
    }
}

impl<A: ViewTuple> View for Fragment<A> {
    type State = A::State;

    fn nodes(&self, vc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.items.span_nodes(vc, state)
    }

    fn build(&self, vc: &mut BuildContext) -> Self::State {
        self.items.build_spans(vc)
    }

    fn update(&self, vc: &mut BuildContext, state: &mut Self::State) {
        self.items.update_spans(vc, state);
    }

    fn assemble(&self, vc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.items.assemble_spans(vc, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.items.raze_spans(world, state);
    }
}

/// A View which renders a sequence of nodes which are inserted into the parent view.
pub struct FragmentClone<A: ViewTupleClone> {
    items: A,
}

impl<A: ViewTupleClone> FragmentClone<A> {
    /// Construct a new [`Fragment`] from a tuple of views.
    pub fn new(items: A) -> Self {
        Self { items }
    }
}

impl<A: ViewTupleClone> View for FragmentClone<A> {
    type State = A::State;

    fn nodes(&self, vc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.items.span_nodes(vc, state)
    }

    fn build(&self, vc: &mut BuildContext) -> Self::State {
        self.items.build_spans(vc)
    }

    fn update(&self, vc: &mut BuildContext, state: &mut Self::State) {
        self.items.update_spans(vc, state);
    }

    fn assemble(&self, vc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.items.assemble_spans(vc, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.items.raze_spans(world, state);
    }
}

impl<A: ViewTupleClone> Clone for FragmentClone<A> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
        }
    }
}

impl<A: ViewTupleClone> PartialEq for FragmentClone<A> {
    fn eq(&self, other: &Self) -> bool {
        self.items.eq(&other.items)
    }
}
