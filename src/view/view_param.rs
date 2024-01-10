use std::sync::{Arc, Mutex};

use bevy::ecs::world::World;

use crate::node_span::NodeSpan;
use crate::{BuildContext, View};

/// A wrapper view around a view which makes it possible to pass a non-copyable view as a
/// parameter to other views.
///
/// Currently, this will cause the view to which this parameter is passed to unconditionally
/// render whenever it's parent renders, because we don't do proper equality comparisons.
pub struct ViewParam<V: View> {
    inner: Arc<Mutex<V>>,
}

impl<V: View> ViewParam<V> {
    /// Construct a new ViewParam that references the given View.
    pub fn new(inner: V) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

impl<V: View> View for ViewParam<V> {
    type State = V::State;

    fn nodes(&self, bc: &BuildContext, state: &Self::State) -> NodeSpan {
        self.inner.lock().unwrap().nodes(bc, state)
    }

    fn build(&self, bc: &mut BuildContext) -> Self::State {
        self.inner.lock().unwrap().build(bc)
    }

    fn update(&self, bc: &mut BuildContext, state: &mut Self::State) {
        self.inner.lock().unwrap().update(bc, state);
    }

    fn assemble(&self, bc: &mut BuildContext, state: &mut Self::State) -> NodeSpan {
        self.inner.lock().unwrap().assemble(bc, state)
    }

    fn raze(&self, world: &mut World, state: &mut Self::State) {
        self.inner.lock().unwrap().raze(world, state);
    }
}

impl<V: View> PartialEq for ViewParam<V> {
    fn eq(&self, other: &Self) -> bool {
        // For now, we're just comparing pointers. However, we should probably do better.
        std::ptr::eq(
            &*self.inner.lock().unwrap() as *const _,
            &*other.inner.lock().unwrap() as *const _,
        )
    }
}

impl<V: View> Clone for ViewParam<V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
