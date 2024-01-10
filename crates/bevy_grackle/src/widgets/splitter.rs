use bevy::ui;
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::tokens::{SPLITTER, SPLITTER_INNER};

// Style definitions for the splitter widget.

// The splitter widget
#[dynamic]
static STYLE_VSPLITTER: StyleHandle = StyleHandle::build(|ss| {
    ss.align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .gap(8)
        .width(9)
});

// The decorative handle inside the splitter.
#[dynamic]
static STYLE_VSPLITTER_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Flex)
        .width(3)
        .pointer_events(PointerEvents::None)
        .height(ui::Val::Percent(5.))
});

#[derive(Clone, PartialEq)]
pub struct SplitterProps {
    pub value: f32,
    pub id: &'static str,
}

// Vertical splitter bar which can be dragged
pub fn v_splitter(cx: Cx<SplitterProps>) -> impl View {
    bevy_egret::widgets::v_splitter.bind(bevy_egret::widgets::SplitterProps {
        id: cx.props.id,
        children: Fragment::new((
            Element::new().styled((
                STYLE_VSPLITTER_INNER.clone(),
                cx.get_scoped_value(SPLITTER_INNER),
            )),
            Element::new().styled((
                STYLE_VSPLITTER_INNER.clone(),
                cx.get_scoped_value(SPLITTER_INNER),
            )),
        )),
        style: (
            STYLE_VSPLITTER.clone(),
            cx.get_scoped_value(SPLITTER).clone(),
        ),
        value: cx.props.value,
    })
}
