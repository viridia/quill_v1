use crate::node_span::NodeSpan;
use crate::{View, ViewContext};
use bevy::prelude::*;
// use bevy::utils::all_tuples;
use bevy::utils::HashSet;
use impl_trait_for_tuples::*;

/// List of style objects which are attached to a given UiNode.
#[derive(Component, Default)]
pub struct ElementClasses(pub HashSet<String>);

impl ElementClasses {
    /// Add a classname to this element.
    pub fn add_class(&mut self, cls: &str) {
        self.0.insert(cls.to_string());
    }

    /// Remove a classname from this element.
    pub fn remove_class(&mut self, cls: &str) {
        self.0.remove(cls);
    }
}

// A wrapper view which applies styles to the output of an inner view.
pub struct ViewClasses<V: View> {
    inner: V,
    class_names: HashSet<String>,
}

impl<V: View> ViewClasses<V> {
    pub fn new<S: ClassNamesTuple>(inner: V, items: S) -> Self {
        Self {
            inner,
            class_names: items
                .to_vec()
                .iter()
                .filter_map(|cl| cl.to_owned())
                .collect(),
        }
    }

    fn set_class_names(&self, nodes: &NodeSpan, vc: &mut ViewContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut vc.entity_mut(*entity);
                match em.get_mut::<ElementClasses>() {
                    Some(mut ec) => {
                        ec.0.clone_from(&self.class_names);
                    }
                    None => {
                        em.insert((ElementClasses(self.class_names.clone()),));
                    }
                }
            }
            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    self.set_class_names(node, vc);
                }
            }
        }
    }
}

impl<V: View> View for ViewClasses<V> {
    type State = V::State;

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(vc, state)
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        let state = self.inner.build(vc);
        self.set_class_names(&self.nodes(vc, &state), vc);
        state
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.update(vc, state);
        self.set_class_names(&mut self.nodes(vc, state), vc);
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(vc, state)
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.raze(vc, state);
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

// A class name with an optional condition

trait ConditionalClassName: Send {
    /// Convert the conditional class name to an Option<String>.
    fn to_class(self) -> Option<String>;
}

struct NoClass;
impl ConditionalClassName for NoClass {
    fn to_class(self) -> Option<String> {
        None
    }
}

impl ConditionalClassName for String {
    fn to_class(self) -> Option<String> {
        Some(self)
    }
}

impl ConditionalClassName for &str {
    fn to_class(self) -> Option<String> {
        Some(self.to_owned())
    }
}

// Tuple of class names, possibly conditional

pub trait ClassNamesTuple: Send {
    fn to_vec(self) -> Vec<Option<String>>;
}

impl ClassNamesTuple for () {
    fn to_vec(self) -> Vec<Option<String>> {
        Vec::new()
    }
}

impl<S0: ConditionalClassName> ClassNamesTuple for S0 {
    fn to_vec(self) -> Vec<Option<String>> {
        vec![self.to_class()]
    }
}

// macro_rules! impl_class_names {
//     ($($T:ident),*) => {
//         impl<$($T: ConditionalClassName),*> ClassNamesTuple for ($($T,)*) {
//             fn to_vec(self) -> Vec<Option<String>> {
//                 Vec::from([$(self.$T.to_class()),* ])
//             }
//         }
//     };
// }

// all_tuples!(impl_class_names, 1, 16, S);

#[impl_for_tuples(1, 16)]
impl ClassNamesTuple for Tuple {
    for_tuples!( where #( Tuple: ConditionalClassName )* );

    fn to_vec(self) -> Vec<Option<String>> {
        Vec::from([for_tuples!( #( self.Tuple.to_class() ),* )])
    }
}
