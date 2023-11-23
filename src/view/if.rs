use crate::View;
use crate::ViewContext;

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

    fn nodes(&self, ecx: &ViewContext, state: &Self::State) -> NodeSpan {
        match state {
            Self::State::True(ref true_state) => self.pos.nodes(ecx, true_state),
            Self::State::False(ref false_state) => self.neg.nodes(ecx, false_state),
        }
    }

    fn build(&self, ecx: &mut ViewContext) -> Self::State {
        if self.test {
            IfState::True(self.pos.build(ecx))
        } else {
            IfState::False(self.neg.build(ecx))
        }
    }

    fn update(&self, ecx: &mut ViewContext, state: &mut Self::State) {
        if self.test {
            match state {
                Self::State::True(ref mut true_state) => {
                    // Mutate state in place
                    self.pos.update(ecx, true_state)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(ecx, state);
                    *state = Self::State::True(self.pos.build(ecx));
                }
            }
        } else {
            match state {
                Self::State::False(ref mut false_state) => {
                    // Mutate state in place
                    self.neg.update(ecx, false_state)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(ecx, state);
                    *state = Self::State::False(self.neg.build(ecx));
                }
            }
        }
    }

    fn assemble(&self, ecx: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        match state {
            Self::State::True(ref mut true_state) => self.pos.assemble(ecx, true_state),
            Self::State::False(ref mut false_state) => self.neg.assemble(ecx, false_state),
        }
    }

    fn raze(&self, ecx: &mut ViewContext, state: &mut Self::State) {
        match state {
            Self::State::True(ref mut true_state) => self.pos.raze(ecx, true_state),
            Self::State::False(ref mut false_state) => self.neg.raze(ecx, false_state),
        }
    }
}
