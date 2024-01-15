#![allow(missing_docs)]

use super::{
    builder::StyleBuilder, computed::ComputedStyle, selector_matcher::SelectorMatcher,
    style_props::StyleSet,
};
use bevy::prelude::*;
use std::sync::Arc;

/// A sharable reference to a collection of UI style properties.
#[derive(Clone, Default)]
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
    pub fn apply_to(
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

    /// Return whether any of the selectors use the ':focus-within' pseudo-class.
    pub fn uses_focus_within(&self) -> bool {
        self.0.as_ref().uses_focus_within()
    }
}

impl PartialEq for StyleHandle {
    fn eq(&self, other: &Self) -> bool {
        // Reference-equality is all we need.
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

/// List of [`StyleHandle`]s which are attached to a given UiNode.
#[derive(Component, Default)]
pub struct ElementStyles {
    /// The collection of styles associated with this element.
    pub styles: Vec<StyleHandle>,

    /// How far up the hierarchy the selectors need to search
    pub(crate) selector_depth: usize,

    /// Whether any selectors use the :hover pseudo-class
    pub(crate) uses_hover: bool,

    /// Whether any selectors use the :focus-within pseudo-class
    pub(crate) uses_focus_within: bool,
}

impl ElementStyles {
    pub fn new(styles: &[StyleHandle]) -> Self {
        let selector_depth = styles.iter().map(|s| s.depth()).max().unwrap_or(0);
        let uses_hover = styles.iter().any(|s| s.uses_hover());
        let uses_focus_within = styles.iter().any(|s| s.uses_focus_within());
        Self {
            styles: styles.to_vec(),
            selector_depth,
            uses_hover,
            uses_focus_within,
        }
    }

    pub fn update(&mut self, styles: &[StyleHandle]) {
        self.styles = styles.to_vec();
        self.selector_depth = self.styles.iter().map(|s| s.depth()).max().unwrap_or(0);
        self.uses_hover = self.styles.iter().any(|s| s.uses_hover());
        self.uses_focus_within = self.styles.iter().any(|s| s.uses_focus_within());
    }
}

/// Component used to store inherited text style properties. This is set whenever an element
/// has one or more style properties which affect text rendering, even if the element is not
/// a text node itself. This is used to calculate the inherited text style for child nodes,
/// and also whether or not the text style has changed.
#[derive(Component, Default, PartialEq, Clone)]
pub struct TextStyles {
    /// The collection of styles associated with this element.
    pub font: Option<Handle<Font>>,

    /// The size of the font.
    pub font_size: Option<f32>,

    /// Text color
    pub color: Option<Color>,
}
