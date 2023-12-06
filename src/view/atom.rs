use std::{any::Any, marker::PhantomData};

use bevy::ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query, SystemParam},
    world::World,
};

/// A unique key which can be used to read and write an atom.
#[derive(Copy, Clone)]
pub struct AtomHandle<T>
where
    T: Clone + Sync + Send + 'static,
{
    pub(crate) id: Entity,
    pub(crate) marker: PhantomData<T>,
}

#[derive(Component)]
#[doc(hidden)]
pub struct AtomCell(pub(crate) Box<dyn Any + Send + Sync + 'static>);

/// Methods for creating, reading and writing atoms.
pub trait AtomMethods {
    /// Create an [`AtomHandle`].
    ///
    /// If this is called on [`World`], then the caller is responsible for deleting the atom.
    /// If this is called on [`Cx`], then the atom is automatically deleted when the presenter
    /// is despawned.
    fn create_atom<T: Clone + Sync + Send + 'static>(&mut self) -> AtomHandle<T>;

    /// Read the value of an atom. Panics if the atom does not exist.
    fn get_atom<T: Clone + Sync + Send + 'static>(&self, handle: AtomHandle<T>) -> T;

    /// Write the value of an atom. Panics if the atom handle is invalid.
    fn set_atom<T: Clone + Sync + Send + 'static>(&mut self, handle: AtomHandle<T>, value: T);
}

impl AtomMethods for World {
    fn create_atom<T: Clone + Sync + Send + 'static>(&mut self) -> AtomHandle<T> {
        AtomHandle {
            id: self.spawn_empty().id(),
            marker: PhantomData,
        }
    }

    fn get_atom<T: Clone + Sync + Send + 'static>(&self, handle: AtomHandle<T>) -> T {
        let cell = self
            .entity(handle.id)
            .get::<AtomCell>()
            .expect("Atom does not exist");
        cell.0
            .as_ref()
            .downcast_ref::<T>()
            .expect("Atom is incorrect type")
            .clone()
    }

    fn set_atom<T: Clone + Sync + Send + 'static>(&mut self, handle: AtomHandle<T>, value: T) {
        let mut entt = self.entity_mut(handle.id);
        match entt.get_mut::<AtomCell>() {
            Some(mut cell) => *cell.0.downcast_mut::<T>().expect("Atom is incorrect type") = value,
            None => {
                entt.insert(AtomCell(Box::new(value)));
            }
        }
    }
}

/// An injectable parameter that allows reading and writing of atoms. Note that this is not
/// a reactive context, so reading atom values will not add the atom to a tracking context.
/// However, writing atom values will trigger reactions for other contexts which have
/// read the atom.
#[derive(SystemParam)]
pub struct AtomStore<'w, 's> {
    #[doc(hidden)]
    pub query: Query<'w, 's, &'static mut AtomCell>,
    #[doc(hidden)]
    pub commands: Commands<'w, 's>,
}

impl<'w, 's> AtomStore<'w, 's> {
    /// Read the value of an atom. Panics if the atom does not exist.
    pub fn get<T: Clone + Sync + Send + 'static>(&self, handle: AtomHandle<T>) -> T {
        let cell = self.query.get(handle.id).expect("Atom does not exist");
        cell.0
            .as_ref()
            .downcast_ref::<T>()
            .expect("Atom is incorrect type")
            .clone()
    }

    /// Write the value of an atom. Panics if the atom handle is invalid.
    pub fn set<T: Clone + Sync + Send + 'static>(&mut self, handle: AtomHandle<T>, value: T) {
        match self.query.get_mut(handle.id) {
            Ok(mut cell) => {
                *cell.0.downcast_mut::<T>().expect("Atom is incorrect type") = value;
            }
            _ => {
                self.commands
                    .entity(handle.id)
                    .insert(AtomCell(Box::new(value)));
            }
        }
    }

    /// Update the value of an atom. Panics if the atom does not exist.
    pub fn update<T: Clone + Sync + Send + 'static, F: FnOnce(T) -> T>(
        &mut self,
        handle: AtomHandle<T>,
        update: F,
    ) {
        let cell = self.query.get(handle.id).expect("Atom does not exist");
        let value = cell
            .0
            .as_ref()
            .downcast_ref::<T>()
            .expect("Atom is incorrect type")
            .clone();
        self.commands
            .entity(handle.id)
            .insert(AtomCell(Box::new(update(value))));
    }
}
