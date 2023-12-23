use bevy::render::color::Color;
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::tokens;

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
pub static STYLE_GRACKLE_THEME: StyleHandle = StyleHandle::build(|ss| {
    ss.set_var_color(tokens::PAGE_BG, COLOR_BG)
        .set_var_color(tokens::BUTTON_PRIMARY_BG, COLOR_PRIMARY)
        .set_var_color(tokens::BUTTON_PRIMARY_HOVER_BG, COLOR_PRIMARY)
        .set_var_color(tokens::BUTTON_PRIMARY_PRESSED_BG, COLOR_PRIMARY)
        .set_var_color(tokens::BUTTON_PRIMARY_BORDER, COLOR_BLACK)
        .set_var_color(tokens::BUTTON_PRIMARY_TEXT, COLOR_WHITE)
        .set_var_color(tokens::BUTTON_DEFAULT_BG, COLOR_G7)
        .set_var_color(tokens::BUTTON_DEFAULT_HOVER_BG, COLOR_G6)
        .set_var_color(tokens::BUTTON_DEFAULT_PRESSED_BG, COLOR_G5)
        .set_var_color(tokens::BUTTON_DEFAULT_BORDER, COLOR_G9)
        .set_var_color(tokens::BUTTON_DEFAULT_TEXT, COLOR_WHITE)
        .set_var_color(tokens::BUTTON_DANGER_BG, COLOR_DANGER)
        .set_var_color(tokens::BUTTON_DANGER_HOVER_BG, COLOR_DANGER)
        .set_var_color(tokens::BUTTON_DANGER_PRESSED_BG, COLOR_DANGER)
        .set_var_color(tokens::BUTTON_DANGER_BORDER, COLOR_BLACK)
        .set_var_color(tokens::BUTTON_DANGER_TEXT, COLOR_WHITE)
});
