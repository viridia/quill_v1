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

/// List of style tokens which are defined on this element.
#[derive(Component, Default)]
pub struct ElementTokens(pub TokenMap);

pub struct TokenLookup<'w, 's, 'h> {
    entity: Entity,
    query_tokens: &'h Query<'w, 's, (Entity, &'static mut ElementTokens)>,
    parent_query: &'h Query<'w, 's, &'static Parent, (With<Node>, With<Visibility>)>,
}

impl<'w, 's, 'h> TokenLookup<'w, 's, 'h> {
    pub(crate) fn new(
        entity: Entity,
        query: &'h Query<'w, 's, (Entity, &'static mut ElementTokens)>,
        parent_query: &'h Query<'w, 's, &'static Parent, (With<Node>, With<Visibility>)>,
    ) -> Self {
        Self {
            entity,
            query_tokens: query,
            parent_query,
        }
    }

    pub fn find(&self, token: &StyleToken) -> Option<TokenValue> {
        let mut entity = self.entity;
        loop {
            if let Ok((_, tokens)) = self.query_tokens.get(entity) {
                if let Some(val) = tokens.0.get(token) {
                    return Some(val.clone());
                }
            }
            match self.parent_query.get(entity) {
                Ok(parent) => entity = **parent,
                _ => return None,
            }
        }
    }
}
