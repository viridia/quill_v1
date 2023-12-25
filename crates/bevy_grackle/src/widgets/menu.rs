use bevy::{prelude::*, ui};
use bevy_egret::widgets::{menu_popup, MenuPopupProps};
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::{tokens::BUTTON_DEFAULT, Size};

#[dynamic]
static STYLE_MENU_BUTTON: StyleHandle = StyleHandle::build(|ss| {
    ss.border(1)
        .display(ui::Display::Flex)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .padding_left(12)
        .padding_right(12)
        .selector(".size-xxxs", |ss| ss.min_height(Size::Xxxs.height()))
        .selector(".size-xxs", |ss| ss.min_height(Size::Xxs.height()))
        .selector(".size-xs", |ss| ss.min_height(Size::Xs.height()))
        .selector(".size-sm", |ss| ss.min_height(Size::Sm.height()))
        .selector(".size-md", |ss| ss.min_height(Size::Md.height()))
        .selector(".size-lg", |ss| ss.min_height(Size::Lg.height()))
        .selector(".size-xl", |ss| ss.min_height(Size::Xl.height()))
});

#[dynamic]
static STYLE_MENU_POPUP: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#f00")
        .border_color("#00f")
        .border(1)
        .display(ui::Display::Flex)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
});

#[derive(Clone, PartialEq, Default)]
pub struct MenuButtonProps<V: View + Clone, VI: View + Clone, S: StyleTuple = ()> {
    pub children: V,
    pub items: VI,
    pub size: Size,
    pub style: S,
    pub disabled: bool,
}

impl MenuButtonProps<(), (), ()> {
    pub fn new() -> Self {
        Self {
            children: (),
            items: (),
            style: (),
            ..Default::default()
        }
    }
}

impl<V: View + Clone, VI: View + Clone, S: StyleTuple> MenuButtonProps<V, VI, S> {
    pub fn children<V2: View + Clone>(self, children: V2) -> MenuButtonProps<V2, VI, S> {
        MenuButtonProps {
            children,
            items: self.items,
            size: self.size,
            style: self.style,
            disabled: self.disabled,
        }
    }

    pub fn items<V2: View + Clone>(self, items: V2) -> MenuButtonProps<V, V2, S> {
        MenuButtonProps {
            children: self.children,
            items,
            size: self.size,
            style: self.style,
            disabled: self.disabled,
        }
    }

    pub fn style<S2: StyleTuple>(self, style: S2) -> MenuButtonProps<V, VI, S2> {
        MenuButtonProps {
            children: self.children,
            items: self.items,
            size: self.size,
            style,
            disabled: self.disabled,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn menu_button<
    V: View + Clone + PartialEq + 'static,
    VI: View + Clone + PartialEq + 'static,
    ST: StyleTuple + PartialEq + 'static,
>(
    cx: Cx<MenuButtonProps<V, VI, ST>>,
) -> impl View {
    bevy_egret::widgets::menu_button.bind(bevy_egret::widgets::MenuButtonProps {
        children: cx.props.children.clone(),
        popup: ViewParam::new(Portal::new().children(menu_popup.bind(MenuPopupProps {
            children: cx.props.items.clone(),
            style: (STYLE_MENU_POPUP.clone(), cx.props.style.clone()),
        }))),
        style: (
            STYLE_MENU_BUTTON.clone(),
            cx.get_scoped_value(BUTTON_DEFAULT),
            cx.props.style.clone(),
        ),
        class_names: cx.props.size.class_name(),
        marker: std::marker::PhantomData,
        disabled: cx.props.disabled,
    })
}
