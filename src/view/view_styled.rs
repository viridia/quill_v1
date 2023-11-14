use std::sync::Arc;

use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::{ElementContext, StyleSet, View};

use crate::node_span::NodeSpan;

/// List of style objects which are attached to a given UiNode.
#[derive(Component, Default)]
pub struct ElementStyles {
    pub styles: Vec<Arc<StyleSet>>,
    pub(crate) ancestor_depth: usize,
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
}

impl<V: View> View for ViewStyled<V> {
    type State = V::State;

    fn build(
        &self,
        ecx: &mut ElementContext,
        state: &mut Self::State,
        prev: &NodeSpan,
    ) -> NodeSpan {
        let nodes = self.inner.build(ecx, state, prev);
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut ecx.world.entity_mut(entity);
                let depth = self
                    .styles
                    .iter()
                    .map(|s| s.as_ref().depth())
                    .max()
                    .unwrap_or(0);
                match em.get_mut::<ElementStyles>() {
                    Some(mut sc) => {
                        sc.styles.clone_from(&self.styles);
                        sc.ancestor_depth = depth;
                    }
                    None => {
                        em.insert((
                            ElementStyles {
                                ancestor_depth: depth,
                                styles: self.styles.clone(),
                            },
                            ElementClasses(HashSet::new()),
                        ));
                    }
                }
            }
            NodeSpan::Fragment(_) => {
                panic!("Styles can only be applied to a single UiNode")
            }
        }
        nodes
    }

    fn raze(&self, ecx: &mut ElementContext, state: &mut Self::State, prev: &NodeSpan) {
        self.inner.raze(ecx, state, prev);
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

// impl StyleTuple for (StyleSet, StyleSet) {
//     fn to_vec(&self) -> Vec<Arc<StyleSet>> {
//         vec![self.0, self.1]
//     }
// }

// impl StyleTuple for (StyleSet, StyleSet, StyleSet) {
//     fn to_vec(&self) -> Vec<Arc<StyleSet>> {
//         vec![self.0, self.1, self.2]
//     }
// }
