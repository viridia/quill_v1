use bevy::{prelude::*, ui};
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::Size;

#[dynamic]
static STYLE_BUTTON: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#282828")
        .border_color("#383838")
        .border(1)
        .display(ui::Display::Flex)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .min_height(32)
        .padding_left(12)
        .padding_right(12)
        .selector(".pressed", |ss| ss.background_color("#404040"))
        .selector(":hover", |ss| {
            ss.border_color("#444").background_color("#2F2F2F")
        })
        .selector(":hover.pressed", |ss| ss.background_color("#484848"))
});

#[dynamic]
static STYLE_BUTTON_PRIMARY: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#282828")
        .border_color("#383838")
        .selector(".pressed", |ss| ss.background_color("#404040"))
        .selector(":hover", |ss| {
            ss.border_color("#444").background_color("#2F2F2F")
        })
        .selector(":hover.pressed", |ss| ss.background_color("#484848"))
});

/// The variant determines the button's color scheme
#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    /// The default apperance.
    #[default]
    Default,

    /// A more prominent, "call to action", appearance.
    Primary,

    /// An appearance indicating a potentially dangerous action.
    Danger,
}

#[derive(Clone, PartialEq, Default)]
pub struct ButtonProps<V: View + Clone, S: StyleTuple> {
    pub id: &'static str,
    pub children: V,
    pub variant: ButtonVariant,
    pub size: Size,
    pub style: S,
}

pub fn button<V: View + Clone + PartialEq + 'static, ST: StyleTuple + PartialEq + 'static>(
    cx: Cx<ButtonProps<V, ST>>,
) -> impl View {
    bevy_egret::button.bind(bevy_egret::ButtonProps {
        id: cx.props.id,
        children: cx.props.children.clone(),
        style: (
            STYLE_BUTTON.clone(),
            STYLE_BUTTON_PRIMARY.clone(),
            cx.props.style.clone(),
        ),
        class_names: ("primary", cx.props.size.class_name()),
        marker: std::marker::PhantomData,
        // ..default()
    })
}
