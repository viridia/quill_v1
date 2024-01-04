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

#[derive(PartialEq, Default)]
pub struct ButtonProps<V: View + Clone, S: StyleTuple = ()> {
    pub id: &'static str,
    pub children: V,
    pub variant: ButtonVariant,
    pub size: Size,
    pub style: S,
    pub disabled: bool,
}

impl ButtonProps<(), ()> {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            children: (),
            style: (),
            ..Default::default()
        }
    }
}

impl<V: View + Clone, S: StyleTuple> ButtonProps<V, S> {
    pub fn children<V2: View + Clone>(self, children: V2) -> ButtonProps<V2, S> {
        ButtonProps {
            children,
            id: self.id,
            variant: self.variant,
            size: self.size,
            style: self.style,
            disabled: self.disabled,
        }
    }

    pub fn style<S2: StyleTuple>(self, style: S2) -> ButtonProps<V, S2> {
        ButtonProps {
            children: self.children,
            id: self.id,
            variant: self.variant,
            size: self.size,
            style,
            disabled: self.disabled,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
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

pub fn button<V: View + Clone + PartialEq + 'static, ST: StyleTuple + PartialEq + 'static>(
    cx: Cx<ButtonProps<V, ST>>,
) -> impl View {
    bevy_egret::widgets::button.bind(bevy_egret::widgets::ButtonProps {
        id: cx.props.id,
        children: cx.props.children.clone(),
        style: (
            STYLE_BUTTON.clone(),
            match cx.props.variant {
                ButtonVariant::Default => cx.get_scoped_value(BUTTON_DEFAULT),
                ButtonVariant::Primary => cx.get_scoped_value(BUTTON_PRIMARY),
                ButtonVariant::Danger => cx.get_scoped_value(BUTTON_DANGER),
            },
            cx.props.style.clone(),
        ),
        class_names: cx.props.size.class_name(),
        marker: std::marker::PhantomData,
        disabled: cx.props.disabled,
    })
}
