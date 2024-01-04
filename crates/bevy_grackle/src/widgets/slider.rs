use std::sync::Arc;

use bevy::{asset::AssetPath, ui};
use bevy_egret::widgets::SliderChildProps;
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::tokens::{H_SLIDER_THUMB, H_SLIDER_TRACK, H_SLIDER_TRACK_ACTIVE};

const THUMB_SIZE: f32 = 18.;

// Style definitions for slider widget.

// The slider
#[dynamic]
static STYLE_SLIDER: StyleHandle =
    StyleHandle::build(|ss| ss.min_width(THUMB_SIZE).height(THUMB_SIZE));

// Slider track
#[dynamic]
static STYLE_TRACK: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .top(ui::Val::Percent(40.))
        .bottom(ui::Val::Percent(40.))
        .left(0)
        .right(0)
});

#[dynamic]
static STYLE_TRACK_ACTIVE: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .top(ui::Val::Percent(40.))
        .bottom(ui::Val::Percent(40.))
        .left(0)
});

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

// Slider thumb fg
#[dynamic]
static STYLE_THUMB_FG: StyleHandle = StyleHandle::build(|ss| {
    ss.position(ui::PositionType::Absolute)
        .top(0.)
        .left(0.)
        .width(THUMB_SIZE)
        .height(THUMB_SIZE)
        .z_index(1)
        .pointer_events(PointerEvents::None)
});

// Slider thumb shadow
#[dynamic]
static STYLE_THUMB_SHADOW: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#0008")
        .background_image(Some(AssetPath::from("grackle://icons/disc.png")))
        .position(ui::PositionType::Absolute)
        .top(3.)
        .left(3.)
        .width(THUMB_SIZE)
        .height(THUMB_SIZE)
        .z_index(-1)
        .pointer_events(PointerEvents::None)
});

#[derive(PartialEq, Default)]
pub struct SliderProps<S: StyleTuple = ()> {
    pub id: &'static str,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub style: S,
}

// Horizontal slider widget
pub fn h_slider<S: StyleTuple + PartialEq + 'static>(cx: Cx<SliderProps<S>>) -> impl View {
    // Get styles from theme. These will be combined with built-in styles.
    let track_style = cx.get_scoped_value(H_SLIDER_TRACK);
    let track_active_style = cx.get_scoped_value(H_SLIDER_TRACK_ACTIVE);
    let thumb_style = cx.get_scoped_value(H_SLIDER_THUMB);
    // The headless slider accepts a closure which renders the elements based on the current
    // slider position.
    bevy_egret::widgets::h_slider.bind(bevy_egret::widgets::SliderProps {
        id: cx.props.id,
        min: cx.props.min,
        max: cx.props.max,
        value: cx.props.value,
        thumb_size: THUMB_SIZE,
        style: (STYLE_SLIDER.clone(), cx.props.style.clone()),
        children: Arc::new(move |spc: SliderChildProps| {
            Fragment::new((
                Element::new().styled((STYLE_TRACK.clone(), track_style.clone())),
                Element::new().styled((
                    STYLE_TRACK_ACTIVE.clone(),
                    track_active_style.clone(),
                    StyleHandle::build(|s| s.width(ui::Val::Percent(spc.percent))),
                )),
                Element::new()
                    .styled(STYLE_THUMB_SPACER.clone())
                    .class_names("drag".if_true(spc.is_dragging))
                    .children(
                        Element::new()
                            .styled((
                                STYLE_THUMB.clone(),
                                StyleHandle::build(|s| s.left(ui::Val::Percent(spc.percent))),
                            ))
                            .children((
                                Element::new()
                                    .styled((STYLE_THUMB_FG.clone(), thumb_style.clone())),
                                Element::new().styled(STYLE_THUMB_SHADOW.clone()),
                            )),
                    ),
            ))
        }),
    })
}
