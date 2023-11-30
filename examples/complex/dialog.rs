use bevy::{prelude::*, ui};
use bevy_mod_picking::prelude::{ListenerInput, On};
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::button::{button, ButtonProps, Clicked};

// Dialog background overlay
#[dynamic]
static STYLE_DIALOG_OVERLAY: StyleHandle = StyleHandle::build(|ss| {
    ss.position(PositionType::Absolute)
        .display(ui::Display::Flex)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Center)
        .left(0)
        .top(0)
        .right(0)
        .bottom(0)
        .z_index(100)
        .background_color("#222c")
});

#[dynamic]
static STYLE_DIALOG: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#333")
        .position(PositionType::Relative)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .justify_content(ui::JustifyContent::Center)
        .align_items(ui::AlignItems::Stretch)
        .border_color(Color::BLACK)
        .width(200)
        .border(2)
        .scale(0.5)
        .transition(&vec![Transition {
            property: TransitionProperty::Transform,
            duration: 0.3,
            timing: timing::EASE_IN_OUT,
            ..default()
        }])
        .selector(".entering > &,.entered > &", |ss| ss.scale(1.))
});

#[dynamic]
static STYLE_DIALOG_HEADER: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::FlexStart)
        .border_color("#0008")
        .border_bottom(1)
        .padding((12, 6))
});

#[dynamic]
static STYLE_DIALOG_BODY: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::Stretch)
        .justify_content(ui::JustifyContent::FlexStart)
        .padding((12, 6))
        .min_height(200)
});

#[dynamic]
static STYLE_DIALOG_FOOTER: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .justify_content(ui::JustifyContent::FlexEnd)
        .align_items(ui::AlignItems::Center)
        .border_color("#0008")
        .border_top(1)
        .column_gap(4)
        .padding((8, 6))
});

#[dynamic]
static STYLE_LIST: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .background_color("#222")
        .flex_direction(ui::FlexDirection::Column)
        .align_items(ui::AlignItems::FlexStart)
        .justify_content(ui::JustifyContent::FlexStart)
        .flex_grow(1.)
        .padding(6)
});

/// Tracks an enter / exit transition.
#[derive(Default)]
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

#[derive(Resource, Default)]
pub struct DemoDialogOpen {
    pub open: bool,
    pub state: EnterExitState,
}

#[derive(Resource, Default)]
pub struct DemoDialogTransitionTimer {
    pub timer: f32,
}

impl DemoDialogOpen {
    pub fn is_shown(&self) -> bool {
        // Shown when any state except 'exited' so we can do opening/closing animation.
        match self.state {
            EnterExitState::Exited => false,
            _ => true,
        }
    }

    pub fn class_name(&self) -> &str {
        match self.state {
            EnterExitState::EnterStart => "enter-start",
            EnterExitState::Entering => "entering",
            EnterExitState::Entered => "entered",
            EnterExitState::ExitStart => "exit-start",
            EnterExitState::Exiting => "exiting",
            EnterExitState::Exited => "exited",
        }
    }
}

// Color swatch
pub fn dialog(mut cx: Cx) -> impl View {
    let dialog_state = cx.use_resource::<DemoDialogOpen>();
    If::new(
        dialog_state.is_shown(),
        Portal::new().children(
            Element::new()
                .styled(STYLE_DIALOG_OVERLAY.clone())
                .class_names(dialog_state.class_name())
                .children(
                    Element::new().styled(STYLE_DIALOG.clone()).children((
                        Element::new()
                            .styled(STYLE_DIALOG_HEADER.clone())
                            .children("A Modal Dialog"),
                        Element::new().styled(STYLE_DIALOG_BODY.clone()).children(
                            Element::new().styled(STYLE_LIST.clone()).children((
                                "Alpha Male",
                                "Beta Tester",
                                "Gamma Ray",
                            )),
                        ),
                        Element::new()
                            .styled(STYLE_DIALOG_FOOTER.clone())
                            .once(|entity, world| {
                                let mut e = world.entity_mut(entity);
                                e.insert(On::<Clicked>::run(
                                    |_ev: Res<ListenerInput<Clicked>>,
                                     mut dlog: ResMut<DemoDialogOpen>| {
                                        dlog.open = false
                                    },
                                ));
                            })
                            .children((
                                button.bind(ButtonProps {
                                    id: "cancel",
                                    children: "Cancel",
                                }),
                                button.bind(ButtonProps {
                                    id: "ok",
                                    children: "OK",
                                }),
                            )),
                    )),
                ),
        ),
        (),
    )
}

pub fn enter_exit_state_machine(
    mut ee: ResMut<DemoDialogOpen>,
    mut tt: ResMut<DemoDialogTransitionTimer>,
    time: Res<Time>,
) {
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
                if tt.timer > 0.3 {
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
