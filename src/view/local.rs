use std::{
    any::Any,
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use bevy::ecs::component::Component;

#[derive(Clone)]
#[doc(hidden)]
pub struct LocalData<T: Send + Sync + 'static> {
    changed: Arc<AtomicBool>,
    data: Arc<Mutex<dyn Any + Send + Sync + 'static>>,
    marker: PhantomData<T>,
}

// TODO: I'd like to do Borrow, Deref, DerefMut etc., but this seems impossible given the mutex.

impl<T: Send + Sync + Clone + PartialEq + 'static> LocalData<T> {
    /// Get the value of the Local
    pub fn get(&self) -> T {
        let lock = self.data.lock().unwrap();
        let r = lock
            .downcast_ref::<T>()
            .expect("Mismatched type for LocalData");
        r.clone()
    }

    /// Set the value of the Local
    pub fn set(&mut self, value: T) {
        let mut lock = self.data.lock().unwrap();
        let r = lock
            .downcast_mut::<T>()
            .expect("Mismatched type for LocalData");
        if *r != value {
            self.changed.store(true, Ordering::Relaxed);
            *r = value.clone();
        }
    }

    /// Return whether the value has been changed
    pub fn is_changed(&self) -> bool {
        self.changed.load(Ordering::Relaxed)
    }
}

/// Tracks local vars used by each View tree entity.
#[derive(Component, Default)]
pub struct TrackedLocals {
    changed: Arc<AtomicBool>,
    locals: Vec<Arc<Mutex<dyn Any + Send + Sync>>>,
}

#[doc(hidden)]
impl TrackedLocals {
    pub fn get<T: Send + Sync + Clone>(
        &mut self,
        index: usize,
        init: impl FnOnce() -> T,
    ) -> LocalData<T> {
        if index < self.locals.len() {
            LocalData::<T> {
                data: self.locals[index].clone(),
                changed: self.changed.clone(),
                marker: PhantomData {},
            }
        } else {
            self.locals.push(Arc::new(Mutex::new(init())));
            LocalData::<T> {
                data: self.locals.last().unwrap().clone(),
                changed: self.changed.clone(),
                marker: PhantomData {},
            }
        }
    }

    /// Return whether this resource has been changed, and also atomically set it to unchanged.
    pub(crate) fn cas(&self) -> bool {
        self.changed.swap(false, Ordering::Relaxed)
    }

    /// Return whether this resource has been changed.
    pub fn is_changed(&self) -> bool {
        self.changed.load(Ordering::Relaxed)
    }

    /// Reset the 'changed' bit.
    pub fn reset_changed(&mut self) {
        self.changed.store(false, Ordering::Relaxed);
    }
}
