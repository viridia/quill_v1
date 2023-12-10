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
    pub fn new<'a, S: ClassNamesTuple<'a>>(inner: V, items: S) -> Self {
        Self {
            inner,
            class_names: items.to_set(),
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

/// A class name with an optional condition
pub trait ClassName: Send + Sized {
    /// Convert the conditional class name to an Option<String>.
    fn to_class(&self) -> Option<&str>;

    /// Method which allows the class name to be added conditionally.
    fn if_true(&self, condition: bool) -> ClassNameWithCondition<Self> {
        ClassNameWithCondition {
            inner: &self,
            enabled: condition,
        }
    }
}

pub struct ClassNameWithCondition<'a, C: ClassName> {
    pub(crate) inner: &'a C,
    pub(crate) enabled: bool,
}

impl<'a, C: ClassName + Sync> ClassName for ClassNameWithCondition<'a, C> {
    fn to_class(&self) -> Option<&str> {
        if self.enabled {
            self.inner.to_class()
        } else {
            None
        }
    }

    fn if_true(&self, condition: bool) -> ClassNameWithCondition<Self> {
        ClassNameWithCondition {
            inner: &self,
            enabled: self.enabled && condition,
        }
    }
}

impl ClassName for String {
    fn to_class(&self) -> Option<&str> {
        Some(self)
    }
}

impl ClassName for &str {
    fn to_class(&self) -> Option<&str> {
        Some(self)
    }
}

// Tuple of class names, possibly conditional

pub trait ClassNamesTuple<'a>: Send {
    fn to_set(self) -> HashSet<String>;
}

impl<'a> ClassNamesTuple<'a> for () {
    fn to_set(self) -> HashSet<String> {
        HashSet::new()
    }
}

impl<'a, S0: ClassName> ClassNamesTuple<'a> for S0 {
    fn to_set(self) -> HashSet<String> {
        match self.to_class() {
            Some(cls) => [cls.to_owned()].into(),
            None => HashSet::new(),
        }
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
impl<'a> ClassNamesTuple<'a> for Tuple {
    for_tuples!( where #( Tuple: ClassName )* );

    fn to_set(self) -> HashSet<String> {
        let mut result = HashSet::<String>::new();
        for_tuples!( #(
            if let Some(cls) = self.Tuple.to_class() {
                result.insert(cls.to_owned());
            } )* );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_names<'a, CN: ClassNamesTuple<'a>>(class_names: CN) -> HashSet<String> {
        class_names.to_set()
    }

    #[test]
    fn test_class_names() {
        let cl = get_names(());
        assert_eq!(cl, HashSet::new());

        let cl = get_names("test");
        assert_eq!(cl, ["test".to_owned()].into());

        let cl = get_names(("one", "two"));
        assert_eq!(cl, ["one".to_owned(), "two".to_owned()].into());

        let cl = get_names(("one".if_true(true), "two"));
        assert_eq!(cl, ["one".to_owned(), "two".to_owned()].into());

        let cl = get_names(("one".if_true(false), "two"));
        assert_eq!(cl, ["two".to_owned()].into());

        let cl = get_names(("one".if_true(true).if_true(false), "two"));
        assert_eq!(cl, ["two".to_owned()].into());
    }
}
