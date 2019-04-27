use crate::data_types::Resource;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub struct ComponentData(HashMap<TypeId, RefCell<Box<Resource>>>);

impl ComponentData {
    pub fn new() -> Self {
        ComponentData(HashMap::new())
    }
}

impl Deref for ComponentData {
    type Target = HashMap<TypeId, RefCell<Box<Resource>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ComponentData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
