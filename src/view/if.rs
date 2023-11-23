use crate::ElementContext;
use crate::View;

use crate::node_span::NodeSpan;

// If

pub enum IfState<Pos, Neg> {
    True(Pos),
    False(Neg),
}

pub struct If<Pos: View, Neg: View> {
    test: bool,
    pos: Pos,
    neg: Neg,
}

impl<Pos: View, Neg: View> If<Pos, Neg> {
    pub fn new(test: bool, pos: Pos, neg: Neg) -> Self {
        Self { test, pos, neg }
    }
}

impl<Pos: View, Neg: View> View for If<Pos, Neg> {
    /// Union of true and false states.
    type State = IfState<Pos::State, Neg::State>;

    fn build(&self, ecx: &mut ElementContext) -> (Self::State, NodeSpan) {
        if self.test {
            let (state, nodes) = self.pos.build(ecx);
            (IfState::True(state), nodes)
        } else {
            let (state, nodes) = self.neg.build(ecx);
            (IfState::False(state), nodes)
        }
    }

    fn rebuild(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        if self.test {
            match state {
                Self::State::True(ref mut true_state) => {
                    // Mutate state in place
                    self.pos.rebuild(ecx, true_state, prev)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(ecx, state, prev);
                    let (st, nodes) = self.pos.build(ecx);
                    *state = Self::State::True(st);
                    nodes
                }
            }
        } else {
            match state {
                Self::State::False(ref mut false_state) => {
                    // Mutate state in place
                    self.neg.rebuild(ecx, false_state, prev)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(ecx, state, prev);
                    let (st, nodes) = self.neg.build(ecx);
                    *state = Self::State::False(st);
                    nodes
                }
            }
        }
    }

    fn collect(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        nodes: &NodeSpan,
    ) -> NodeSpan {
        match state {
            Self::State::True(ref mut true_state) => self.pos.collect(ecx, true_state, nodes),
            Self::State::False(ref mut false_state) => self.neg.collect(ecx, false_state, nodes),
        }
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, prev: &NodeSpan) {
        match state {
            Self::State::True(ref mut true_state) => self.pos.raze(ecx, true_state, prev),
            Self::State::False(ref mut false_state) => self.neg.raze(ecx, false_state, prev),
        }
    }
}
