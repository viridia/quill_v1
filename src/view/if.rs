use bevy::ecs::world::World;

use crate::BuildContext;
use crate::View;

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

    fn nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        match state {
            Self::State::True(ref true_state) => self.pos.nodes(bc, true_state),
            Self::State::False(ref false_state) => self.neg.nodes(bc, false_state),
        }
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        if self.test {
            IfState::True(self.pos.build(bc))
        } else {
            IfState::False(self.neg.build(bc))
        }
    }

    fn update(&self, bc: &mut BuildContext, state: &mut Self::State) {
        if self.test {
            match state {
                Self::State::True(ref mut true_state) => {
                    // Mutate state in place
                    self.pos.update(bc, true_state)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(bc.world, state);
                    bc.mark_changed_shape();
                    *state = Self::State::True(self.pos.build(bc));
                }
            }
        } else {
            match state {
                Self::State::False(ref mut false_state) => {
                    // Mutate state in place
                    self.neg.update(bc, false_state)
                }

                _ => {
                    // Despawn old state and construct new state
                    self.raze(bc.world, state);
                    bc.mark_changed_shape();
                    *state = Self::State::False(self.neg.build(bc));
                }
            }
        }
    }

    fn assemble(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        match state {
            Self::State::True(ref mut true_state) => self.pos.assemble(bc, true_state),
            Self::State::False(ref mut false_state) => self.neg.assemble(bc, false_state),
        }
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        match state {
            Self::State::True(ref mut true_state) => self.pos.raze(world, true_state),
            Self::State::False(ref mut false_state) => self.neg.raze(world, false_state),
        }
    }
}
