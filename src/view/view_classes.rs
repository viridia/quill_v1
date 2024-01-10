use crate::node_span::NodeSpan;
use crate::{BuildContext, ClassNames, ElementClasses, View};
use bevy::ecs::world::World;
use bevy::utils::HashSet;

// A wrapper view which applies styles to the output of an inner view.
pub struct ViewClasses<V: View> {
    inner: V,
    class_names: HashSet<String>,
}

impl<V: View> ViewClasses<V> {
    pub fn new<'a, S: ClassNames<'a>>(inner: V, items: S) -> Self {
        let mut class_names: HashSet<String> = HashSet::with_capacity(items.len());
        items.add_classes(&mut class_names);
        Self { inner, class_names }
    }

    fn set_class_names(&self, nodes: &NodeSpan, bc: &mut BuildContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut bc.entity_mut(*entity);
                match em.get_mut::<ElementClasses>() {
                    Some(mut ec) => {
                        if !ec.0.eq(&self.class_names) {
                            ec.as_mut().0.clone_from(&self.class_names);
                        }
                    }
                    None => {
                        em.insert((ElementClasses(self.class_names.clone()),));
                    }
                }
            }
            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    self.set_class_names(node, bc);
                }
            }
        }
    }
}

impl<V: View> View for ViewClasses<V> {
    type State = V::State;

    fn nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(bc, state)
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        let state = self.inner.build(bc);
        self.set_class_names(&self.nodes(bc, &state), bc);
        state
    }

    fn update(&self, bc: &mut BuildContext, state: &mut Self::State) {
        self.inner.update(bc, state);
        self.set_class_names(&self.nodes(bc, state), bc);
    }

    fn assemble(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(bc, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.inner.raze(world, state);
    }
}

impl<V: View> Clone for ViewClasses<V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            class_names: self.class_names.clone(),
        }
    }
}

impl<V: View> PartialEq for ViewClasses<V>
where
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.class_names == other.class_names
    }
}
