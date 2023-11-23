use std::sync::Arc;

use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::{ElementContext, StyleSet, View};

use crate::node_span::NodeSpan;

/// List of style objects which are attached to a given UiNode.
#[derive(Component, Default)]
pub struct ElementStyles {
    pub styles: Vec<Arc<StyleSet>>,

    // How far up the hierarchy the selectors need to search
    pub(crate) selector_depth: usize,

    // Whether any selectors use the :hover pseudo-class
    pub(crate) uses_hover: bool,
    // TODO: Inherited
}

/// List of style objects which are attached to a given UiNode.
#[derive(Component, Default)]
pub struct ElementClasses(pub HashSet<String>);

impl ElementClasses {
    pub fn add_class(&mut self, cls: &str) {
        self.0.insert(cls.to_string());
    }

    pub fn remove_class(&mut self, cls: &str) {
        self.0.remove(cls);
    }
}

// A wrapper view which applies styles to the output of an inner view.
pub struct ViewStyled<V: View> {
    inner: V,
    styles: Vec<Arc<StyleSet>>,
}

impl<V: View> ViewStyled<V> {
    pub fn new<S: StyleTuple>(inner: V, items: S) -> Self {
        Self {
            inner,
            styles: items.to_vec(),
        }
    }

    fn insert_styles(&self, nodes: &NodeSpan, ecx: &mut ElementContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut ecx.world.entity_mut(*entity);
                let selector_depth = self
                    .styles
                    .iter()
                    .map(|s| s.as_ref().depth())
                    .max()
                    .unwrap_or(0);
                let uses_hover = self
                    .styles
                    .iter()
                    .map(|s| s.as_ref().uses_hover())
                    .max()
                    .unwrap_or(false);

                match em.get_mut::<ElementStyles>() {
                    Some(mut sc) => {
                        sc.styles.clone_from(&self.styles);
                        sc.selector_depth = selector_depth;
                        sc.uses_hover = uses_hover;
                    }
                    None => {
                        em.insert((
                            ElementStyles {
                                styles: self.styles.clone(),
                                selector_depth,
                                uses_hover,
                            },
                            ElementClasses(HashSet::new()),
                        ));
                    }
                }
            }
            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    self.insert_styles(node, ecx);
                }
            }
        }
    }
}

impl<V: View> View for ViewStyled<V> {
    type State = V::State;

    fn build(&self, ecx: &mut ElementContext) -> (Self::State, NodeSpan) {
        let (state, nodes) = self.inner.build(ecx);
        self.insert_styles(&nodes, ecx);
        (state, nodes)
    }

    fn rebuild(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        let nodes = self.inner.rebuild(ecx, state, prev);
        self.insert_styles(&nodes, ecx);
        nodes
    }

    fn collect(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        nodes: &NodeSpan,
    ) -> NodeSpan {
        self.inner.collect(ecx, state, nodes)
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, nodes: &NodeSpan) {
        self.inner.raze(ecx, state, nodes);
    }

    // Apply styles to this view.
    // TODO: Possible optimization by replacing the style object rather than wrapping it.
    // fn styled<S: StyleTuple<'a>>(&self, styles: S) -> StyledView<'a, Self> {
    //     StyledView::<'a, Self>::new(&self, styles)
    // }
}

// StyleTuple - a variable-length tuple of styles.

// TODO: Turn this into a macro once it's stable.
pub trait StyleTuple: Send + Sync {
    fn to_vec(&self) -> Vec<Arc<StyleSet>>;
}

impl StyleTuple for () {
    fn to_vec(&self) -> Vec<Arc<StyleSet>> {
        Vec::new()
    }
}

impl StyleTuple for Arc<StyleSet> {
    fn to_vec(&self) -> Vec<Arc<StyleSet>> {
        vec![self.clone()]
    }
}

impl StyleTuple for &Arc<StyleSet> {
    fn to_vec(&self) -> Vec<Arc<StyleSet>> {
        vec![(*self).clone()]
    }
}

impl StyleTuple for (Arc<StyleSet>,) {
    fn to_vec(&self) -> Vec<Arc<StyleSet>> {
        vec![self.0.clone()]
    }
}

impl StyleTuple for (Arc<StyleSet>, Arc<StyleSet>) {
    fn to_vec(&self) -> Vec<Arc<StyleSet>> {
        vec![self.0.clone(), self.1.clone()]
    }
}
