use bevy_quill::{ContextKey, StyleHandle};

pub const SIDEBAR: ContextKey<StyleHandle> = ContextKey::new("sidebar");
pub const BUTTON_DEFAULT: ContextKey<StyleHandle> = ContextKey::new("button-default");
pub const BUTTON_PRIMARY: ContextKey<StyleHandle> = ContextKey::new("button-primary");
pub const BUTTON_DANGER: ContextKey<StyleHandle> = ContextKey::new("button-danger");
