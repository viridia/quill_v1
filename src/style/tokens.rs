use std::fmt::Debug;

use bevy::{prelude::*, ui, utils::HashMap};

/// A "design token" identifies a variable which can be used in a style expression.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct StyleToken {
    pub(crate) name: &'static str,
}

impl StyleToken {
    /// Construct a new variable token given a name.
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl From<&'static str> for StyleToken {
    fn from(name: &'static str) -> Self {
        Self::new(name)
    }
}

/// Dynamically-typed token value
#[derive(Clone, PartialEq, Debug)]
pub enum TokenValue {
    Color(Option<Color>),
    Length(ui::Val),
}

/// HashMap of style token to style token value.
pub type TokenMap = HashMap<StyleToken, TokenValue>;
