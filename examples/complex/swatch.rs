use bevy::{prelude::*, ui};
use bevy_quill::prelude::*;
use static_init::dynamic;

// Style definitions for color swatch widget.

// A swatch widget
#[dynamic]
static STYLE_SWATCH: StyleHandle = StyleHandle::build(|ss| {
    ss.border_color(Some(Color::BLACK))
        .border(1)
        .min_width(9)
        .height(16)
        .outline_color(Some(Color::NONE))
        .outline_offset(1.)
        .outline_width(1.)
        .selector(":hover", |ss| ss.outline_color("#fff4"))
});

// A swatch grid
#[dynamic]
static STYLE_SWATCH_GRID: StyleHandle = StyleHandle::build(|ss| {
    ss.display(ui::Display::Grid)
        .grid_template_columns(vec![ui::RepeatedGridTrack::fr(5, 1.)])
        .gap(3)
        .min_width(9)
        .min_height(16)
});

#[derive(Clone, PartialEq)]
pub struct SwatchProps {
    pub color: Color,
}

// Color swatch
pub fn swatch(cx: Cx<SwatchProps>) -> impl View {
    Element::new().styled((
        STYLE_SWATCH.clone(),
        StyleHandle::build(|s| s.background_color(Some(cx.props.color))),
    ))
}

#[derive(Clone, PartialEq)]
pub struct SwatchGridProps<'a> {
    pub colors: &'a [Color],
    pub row_span: usize,
}

// Color swatch grid
pub fn swatch_grid(cx: Cx<SwatchGridProps>) -> impl View {
    Element::new()
        .styled(STYLE_SWATCH_GRID.clone())
        .children(For::each(cx.props.colors, |color| {
            swatch.bind(SwatchProps { color: *color })
        }))
}

// Color swatch list
pub fn swatch_list(cx: Cx<SwatchGridProps>) -> impl View {
    For::each(cx.props.colors, |color| {
        swatch.bind(SwatchProps { color: *color })
    })
}
