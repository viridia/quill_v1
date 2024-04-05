use bevy::{prelude::*, ui};
use bevy_grackle::{
    events::Clicked,
    hooks::{EnterExitApi, EnterExitState},
    widgets::{button, ButtonProps},
};
use bevy_mod_picking::prelude::{EntityEvent, Listener, On};
use bevy_quill::prelude::*;
use static_init::dynamic;

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
        .transition(&[Transition {
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
        .justify_content(ui::JustifyContent::SpaceBetween)
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

#[derive(PartialEq, Clone)]
pub struct DemoDialogProps {
    pub open: bool,
    pub target: Entity,
}

#[derive(Clone, Event, EntityEvent)]
#[can_bubble]
pub struct RequestClose {
    #[target]
    pub target: Entity,
    pub id: &'static str,
}

pub fn dialog(mut cx: Cx<DemoDialogProps>) -> impl View {
    let open = cx.props.open;
    let target = cx.props.target;
    let state = cx.use_enter_exit(open, 0.3);
    If::new(
        state != EnterExitState::Exited,
        Portal::new().children(
            Element::new()
                .styled(STYLE_DIALOG_OVERLAY.clone())
                .class_names(state.as_class_name())
                .children(
                    Element::new().styled(STYLE_DIALOG.clone()).children((
                        Element::new()
                            .styled(STYLE_DIALOG_HEADER.clone())
                            .children(("A Modal Dialog", "[x]")),
                        Element::new().styled(STYLE_DIALOG_BODY.clone()).children(
                            Element::new().styled(STYLE_LIST.clone()).children((
                                "Alpha Male",
                                "Beta Tester",
                                "Gamma Ray",
                                "Delta Sleep",
                                "Epsilon Eridani",
                                "Zeta Function",
                                "Eta Oin Shrdlu",
                            )),
                        ),
                        Element::new()
                            .styled(STYLE_DIALOG_FOOTER.clone())
                            .insert(On::<Clicked>::run(move |_ev: Listener<Clicked>,
                                mut writer: EventWriter<RequestClose>| {
                                    writer.send(RequestClose {
                                        target,
                                        id: "demo_dialog",
                                    });
                            }))
                            .children((
                                button.bind(ButtonProps::new("cancel").children("Cancel")),
                                button.bind(ButtonProps::new("ok").children("Ok")),
                            )),
                    )),
                ),
        ),
        (),
    )
}
