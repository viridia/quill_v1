use bevy::{asset::AssetPath, render::color::Color};
use bevy_quill::prelude::*;
use static_init::dynamic;

use crate::tokens::*;

// Standard colors

pub const COLOR_BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
pub const COLOR_WHITE: Color = Color::rgb(1.0, 1.0, 1.0);

pub const COLOR_GRAY_50: Color = Color::rgb(0.980, 0.980, 0.980);
pub const COLOR_GRAY_100: Color = Color::rgb(0.961, 0.961, 0.961);
pub const COLOR_GRAY_200: Color = Color::rgb(0.933, 0.933, 0.933);
pub const COLOR_GRAY_300: Color = Color::rgb(0.878, 0.878, 0.878);
pub const COLOR_GRAY_400: Color = Color::rgb(0.741, 0.741, 0.741);
pub const COLOR_GRAY_500: Color = Color::rgb(0.620, 0.620, 0.620);
pub const COLOR_GRAY_600: Color = Color::rgb(0.459, 0.459, 0.459);
pub const COLOR_GRAY_700: Color = Color::rgb(0.380, 0.380, 0.380);
pub const COLOR_GRAY_800: Color = Color::rgb(0.259, 0.259, 0.259);
pub const COLOR_GRAY_900: Color = Color::rgb(0.129, 0.129, 0.129);

pub const COLOR_BLUEGRAY_50: Color = Color::rgb(0.925, 0.937, 0.941);
pub const COLOR_BLUEGRAY_100: Color = Color::rgb(0.812, 0.847, 0.859);
pub const COLOR_BLUEGRAY_200: Color = Color::rgb(0.690, 0.745, 0.773);
pub const COLOR_BLUEGRAY_300: Color = Color::rgb(0.565, 0.643, 0.682);
pub const COLOR_BLUEGRAY_400: Color = Color::rgb(0.471, 0.565, 0.612);
pub const COLOR_BLUEGRAY_500: Color = Color::rgb(0.376, 0.490, 0.545);
pub const COLOR_BLUEGRAY_600: Color = Color::rgb(0.329, 0.431, 0.478);
pub const COLOR_BLUEGRAY_700: Color = Color::rgb(0.263, 0.353, 0.392);
pub const COLOR_BLUEGRAY_800: Color = Color::rgb(0.216, 0.278, 0.314);
pub const COLOR_BLUEGRAY_900: Color = Color::rgb(0.149, 0.196, 0.220);

pub const COLOR_TEAL_50: Color = Color::rgb(0.878, 0.949, 0.945);
pub const COLOR_TEAL_100: Color = Color::rgb(0.698, 0.875, 0.843);
pub const COLOR_TEAL_200: Color = Color::rgb(0.502, 0.796, 0.768);
pub const COLOR_TEAL_300: Color = Color::rgb(0.302, 0.714, 0.675);
pub const COLOR_TEAL_400: Color = Color::rgb(0.149, 0.663, 0.612);
pub const COLOR_TEAL_500: Color = Color::rgb(0.000, 0.588, 0.533);
pub const COLOR_TEAL_600: Color = Color::rgb(0.000, 0.537, 0.478);
pub const COLOR_TEAL_700: Color = Color::rgb(0.000, 0.475, 0.420);
pub const COLOR_TEAL_800: Color = Color::rgb(0.000, 0.412, 0.361);
pub const COLOR_TEAL_900: Color = Color::rgb(0.000, 0.302, 0.251);

pub const COLOR_PRIMARY: Color = Color::rgb(0.220, 0.345, 0.408);
pub const COLOR_DANGER: Color = Color::rgb(0.267, 0.000, 0.333);

#[dynamic]
static STYLE_TYPOGRAPHY: StyleHandle = StyleHandle::build(|ss| {
    ss.font_size(14.).font(Some(AssetPath::from(
        "grackle://fonts/Ubuntu/Ubuntu-Medium.ttf",
    )))
});

// Sidebar

#[dynamic]
static STYLE_LT_SIDEBAR: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_GRAY_400)
        .font_size(14.)
        .font(Some(AssetPath::from(
            "grackle://fonts/Ubuntu/Ubuntu-Medium.ttf",
        )))
});

#[dynamic]
static STYLE_DK_SIDEBAR: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_BLUEGRAY_800)
        .font_size(14.)
        .font(Some(AssetPath::from(
            "grackle://fonts/Ubuntu/Ubuntu-Medium.ttf",
        )))
});

// Buttons

#[dynamic]
static STYLE_LT_BUTTON_DEFAULT: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_GRAY_500)
        .border_color(COLOR_GRAY_700)
        .color(COLOR_GRAY_900)
        .selector(".pressed", |ss| ss.background_color(COLOR_GRAY_300))
        .selector(":hover", |ss| ss.background_color(COLOR_GRAY_400))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_GRAY_200))
        .selector(":focus", |ss| {
            ss.outline_color(COLOR_GRAY_400)
                .outline_width(2)
                .outline_offset(1)
        })
});

#[dynamic]
static STYLE_DK_BUTTON_DEFAULT: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_BLUEGRAY_900)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_BLUEGRAY_600))
        .selector(":hover", |ss| ss.background_color(COLOR_BLUEGRAY_700))
        .selector(":hover.pressed", |ss| {
            ss.background_color(COLOR_BLUEGRAY_500)
        })
        .selector(":focus", |ss| {
            ss.outline_color(COLOR_GRAY_400)
                .outline_width(2)
                .outline_offset(1)
        })
});

#[dynamic]
static STYLE_DK_BUTTON_PRIMARY: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_PRIMARY)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_PRIMARY))
        .selector(":hover", |ss| ss.background_color(COLOR_PRIMARY))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_PRIMARY))
        .selector(":focus", |ss| {
            ss.outline_color(COLOR_GRAY_400)
                .outline_width(2)
                .outline_offset(1)
        })
});

#[dynamic]
static STYLE_DK_BUTTON_DANGER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_DANGER)
        .border_color(COLOR_BLACK)
        .selector(".pressed", |ss| ss.background_color(COLOR_DANGER))
        .selector(":hover", |ss| ss.background_color(COLOR_DANGER))
        .selector(":hover.pressed", |ss| ss.background_color(COLOR_DANGER))
        .selector(":focus", |ss| {
            ss.outline_color(COLOR_GRAY_400)
                .outline_width(2)
                .outline_offset(1)
        })
});

// Splitter

#[dynamic]
static STYLE_LT_SPLITTER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_GRAY_500)
        .selector(".drag", |ss| ss.background_color(COLOR_GRAY_600))
});

// The decorative handle inside the splitter.
#[dynamic]
static STYLE_LT_SPLITTER_INNER: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_GRAY_600)
        .selector(":hover > &", |ss| ss.background_color(COLOR_GRAY_700))
        .selector(".drag > &", |ss| ss.background_color(COLOR_GRAY_700))
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

// Slider

#[dynamic]
static STYLE_LT_SLIDER_TRACK: StyleHandle =
    StyleHandle::build(|ss| ss.background_color(COLOR_GRAY_500));

#[dynamic]
static STYLE_LT_SLIDER_TRACK_ACTIVE: StyleHandle =
    StyleHandle::build(|ss| ss.background_color(COLOR_TEAL_500));

#[dynamic]
static STYLE_LT_SLIDER_THUMB: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_GRAY_200)
        .background_image(Some(AssetPath::from("grackle://icons/disc.png")))
        .selector(":hover > &,.drag > &", |ss| {
            ss.background_color(COLOR_GRAY_50)
        })
});

#[dynamic]
static STYLE_DK_SLIDER_TRACK: StyleHandle =
    StyleHandle::build(|ss| ss.background_color(COLOR_BLUEGRAY_800));

#[dynamic]
static STYLE_DK_SLIDER_TRACK_ACTIVE: StyleHandle =
    StyleHandle::build(|ss| ss.background_color(COLOR_TEAL_600));

#[dynamic]
static STYLE_DK_SLIDER_THUMB: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color("#777")
        .background_image(Some(AssetPath::from("grackle://icons/disc.png")))
        .selector(":hover > &,.drag > &", |ss| ss.background_color("#aaa"))
});

// Menus

#[dynamic]
static STYLE_LT_MENU_POPUP: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_GRAY_400)
        .border_color(COLOR_BLACK)
});

// The decorative handle inside the splitter.
#[dynamic]
static STYLE_LT_MENU_ITEM: StyleHandle = StyleHandle::build(|ss| {
    ss.color(COLOR_BLACK)
        .selector(":hover", |ss| ss.background_color(COLOR_GRAY_500))
        .selector(".selected", |ss| ss.background_color(COLOR_GRAY_600))
});

#[dynamic]
static STYLE_DK_MENU_POPUP: StyleHandle = StyleHandle::build(|ss| {
    ss.background_color(COLOR_BLUEGRAY_800)
        .border_color(COLOR_BLACK)
});

// The decorative handle inside the splitter.
#[dynamic]
static STYLE_DK_MENU_ITEM: StyleHandle = StyleHandle::build(|ss| {
    ss.color(COLOR_BLUEGRAY_200)
        .selector(":hover", |ss| ss.background_color(COLOR_BLUEGRAY_700))
        .selector(".selected", |ss| ss.background_color(COLOR_BLUEGRAY_600))
});

#[derive(PartialEq, Copy, Clone)]
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
            cx.define_scoped_value(H_SLIDER_TRACK, STYLE_LT_SLIDER_TRACK.clone());
            cx.define_scoped_value(H_SLIDER_TRACK_ACTIVE, STYLE_LT_SLIDER_TRACK_ACTIVE.clone());
            cx.define_scoped_value(H_SLIDER_THUMB, STYLE_LT_SLIDER_THUMB.clone());
            cx.define_scoped_value(MENU_POPUP, STYLE_LT_MENU_POPUP.clone());
            cx.define_scoped_value(MENU_ITEM, STYLE_LT_MENU_ITEM.clone());
        }
        GrackleTheme::Dark => {
            cx.define_scoped_value(TYPOGRAPHY, STYLE_TYPOGRAPHY.clone());
            cx.define_scoped_value(SIDEBAR, STYLE_DK_SIDEBAR.clone());
            cx.define_scoped_value(BUTTON_DEFAULT, STYLE_DK_BUTTON_DEFAULT.clone());
            cx.define_scoped_value(BUTTON_PRIMARY, STYLE_DK_BUTTON_PRIMARY.clone());
            cx.define_scoped_value(BUTTON_DANGER, STYLE_DK_BUTTON_DANGER.clone());
            cx.define_scoped_value(SPLITTER, STYLE_DK_SPLITTER.clone());
            cx.define_scoped_value(SPLITTER_INNER, STYLE_DK_SPLITTER_INNER.clone());
            cx.define_scoped_value(H_SLIDER_TRACK, STYLE_DK_SLIDER_TRACK.clone());
            cx.define_scoped_value(H_SLIDER_TRACK_ACTIVE, STYLE_DK_SLIDER_TRACK_ACTIVE.clone());
            cx.define_scoped_value(H_SLIDER_THUMB, STYLE_DK_SLIDER_THUMB.clone());
            cx.define_scoped_value(MENU_POPUP, STYLE_DK_MENU_POPUP.clone());
            cx.define_scoped_value(MENU_ITEM, STYLE_DK_MENU_ITEM.clone());
        }
    }
}
