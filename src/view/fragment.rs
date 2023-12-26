use bevy::ecs::world::World;

use crate::{BuildContext, View, ViewTuple};

use crate::node_span::NodeSpan;

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
