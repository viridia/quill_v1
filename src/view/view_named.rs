use crate::node_span::NodeSpan;
use crate::{View, ViewContext};
use bevy::prelude::*;

// A wrapper view which applies styles to the output of an inner view.
pub struct ViewNamed<'a, V: View> {
    inner: V,
    name: &'a str,
}

impl<'a, V: View> ViewNamed<'a, V> {
    pub fn new(inner: V, name: &'a str) -> Self {
        Self { inner, name }
    }

    fn set_name(&self, nodes: &NodeSpan, vc: &mut ViewContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut vc.entity_mut(*entity);
                em.insert(Name::new(self.name.to_string()));
            }

            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    self.set_name(node, vc);
                }
            }
        }
    }
}

impl<'a, V: View> View for ViewNamed<'a, V> {
    type State = V::State;

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(vc, state)
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        let state = self.inner.build(vc);
        self.set_name(&self.nodes(vc, &state), vc);
        state
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.update(vc, state);
        // Don't think we need to update on rebuild
        // self.set_name(&mut self.nodes(vc, state), vc);
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(vc, state)
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.raze(vc, state);
    }
}

impl<'a, V: View> Clone for ViewNamed<'a, V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            name: self.name,
        }
    }
}

impl<'a, V: View> PartialEq for ViewNamed<'a, V>
where
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.name == other.name
    }
}
