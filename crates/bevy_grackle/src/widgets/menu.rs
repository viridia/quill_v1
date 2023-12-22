use bevy::{prelude::*, ui};
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::Size;

#[dynamic]
static STYLE_MENU_BUTTON: StyleHandle = StyleHandle::build(|ss| {
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

#[derive(Clone, PartialEq, Default)]
pub struct MenuButtonProps<V: View + Clone, S: StyleTuple = ()> {
    pub children: V,
    pub size: Size,
    pub style: S,
    pub disabled: bool,
}

pub fn menu_button<V: View + Clone + PartialEq + 'static, ST: StyleTuple + PartialEq + 'static>(
    cx: Cx<MenuButtonProps<V, ST>>,
) -> impl View {
    bevy_egret::widgets::menu_button.bind(bevy_egret::widgets::MenuButtonProps {
        children: cx.props.children.clone(),
        style: (STYLE_MENU_BUTTON.clone(), cx.props.style.clone()),
        class_names: ("primary", cx.props.size.class_name()),
        marker: std::marker::PhantomData,
        disabled: cx.props.disabled,
    })
}
