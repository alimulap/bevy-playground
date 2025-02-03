use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Resource)]
pub struct ObjectPool<T>
where
    T: Send + Sync + 'static,
{
    pool: Vec<Entity>,
    _marker: PhantomData<T>,
}

impl<T> ObjectPool<T>
where
    T: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn get(&mut self) -> Option<Entity> {
        self.pool.pop()
    }

    pub fn put(&mut self, item: Entity) {
        self.pool.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }
}

pub fn pool_empty<T: Send + Sync + 'static>(pool: Res<ObjectPool<T>>) -> bool {
    pool.is_empty()
}
