use crate::data_types::{LazyVector, Resource, ResourceContainer};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ComponentVector<R>(pub LazyVector<R>);

impl<R> ComponentVector<R> {
    pub fn new() -> Self {
        ComponentVector(LazyVector::new())
    }
}

impl<R> Deref for ComponentVector<R> {
    type Target = LazyVector<R>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R> DerefMut for ComponentVector<R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R> ResourceContainer for ComponentVector<R>
where
    R: Resource,
{
    type Target = Option<Box<R>>;

    #[inline]
    fn get(&self, index: usize) -> &Self::Target {
        &(***self)[index]
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> &mut Self::Target {
        &mut (***self)[index]
    }
}
