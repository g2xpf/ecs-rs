use crate::data_types::LazyVector;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ComponentVector<R>(pub LazyVector<R>);

impl<R> ComponentVector<R> {
    pub fn new() -> Self {
        ComponentVector(LazyVector::new())
    }

    #[inline]
    fn get(&self, index: usize) -> &Option<Box<R>> {
        &(***self)[index]
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> &mut Option<Box<R>> {
        &mut (***self)[index]
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
