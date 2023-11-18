use crate::ElementContext;
use crate::View;

use crate::node_span::NodeSpan;

// If

#[derive(Default)]
pub enum IfState<Pos, Neg> {
    #[default]
    Indeterminate,
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

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        if self.test {
            match state {
                Self::State::True(ref mut true_state) => {
                    // Mutate state in place
                    self.pos.build(ecx, true_state, prev)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(ecx, state, prev);
                    let mut true_state: Pos::State = Default::default();
                    let nodes = self.pos.build(ecx, &mut true_state, &NodeSpan::Empty);
                    *state = Self::State::True(true_state);
                    nodes
                }
            }
        } else {
            match state {
                Self::State::False(ref mut false_state) => {
                    // Mutate state in place
                    self.neg.build(ecx, false_state, prev)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(ecx, state, prev);
                    let mut false_state: Neg::State = Default::default();
                    let nodes = self.neg.build(ecx, &mut false_state, &NodeSpan::Empty);
                    *state = Self::State::False(false_state);
                    nodes
                }
            }
        }
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, prev: &NodeSpan) {
        match state {
            Self::State::True(ref mut true_state) => self.pos.raze(ecx, true_state, prev),
            Self::State::False(ref mut false_state) => self.neg.raze(ecx, false_state, prev),
            _ => (),
        }
    }
}
