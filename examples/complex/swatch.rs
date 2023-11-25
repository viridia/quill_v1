use bevy::{prelude::*, ui};
use bevy_quill::prelude::*;
use lazy_static::lazy_static;

// Style definitions for color swatch widget.
lazy_static! {
    // A swatch widget
    static ref STYLE_SWATCH: StyleHandle = StyleHandle::build(|ss| ss
        .border_color(Some(Color::BLACK))
        .border(1)
        .min_width(9)
        .min_height(16)
        .outline_color(Some(Color::NONE))
        .outline_offset(1.)
        .outline_width(1.)
        .selector(":hover", |ss| ss
            .outline_color(Some(Color::hex("#fff4").unwrap()))));
    // A swatch grid
    static ref STYLE_SWATCH_GRID: StyleHandle = StyleHandle::build(|ss| ss
        .display(ui::Display::Grid)
        .grid_template_columns(vec![ui::RepeatedGridTrack::fr(5, 1.)])
        // .grid_auto_flow(ui::GridAutoFlow::Column)
        .gap(3)
        .min_width(9)
        .min_height(16)
    );
}

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
        .styled((
            STYLE_SWATCH_GRID.clone(),
            // StyleHandle::build(|s| s.background_color(Some(cx.props.color))),
        ))
        .children(For::each(cx.props.colors, |color| {
            swatch.bind(SwatchProps { color: *color })
        }))
}
