#![allow(missing_docs)]

use super::{
    builder::StyleBuilder, computed::ComputedStyle, selector_matcher::SelectorMatcher,
    style_props::StyleSet,
};
use bevy::prelude::*;
use std::sync::Arc;

/// A sharable reference to a collection of UI style properties.
#[derive(Clone)]
pub struct StyleHandle(pub Arc<StyleSet>);

/// Handle which maintains a shared reference to a set of styles and selectors.
impl StyleHandle {
    /// Build a StyleSet using a builder callback.
    pub fn build(builder_fn: impl FnOnce(&mut StyleBuilder) -> &mut StyleBuilder) -> Self {
        let mut builder = StyleBuilder::new();
        builder_fn(&mut builder);
        Self(Arc::new(StyleSet {
            props: builder.props,
            selectors: builder.selectors,
        }))
    }

    /// Merge the style properties into a computed `Style` object.
    pub fn apply_to<'a>(
        &self,
        computed: &mut ComputedStyle,
        matcher: &SelectorMatcher,
        entity: &Entity,
    ) {
        self.0.as_ref().apply_to(computed, matcher, entity);
    }

    /// Return the number of UiNode levels referenced by selectors.
    pub fn depth(&self) -> usize {
        self.0.as_ref().depth()
    }

    /// Return whether any of the selectors use the ':hover' pseudo-class.
    pub fn uses_hover(&self) -> bool {
        self.0.as_ref().uses_hover()
    }
}

impl PartialEq for StyleHandle {
    fn eq(&self, other: &Self) -> bool {
        // Reference-equality is all we need.
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl Default for StyleHandle {
    fn default() -> Self {
        Self(Default::default())
    }
}

/// List of style objects which are attached to a given UiNode.
#[derive(Component, Default)]
pub struct ElementStyles {
    /// The collection of styles associated with this element.
    pub styles: Vec<StyleHandle>,

    /// How far up the hierarchy the selectors need to search
    pub(crate) selector_depth: usize,

    /// Whether any selectors use the :hover pseudo-class
    pub(crate) uses_hover: bool,
    // Whether any selectors use inherited properties.
    // pub(crate) uses_inherited: bool,
}
