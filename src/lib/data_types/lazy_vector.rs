use std::fmt;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct LazyVector<T>(pub Vec<Option<Box<T>>>);

impl<T> Debug for LazyVector<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T> Deref for LazyVector<T> {
    type Target = Vec<Option<Box<T>>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R> DerefMut for LazyVector<R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> LazyVector<T> {
    #[inline]
    pub fn new() -> Self {
        LazyVector(Vec::new())
    }

    #[inline]
    pub fn insert(&mut self, t: T, index: usize) {
        let len = self.0.len();

        if len <= index {
            // For power-of-two length
            let new_len = (index + 1).next_power_of_two();
            self.0.resize_with(new_len, Default::default);
        }

        self.0[index] = Some(Box::new(t));
    }

    #[inline]
    pub fn remove(&mut self, index: usize) {
        self.0[index] = None;
    }
}
