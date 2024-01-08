use bevy::ecs::world::World;

use crate::node_span::NodeSpan;
use crate::{BuildContext, ElementClasses, ElementStyles, StyleHandle, StyleTuple, View};

// A wrapper view which applies styles to the output of an inner view.
pub struct ViewStyled<V: View> {
    inner: V,
    styles: Vec<StyleHandle>,
}

impl<V: View> ViewStyled<V> {
    pub fn new<S: StyleTuple>(inner: V, items: S) -> Self {
        Self {
            inner,
            styles: items.to_vec(),
        }
    }

    fn insert_styles(&self, nodes: &NodeSpan, vc: &mut BuildContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut vc.entity_mut(*entity);
                let selector_depth = self.styles.iter().map(|s| s.depth()).max().unwrap_or(0);
                let uses_hover = self.styles.iter().any(|s| s.uses_hover());

                match em.get_mut::<ElementStyles>() {
                    Some(mut sc) => {
                        sc.styles.clone_from(&self.styles);
                        sc.selector_depth = selector_depth;
                        sc.uses_hover = uses_hover;
                    }
                    None => {
                        em.insert((ElementStyles {
                            styles: self.styles.clone(),
                            selector_depth,
                            uses_hover,
                        },));
                    }
                }

                if em.get_mut::<ElementClasses>().is_none() {
                    em.insert(ElementClasses::default());
                }
            }

            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    self.insert_styles(node, vc);
                }
            }
        }
    }
}

impl<V: View> View for ViewStyled<V> {
    type State = V::State;

    fn nodes(&self, vc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(vc, state)
    }

    fn build(&self, vc: &mut BuildContext) -> Self::State {
        let state = self.inner.build(vc);
        self.insert_styles(&self.nodes(vc, &state), vc);
        state
    }

    fn update(&self, vc: &mut BuildContext, state: &mut Self::State) {
        self.inner.update(vc, state);
        self.insert_styles(&self.nodes(vc, state), vc);
    }

    fn assemble(&self, vc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(vc, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.inner.raze(world, state);
    }
}

impl<V: View> Clone for ViewStyled<V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            styles: self.styles.clone(),
        }
    }
}

impl<V: View> PartialEq for ViewStyled<V>
where
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.styles == other.styles
    }
}
