use bevy::prelude::*;
use bevy_quill::prelude::*;

pub struct EnterExitPlugin;

impl Plugin for EnterExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enter_exit_state_machine);
    }
}

/// Tracks an enter / exit transition. This is useful for widgets like dialog boxes and popup
/// menus which have an opening and closing animation.
#[derive(Default, Clone, PartialEq)]
pub enum EnterExitState {
    /// One-frame delay at start of entering.
    EnterStart,

    /// Opening animation.
    Entering,

    /// Fully open
    Entered,

    /// One frame delay at start of exiting
    ExitStart,

    // Closing animation
    Exiting,

    /// Fully closed
    #[default]
    Exited,
}

impl EnterExitState {
    /// Convert an Enter/Exit state into a class name.
    pub fn as_class_name(&self) -> &str {
        match self {
            EnterExitState::EnterStart => "enter-start",
            EnterExitState::Entering => "entering",
            EnterExitState::Entered => "entered",
            EnterExitState::ExitStart => "exit-start",
            EnterExitState::Exiting => "exiting",
            EnterExitState::Exited => "exited",
        }
    }
}

#[derive(Component, Default)]
pub struct EnterExit {
    pub open: bool,
    pub delay: f32,
    pub state: EnterExitState,
}

#[derive(Component, Default)]
pub struct EnterExitTimer {
    pub timer: f32,
}

/// Trait which adds `use_enter_exit` to [`Cx`].
pub trait EnterExitApi {
    fn use_enter_exit(&mut self, open: bool, delay: f32) -> EnterExitState;
}

impl<'w, 'p, Props> EnterExitApi for Cx<'w, 'p, Props> {
    fn use_enter_exit(&mut self, open: bool, delay: f32) -> EnterExitState {
        self.use_effect(
            |mut ve| {
                match ve.get_mut::<EnterExit>() {
                    Some(mut ee) => {
                        ee.open = open;
                    }
                    None => {
                        ve.insert((
                            EnterExit {
                                open,
                                delay,
                                ..default()
                            },
                            EnterExitTimer { ..default() },
                        ));
                    }
                };
            },
            open,
        );

        self.use_view_component::<EnterExit>()
            .unwrap()
            .state
            .clone()
    }
}

pub fn enter_exit_state_machine(
    mut query: Query<(&mut EnterExit, &mut EnterExitTimer)>,
    time: Res<Time>,
) {
    for (mut ee, mut tt) in query.iter_mut() {
        match ee.state {
            EnterExitState::EnterStart => {
                if ee.open {
                    ee.state = EnterExitState::Entering;
                    tt.timer = 0.;
                } else {
                    ee.state = EnterExitState::ExitStart;
                }
            }
            EnterExitState::Entering => {
                if ee.open {
                    tt.timer += time.delta_seconds();
                    if tt.timer > ee.delay {
                        ee.state = EnterExitState::Entered;
                    }
                } else {
                    ee.state = EnterExitState::ExitStart;
                }
            }
            EnterExitState::Entered => {
                if !ee.open {
                    ee.state = EnterExitState::ExitStart;
                }
            }
            EnterExitState::ExitStart => {
                if !ee.open {
                    ee.state = EnterExitState::Exiting;
                    tt.timer = 0.;
                } else {
                    ee.state = EnterExitState::EnterStart;
                }
            }
            EnterExitState::Exiting => {
                if ee.open {
                    ee.state = EnterExitState::EnterStart;
                } else {
                    tt.timer += time.delta_seconds();
                    if tt.timer > 0.3 {
                        ee.state = EnterExitState::Exited;
                    }
                }
            }
            EnterExitState::Exited => {
                if ee.open {
                    ee.state = EnterExitState::EnterStart;
                }
            }
        }
    }
}
