use bevy::{prelude::*, ui};
use bevy_egret::{
    floating::{FloatAlign, FloatPosition, FloatSide, Floating},
    widgets::{menu_popup, MenuPopupProps},
};
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::{
    tokens::{BUTTON_DEFAULT, MENU_ITEM, MENU_POPUP, TYPOGRAPHY},
    Size,
};

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
    ss.position(PositionType::Absolute)
        .border(1)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Stretch)
        .padding((0, 2))
        .z_index(101)
        .scale(0.5)
        .transition(&vec![Transition {
            property: TransitionProperty::Transform,
            duration: 0.3,
            timing: timing::EASE_IN_OUT,
            ..default()
        }])
        .selector(".entering > &,.entered > &", |ss| ss.scale(1.))
        .selector(".enter-start > &", |ss| ss.display(ui::Display::None))
});

#[dynamic]
static STYLE_MENU_ITEM: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Row)
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Start)
        .padding((8, 6))
        .margin((2, 0))
        .selector(".indent > &", |ss| ss.padding_left(24))
});

#[dynamic]
static STYLE_MENU_DIVIDER: StyleHandle =
    StyleHandle::build(|ss| ss.background_color("#000").height(1).margin((0, 2)));

#[derive(Clone, PartialEq, Default)]
pub struct MenuButtonProps<V: View + Clone, VI: View + Clone, S: StyleTuple = ()> {
    pub children: V,
    pub items: VI,
    pub size: Size,
    pub style: S,
    pub disabled: bool,
    pub indent: bool,
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

#[derive(PartialEq, Default)]
pub struct MenuItemProps<V: View + Clone> {
    pub id: &'static str,
    pub label: V,
    pub checked: bool,
    pub disabled: bool,
    // icon
}

impl<V: View + Clone, VI: View + Clone, S: StyleTuple> MenuButtonProps<V, VI, S> {
    pub fn children<V2: View + Clone>(self, children: V2) -> MenuButtonProps<V2, VI, S> {
        MenuButtonProps {
            children,
            items: self.items,
            size: self.size,
            style: self.style,
            disabled: self.disabled,
            indent: self.indent,
        }
    }

    pub fn items<V2: View + Clone>(self, items: V2) -> MenuButtonProps<V, V2, S> {
        MenuButtonProps {
            children: self.children,
            items,
            size: self.size,
            style: self.style,
            disabled: self.disabled,
            indent: self.indent,
        }
    }

    pub fn style<S2: StyleTuple>(self, style: S2) -> MenuButtonProps<V, VI, S2> {
        MenuButtonProps {
            children: self.children,
            items: self.items,
            size: self.size,
            style,
            disabled: self.disabled,
            indent: self.indent,
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

    pub fn indent(mut self, indent: bool) -> Self {
        self.indent = indent;
        self
    }
}

pub fn menu_button<
    V: View + Clone + PartialEq + 'static,
    VI: View + Clone + PartialEq + 'static,
    ST: StyleTuple + PartialEq + 'static,
>(
    mut cx: Cx<MenuButtonProps<V, VI, ST>>,
) -> impl View {
    let id_anchor = cx.create_entity();
    bevy_egret::widgets::menu_button.bind(bevy_egret::widgets::MenuButtonProps {
        anchor: id_anchor,
        children: cx.props.children.clone(),
        popup: ViewParam::new(
            menu_popup
                .bind(MenuPopupProps {
                    children: cx.props.items.clone(),
                    class_names: "indent".if_true(cx.props.indent),
                    style: (
                        STYLE_MENU_POPUP.clone(),
                        cx.get_scoped_value(TYPOGRAPHY),
                        cx.get_scoped_value(MENU_POPUP),
                        cx.props.style.clone(),
                    ),
                    marker: std::marker::PhantomData,
                })
                .insert(Floating {
                    anchor: id_anchor,
                    position: vec![
                        FloatPosition {
                            side: FloatSide::Right,
                            align: FloatAlign::Start,
                            stretch: false,
                            gap: 2.,
                        },
                        FloatPosition {
                            side: FloatSide::Bottom,
                            align: FloatAlign::Start,
                            stretch: false,
                            gap: 2.,
                        },
                    ],
                }),
        ),
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

pub fn menu_item<'a, V: View + Clone + PartialEq + 'static>(cx: Cx<MenuItemProps<V>>) -> impl View {
    bevy_egret::widgets::menu_item.bind(bevy_egret::widgets::MenuItemProps {
        label: cx.props.label.clone(),
        id: cx.props.id,
        style: (STYLE_MENU_ITEM.clone(), cx.get_scoped_value(MENU_ITEM)),
        checked: cx.props.checked,
        disabled: cx.props.disabled,
    })
}

pub fn menu_divider(_cx: Cx) -> impl View {
    Element::new()
        .named("menu-divider")
        .styled(STYLE_MENU_DIVIDER.clone())
}
