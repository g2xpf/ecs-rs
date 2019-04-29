use super::ComponentMask;
use crate::data_types::CachedVec;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Entity(pub CachedVec<ComponentMask>);

impl Entity {
    #[inline]
    pub fn new() -> Self {
        Entity(CachedVec::new())
    }

    #[inline]
    pub fn entry(&mut self) -> Option<(usize, &ComponentMask)> {
        self.0.entry()
    }

    #[inline]
    pub fn entry_mut(&mut self) -> Option<(usize, &mut ComponentMask)> {
        self.0.entry_mut()
    }
}

impl Deref for Entity {
    type Target = HashMap<usize, ComponentMask>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Entity {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
