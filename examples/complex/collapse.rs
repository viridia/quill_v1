use super::element_rect::ElementRectApi;
use bevy::{prelude::*, ui};
use bevy_quill::prelude::*;
use static_init::dynamic;

#[dynamic]
static STYLE_COLLAPSE: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .flex_grow(1.)
        .align_items(ui::AlignItems::Stretch)
        .margin_left(16)
        .height(0)
        .overflow(ui::OverflowAxis::Clip)
        .transition(&vec![Transition {
            property: TransitionProperty::Height,
            duration: 0.3,
            timing: timing::EASE_IN_OUT,
            ..default()
        }])
});

#[dynamic]
static STYLE_COLLAPSE_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .position(ui::PositionType::Absolute)
        .left(0)
        .top(0)
        .flex_direction(ui::FlexDirection::Column)
        .flex_grow(1.)
        .align_items(ui::AlignItems::Stretch)
        .margin_left(16)
        .overflow_y(ui::OverflowAxis::Clip)
});

#[derive(Clone, PartialEq, Default)]
pub struct CollapseProps<V: View> {
    pub expanded: bool,
    pub children: V,
    pub style: StyleHandle,
}

pub fn collapse<V: View + Clone>(mut cx: Cx<CollapseProps<V>>) -> impl View {
    let id_inner = cx.create_entity();
    let rect = cx.use_element_rect(id_inner);
    let height = if cx.props.expanded { rect.height() } else { 0. };

    Element::new()
        .styled((
            STYLE_COLLAPSE.clone(),
            cx.props.style.clone(),
            StyleHandle::build(|ss| ss.height(height).width(rect.width())),
        ))
        .children(
            RefElement::new(id_inner)
                .styled(STYLE_COLLAPSE_INNER.clone())
                .children(cx.props.children.clone()),
        )
}
