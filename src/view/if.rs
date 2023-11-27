use crate::View;
use crate::ViewContext;

use crate::node_span::NodeSpan;

// If

pub enum IfState<Pos, Neg> {
    True(Pos),
    False(Neg),
}

/// A conditional view which renders one of two children depending on the condition expression.
pub struct If<Pos: View, Neg: View> {
    test: bool,
    pos: Pos,
    neg: Neg,
}

impl<Pos: View, Neg: View> If<Pos, Neg> {
    /// Construct a new If View.
    pub fn new(test: bool, pos: Pos, neg: Neg) -> Self {
        Self { test, pos, neg }
    }
}

impl<Pos: View, Neg: View> View for If<Pos, Neg> {
    /// Union of true and false states.
    type State = IfState<Pos::State, Neg::State>;

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        match state {
            Self::State::True(ref true_state) => self.pos.nodes(vc, true_state),
            Self::State::False(ref false_state) => self.neg.nodes(vc, false_state),
        }
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        if self.test {
            IfState::True(self.pos.build(vc))
        } else {
            IfState::False(self.neg.build(vc))
        }
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        if self.test {
            match state {
                Self::State::True(ref mut true_state) => {
                    // Mutate state in place
                    self.pos.update(vc, true_state)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(vc, state);
                    vc.mark_changed_shape();
                    *state = Self::State::True(self.pos.build(vc));
                }
            }
        } else {
            match state {
                Self::State::False(ref mut false_state) => {
                    // Mutate state in place
                    self.neg.update(vc, false_state)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(vc, state);
                    vc.mark_changed_shape();
                    *state = Self::State::False(self.neg.build(vc));
                }
            }
        }
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        match state {
            Self::State::True(ref mut true_state) => self.pos.assemble(vc, true_state),
            Self::State::False(ref mut false_state) => self.neg.assemble(vc, false_state),
        }
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        match state {
            Self::State::True(ref mut true_state) => self.pos.raze(vc, true_state),
            Self::State::False(ref mut false_state) => self.neg.raze(vc, false_state),
        }
    }
}
