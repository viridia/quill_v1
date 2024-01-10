use crate::node_span::NodeSpan;
use crate::{BuildContext, View};
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

    fn set_name(&self, nodes: &NodeSpan, bc: &mut BuildContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut bc.entity_mut(*entity);
                em.insert(Name::new(self.name.to_string()));
            }

            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    self.set_name(node, bc);
                }
            }
        }
    }
}

impl<'a, V: View> View for ViewNamed<'a, V> {
    type State = V::State;

    fn nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(bc, state)
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        let state = self.inner.build(bc);
        self.set_name(&self.nodes(bc, &state), bc);
        state
    }

    fn update(&self, bc: &mut BuildContext, state: &mut Self::State) {
        self.inner.update(bc, state);
        // Don't think we need to update on rebuild
        // self.set_name(&mut self.nodes(bc, state), bc);
    }

    fn assemble(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(bc, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.inner.raze(world, state);
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
