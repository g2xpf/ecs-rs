use super::ComponentMask;
use std::any::TypeId;
use std::collections::HashMap;

pub struct TypeMap(HashMap<TypeId, ComponentMask>);

impl TypeMap {
    #[inline]
    pub fn new() -> Self {
        TypeMap(HashMap::new())
    }

    #[inline]
    pub fn get(&self, type_id: &TypeId) -> &ComponentMask {
        self.0
            .get(type_id)
            .unwrap_or_else(|| panic!("Type dispatch failed"))
    }

    #[inline]
    pub fn get_mut(&mut self, type_id: &TypeId) -> &mut ComponentMask {
        self.0
            .get_mut(type_id)
            .unwrap_or_else(|| panic!("Type dispatch failed"))
    }

    #[inline]
    pub fn insert(&mut self, type_id: TypeId, component_mask: ComponentMask) {
        if let Some(_) = self.0.insert(type_id, component_mask) {
            panic!("The type is already registered");
        }
    }
}
