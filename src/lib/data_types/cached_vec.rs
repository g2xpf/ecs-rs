use super::stack::Stack;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct CachedVec<T> {
    key_max: usize,
    key_garbage: Stack<usize>,
    data: HashMap<usize, T>,
}

impl<T> CachedVec<T>
where
    T: Default,
{
    #[inline]
    pub fn new() -> Self {
        CachedVec {
            key_max: 0,
            key_garbage: Stack::new(),
            data: HashMap::new(),
        }
    }

    #[inline]
    pub fn entry(&mut self) -> Option<(usize, &T)> {
        if let Some(key) = self.key_garbage.pop() {
            self.try_entry(key)
        } else {
            let key = self.key_max;
            self.key_max += 1;
            self.try_entry(key)
        }
    }

    #[inline]
    pub fn entry_mut(&mut self) -> Option<(usize, &mut T)>
    where
        T: std::fmt::Debug,
    {
        if let Some(key) = self.key_garbage.pop() {
            self.try_entry_mut(key)
        } else {
            let key = self.key_max;
            self.key_max += 1;
            self.try_entry_mut(key)
        }
    }

    #[inline]
    fn try_entry(&mut self, key: usize) -> Option<(usize, &T)> {
        match self.data.entry(key) {
            Entry::Occupied(_) => None,
            Entry::Vacant(v) => Some((key, &*v.insert(Default::default()))),
        }
    }

    #[inline]
    fn try_entry_mut(&mut self, key: usize) -> Option<(usize, &mut T)> {
        match self.data.entry(key) {
            Entry::Occupied(_) => None,
            Entry::Vacant(v) => Some((key, v.insert(Default::default()))),
        }
    }
    #[inline]
    pub fn get(&self, key: &usize) -> Option<&T> {
        self.data.get(key)
    }

    #[inline]
    pub fn get_mut(&mut self, key: &usize) -> Option<&mut T> {
        self.data.get_mut(key)
    }

    #[inline]
    pub fn push(&mut self, value: T) -> usize {
        if let Some(key) = self.key_garbage.pop() {
            self.data.insert(key, value);
            key
        } else {
            let key = self.key_max;
            self.data.insert(key, value);
            self.key_max += 1;
            key
        }
    }

    #[inline]
    pub fn remove(&mut self, key: usize) -> Option<(usize, T)> {
        let ret = self.data.remove_entry(&key)?;
        self.key_garbage.push(key);
        Some(ret)
    }
}

impl<T> Deref for CachedVec<T> {
    type Target = HashMap<usize, T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for CachedVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
