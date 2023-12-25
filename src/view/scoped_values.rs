use bevy::prelude::*;
use bevy::utils::HashMap;
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;

/// A unique key for accessing a variable whose scope is the current presenter invocation
/// and any nested presenters.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ScopedValueKey<T: Clone> {
    name: &'static str,
    marker: PhantomData<T>,
}

impl<T: Clone> Debug for ScopedValueKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("#{}", self.name))
    }
}

impl<T: Clone> ScopedValueKey<T> {
    /// Construct a new variable token given a name.
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            marker: PhantomData,
        }
    }

    /// Return a unique id for this token.
    pub fn id(&self) -> &'static str {
        self.name
    }
}

/// Component used to store context variables.
#[derive(Component, Default)]
#[doc(hidden)]
pub struct ScopedValueMap(pub(crate) HashMap<&'static str, Box<dyn Any + Send + Sync + 'static>>);
