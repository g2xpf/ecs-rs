use crate::data_types::Resource;
use crate::types::{ComponentData, ComponentMask, ComponentVector, TypeMap};
use std::any::TypeId;

pub struct EntityBuilder<'a> {
    key: usize,
    component_mask: &'a mut ComponentMask,
    type_map: &'a mut TypeMap,
    component_data: &'a mut ComponentData,
}

impl<'a> EntityBuilder<'a> {
    #[inline]
    pub fn new(
        key: usize,
        component_mask: &'a mut ComponentMask,
        type_map: &'a mut TypeMap,
        component_data: &'a mut ComponentData,
    ) -> Self {
        EntityBuilder {
            key,
            component_mask,
            type_map,
            component_data,
        }
    }

    #[inline]
    pub fn push<R: Resource>(&mut self, r: R) -> &mut Self {
        let type_id = TypeId::of::<R>();
        let new_mask = self.type_map.get(&type_id).unwrap();
        *self.component_mask |= new_mask;
        {
            let mut component_data = self.component_data.get_mut(&type_id).unwrap().borrow_mut();
            let v = component_data.downcast_mut::<ComponentVector<R>>().unwrap();
            v.insert(r, self.key);
        }
        self
    }
}
