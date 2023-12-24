use bevy::render::color::Color;
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
static STYLE_SIDEBAR: StyleHandle = StyleHandle::build(|ss| ss.background_color(COLOR_BG));

#[dynamic]
static STYLE_BUTTON_DEFAULT: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_G7)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_G5))
        .selector(":hover", |ss| ss.background_color(COLOR_G6))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_G5))
});

#[dynamic]
static STYLE_BUTTON_PRIMARY: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_PRIMARY)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_PRIMARY))
        .selector(":hover", |ss| ss.background_color(COLOR_PRIMARY))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_PRIMARY))
});

#[dynamic]
static STYLE_BUTTON_DANGER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_DANGER)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_DANGER))
        .selector(":hover", |ss| ss.background_color(COLOR_DANGER))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_DANGER))
});

pub fn init_grackle_theme<T>(cx: &mut Cx<T>) {
    cx.create_context(SIDEBAR, STYLE_SIDEBAR.clone());
    cx.create_context(BUTTON_DEFAULT, STYLE_BUTTON_DEFAULT.clone());
    cx.create_context(BUTTON_PRIMARY, STYLE_BUTTON_PRIMARY.clone());
    cx.create_context(BUTTON_DANGER, STYLE_BUTTON_DANGER.clone());
}
