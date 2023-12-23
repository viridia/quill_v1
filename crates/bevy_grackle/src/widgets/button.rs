use bevy::{prelude::*, ui};
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::{tokens::*, Size};

#[dynamic]
static STYLE_BUTTON: StyleHandle = StyleHandle::build(|ss| {
    ss.border(1)
        .display(ui::Display::Flex)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .min_height(32)
        .padding_left(12)
        .padding_right(12)
    // .font(Some(AssetPath::from(
    //     "grackle://fonts/Ubuntu/Ubuntu-Medium.ttf",
    // )))
});

#[dynamic]
static STYLE_BUTTON_DEFAULT: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(BUTTON_DEFAULT_BG)
        .border_color(BUTTON_DEFAULT_BORDER)
        .selector(".pressed", |ss| {
            ss.background_color(BUTTON_DEFAULT_PRESSED_BG)
        })
        .selector(":hover", |ss| ss.background_color(BUTTON_DEFAULT_HOVER_BG))
        .selector(":hover.pressed", |ss| {
            ss.background_color(BUTTON_DEFAULT_PRESSED_BG)
        })
});

#[dynamic]
static STYLE_BUTTON_PRIMARY: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(BUTTON_PRIMARY_BG)
        .border_color(BUTTON_PRIMARY_BORDER)
        .selector(".pressed", |ss| {
            ss.background_color(BUTTON_PRIMARY_PRESSED_BG)
        })
        .selector(":hover", |ss| ss.background_color(BUTTON_PRIMARY_HOVER_BG))
        .selector(":hover.pressed", |ss| {
            ss.background_color(BUTTON_PRIMARY_PRESSED_BG)
        })
});

#[dynamic]
static STYLE_BUTTON_DANGER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(BUTTON_DANGER_BG)
        .border_color(BUTTON_DANGER_BORDER)
        .selector(".pressed", |ss| {
            ss.background_color(BUTTON_DANGER_PRESSED_BG)
        })
        .selector(":hover", |ss| ss.background_color(BUTTON_DANGER_HOVER_BG))
        .selector(":hover.pressed", |ss| {
            ss.background_color(BUTTON_DANGER_PRESSED_BG)
        })
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
pub struct ButtonProps<V: View + Clone, S: StyleTuple = ()> {
    pub id: &'static str,
    pub children: V,
    pub variant: ButtonVariant,
    pub size: Size,
    pub style: S,
    pub disabled: bool,
}

pub fn button<V: View + Clone + PartialEq + 'static, ST: StyleTuple + PartialEq + 'static>(
    cx: Cx<ButtonProps<V, ST>>,
) -> impl View {
    bevy_egret::widgets::button.bind(bevy_egret::widgets::ButtonProps {
        id: cx.props.id,
        children: cx.props.children.clone(),
        style: (
            STYLE_BUTTON.clone(),
            match cx.props.variant {
                ButtonVariant::Default => STYLE_BUTTON_DEFAULT.clone(),
                ButtonVariant::Primary => STYLE_BUTTON_PRIMARY.clone(),
                ButtonVariant::Danger => STYLE_BUTTON_DANGER.clone(),
            },
            cx.props.style.clone(),
        ),
        class_names: ("primary", cx.props.size.class_name()),
        marker: std::marker::PhantomData,
        disabled: cx.props.disabled,
    })
}
