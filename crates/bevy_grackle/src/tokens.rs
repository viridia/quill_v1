use bevy_quill::{ScopedValueKey, StyleHandle};

pub const TYPOGRAPHY: ScopedValueKey<StyleHandle> = ScopedValueKey::new("typography");
pub const SIDEBAR: ScopedValueKey<StyleHandle> = ScopedValueKey::new("sidebar");
pub const BUTTON_DEFAULT: ScopedValueKey<StyleHandle> = ScopedValueKey::new("button-default");
pub const BUTTON_PRIMARY: ScopedValueKey<StyleHandle> = ScopedValueKey::new("button-primary");
pub const BUTTON_DANGER: ScopedValueKey<StyleHandle> = ScopedValueKey::new("button-danger");
pub const SPLITTER: ScopedValueKey<StyleHandle> = ScopedValueKey::new("splitter");
pub const SPLITTER_INNER: ScopedValueKey<StyleHandle> = ScopedValueKey::new("splitter-inner");
pub const H_SLIDER_TRACK: ScopedValueKey<StyleHandle> = ScopedValueKey::new("h-slider-track");
pub const H_SLIDER_TRACK_ACTIVE: ScopedValueKey<StyleHandle> =
    ScopedValueKey::new("h-slider-track-active");
pub const H_SLIDER_THUMB: ScopedValueKey<StyleHandle> = ScopedValueKey::new("h-slider-thumb");
pub const MENU_POPUP: ScopedValueKey<StyleHandle> = ScopedValueKey::new("menu-popup");
pub const MENU_ITEM: ScopedValueKey<StyleHandle> = ScopedValueKey::new("menu-item");
