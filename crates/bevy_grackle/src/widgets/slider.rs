use bevy::ui;
use bevy_egret::widgets::SliderChildProps;
use bevy_quill::prelude::*;
use static_init::dynamic;

const THUMB_SIZE: f32 = 18.;

// Style definitions for slider widget.

// The slider
#[dynamic]
static STYLE_SLIDER: StyleHandle =
    StyleHandle::build(|ss| ss.min_width(THUMB_SIZE).height(THUMB_SIZE));

#[dynamic]
static STYLE_THUMB_SPACER: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .top(0)
        .bottom(0)
        .left(0)
        .right(THUMB_SIZE)
});

// Slider thumb
#[dynamic]
static STYLE_THUMB: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .top(0.)
        .width(THUMB_SIZE)
        .height(THUMB_SIZE)
});

const CLS_DRAG: &str = "drag";

#[derive(Clone, PartialEq)]
pub struct SliderProps {
    pub id: &'static str,
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

// Horizontal slider widget
pub fn h_slider(mut cx: Cx<SliderProps>) -> impl View {
    // bevy_egret::widgets::h_slider.bind(bevy_egret::widgets::SliderProps {
    //     id: cx.props.id,
    //     min: cx.props.min,
    //     max: cx.props.max,
    //     value: cx.props.value,
    //     children: |s| FragmentClone::new(Element::new()),
    // })

    // Element::new().styled(STYLE_SLIDER.clone()).children((
    //     cx.props.track.clone(),
    //     Element::new()
    //         .styled(STYLE_THUMB_SPACER.clone())
    //         .class_names(CLS_DRAG.if_true(cx.read_atom(is_dragging)))
    //         .children(
    //             Element::new()
    //                 .styled((
    //                     STYLE_THUMB.clone(),
    //                     StyleHandle::build(|s| s.left(ui::Val::Percent(pos * 100.))),
    //                 ))
    //                 .children(cx.props.thumb.clone()),
    //         ),
    // ))
}

fn h_slider_inner(props: SliderChildProps) -> impl View {
    FragmentClone::new(Element::new())
}
