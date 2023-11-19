use std::marker::PhantomData;

use bevy::ecs::{component::Component, system::Resource, world::World};

pub trait AnyResource: Send + Sync {
    fn is_changed(&self, world: &World) -> bool;
}

#[derive(PartialEq, Eq)]
pub struct AnyRes<T> {
    pub pdata: PhantomData<T>,
}

impl<T> AnyRes<T> {
    pub(crate) fn new() -> Self {
        Self { pdata: PhantomData }
    }
}

impl<T> AnyResource for AnyRes<T>
where
    T: Resource,
{
    fn is_changed(&self, world: &World) -> bool {
        world.is_resource_changed::<T>()
    }
}

/// Tracks resources used by each View tree entity
#[derive(Component, Default)]
pub struct TrackedResources {
    pub data: Vec<Box<dyn AnyResource>>,
}
