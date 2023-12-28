use bevy::{asset::AssetPath, render::color::Color};
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::tokens::*;

pub const COLOR_BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
pub const COLOR_WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
pub const COLOR_BG: Color = Color::rgb(0.200 * 0.9, 0.200 * 0.9, 0.306 * 0.9);

pub const COLOR_G0: Color = Color::rgb(0.937, 0.937, 0.996);
pub const COLOR_G1: Color = Color::rgb(0.816, 0.816, 0.894);
pub const COLOR_G2: Color = Color::rgb(0.702, 0.702, 0.804);
pub const COLOR_G3: Color = Color::rgb(0.580, 0.580, 0.722);
pub const COLOR_G4: Color = Color::rgb(0.459, 0.459, 0.643);
pub const COLOR_G5: Color = Color::rgb(0.361, 0.361, 0.537);
pub const COLOR_G6: Color = Color::rgb(0.278, 0.278, 0.420);
pub const COLOR_G7: Color = Color::rgb(0.212, 0.212, 0.318);
pub const COLOR_G8: Color = Color::rgb(0.118, 0.118, 0.192);
pub const COLOR_G9: Color = Color::rgb(0.035, 0.035, 0.090);

pub const COLOR_TEAL0: Color = Color::rgb(0.871, 1.000, 0.976);
pub const COLOR_TEAL1: Color = Color::rgb(0.702, 1.000, 0.925);
pub const COLOR_TEAL2: Color = Color::rgb(0.522, 0.996, 0.871);
pub const COLOR_TEAL3: Color = Color::rgb(0.345, 0.996, 0.824);
pub const COLOR_TEAL4: Color = Color::rgb(0.220, 0.996, 0.780);
pub const COLOR_TEAL5: Color = Color::rgb(0.165, 0.898, 0.678);
pub const COLOR_TEAL6: Color = Color::rgb(0.114, 0.698, 0.529);
pub const COLOR_TEAL7: Color = Color::rgb(0.063, 0.502, 0.380);
pub const COLOR_TEAL8: Color = Color::rgb(0.220, 0.282, 0.345);
pub const COLOR_TEAL9: Color = Color::rgb(0.000, 0.106, 0.071);

pub const COLOR_PRIMARY: Color = Color::rgb(0.220, 0.345, 0.408);
pub const COLOR_DANGER: Color = Color::rgb(0.267, 0.000, 0.333);

#[dynamic]
static STYLE_TYPOGRAPHY: StyleHandle = StyleHandle::build(|ss| {
    ss.font_size(14.).font(Some(AssetPath::from(
        "grackle://fonts/Ubuntu/Ubuntu-Medium.ttf",
    )))
});

#[dynamic]
static STYLE_LT_SIDEBAR: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_G2)
        .font_size(14.)
        .font(Some(AssetPath::from(
            "grackle://fonts/Ubuntu/Ubuntu-Medium.ttf",
        )))
});

#[dynamic]
static STYLE_DK_SIDEBAR: StyleHandle = StyleHandle::build(|ss| ss.background_color(COLOR_BG));

#[dynamic]
static STYLE_LT_BUTTON_DEFAULT: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_G5)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_G6))
        .selector(":hover", |ss| ss.background_color(COLOR_G4))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_G6))
});

#[dynamic]
static STYLE_DK_BUTTON_DEFAULT: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_G7)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_G5))
        .selector(":hover", |ss| ss.background_color(COLOR_G6))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_G5))
});

#[dynamic]
static STYLE_DK_BUTTON_PRIMARY: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_PRIMARY)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_PRIMARY))
        .selector(":hover", |ss| ss.background_color(COLOR_PRIMARY))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_PRIMARY))
});

#[dynamic]
static STYLE_DK_BUTTON_DANGER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_DANGER)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_DANGER))
        .selector(":hover", |ss| ss.background_color(COLOR_DANGER))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_DANGER))
});

#[dynamic]
static STYLE_LT_SPLITTER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_G4)
        .selector(".drag", |ss| ss.background_color(COLOR_G5))
});

// The decorative handle inside the splitter.
#[dynamic]
static STYLE_LT_SPLITTER_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_G3)
        .selector(":hover > &", |ss| ss.background_color(COLOR_G2))
        .selector(".drag > &", |ss| ss.background_color(COLOR_G4))
});

#[dynamic]
static STYLE_DK_SPLITTER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#181818")
        .selector(".drag", |ss| ss.background_color("#080808"))
});

// The decorative handle inside the splitter.
#[dynamic]
static STYLE_DK_SPLITTER_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#282828")
        .selector(":hover > &", |ss| ss.background_color("#383838"))
        .selector(".drag > &", |ss| ss.background_color("#484848"))
});

pub enum GrackleTheme {
    Light,
    Dark,
}

pub fn init_grackle_theme<T>(cx: &mut Cx<T>, theme: GrackleTheme) {
    match theme {
        GrackleTheme::Light => {
            cx.define_scoped_value(TYPOGRAPHY, STYLE_TYPOGRAPHY.clone());
            cx.define_scoped_value(SIDEBAR, STYLE_LT_SIDEBAR.clone());
            cx.define_scoped_value(BUTTON_DEFAULT, STYLE_LT_BUTTON_DEFAULT.clone());
            cx.define_scoped_value(BUTTON_PRIMARY, STYLE_DK_BUTTON_PRIMARY.clone());
            cx.define_scoped_value(BUTTON_DANGER, STYLE_DK_BUTTON_DANGER.clone());
            cx.define_scoped_value(SPLITTER, STYLE_LT_SPLITTER.clone());
            cx.define_scoped_value(SPLITTER_INNER, STYLE_LT_SPLITTER_INNER.clone());
        }
        GrackleTheme::Dark => {
            cx.define_scoped_value(TYPOGRAPHY, STYLE_TYPOGRAPHY.clone());
            cx.define_scoped_value(SIDEBAR, STYLE_DK_SIDEBAR.clone());
            cx.define_scoped_value(BUTTON_DEFAULT, STYLE_DK_BUTTON_DEFAULT.clone());
            cx.define_scoped_value(BUTTON_PRIMARY, STYLE_DK_BUTTON_PRIMARY.clone());
            cx.define_scoped_value(BUTTON_DANGER, STYLE_DK_BUTTON_DANGER.clone());
            cx.define_scoped_value(SPLITTER, STYLE_DK_SPLITTER.clone());
            cx.define_scoped_value(SPLITTER_INNER, STYLE_DK_SPLITTER_INNER.clone());
        }
    }
}
