use crate::data_types::Resource;
use crate::types::{ComponentData, ComponentMask, ComponentVector, Entity, EntityBuilder, TypeMap};
use std::any::TypeId;
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::collections::HashMap;

pub struct World {
    mask_counter: ComponentMask,
    type_map: TypeMap,
    entity: Entity,
    component_data: ComponentData,
    global_resource: HashMap<TypeId, RefCell<Box<Resource>>>,
}

impl World {
    pub fn new() -> Self {
        World {
            mask_counter: 1,
            type_map: HashMap::new(),
            entity: Entity::new(),
            component_data: ComponentData::new(),
            global_resource: HashMap::new(),
        }
    }

    pub fn register_component<R: Resource>(&mut self) {
        let type_id = TypeId::of::<R>();
        self.type_map.insert(type_id, self.mask_counter);
        self.component_data
            .insert(type_id, RefCell::new(Box::new(ComponentVector::<R>::new())));
        self.mask_counter <<= 1;
    }

    pub fn push_global_resource<R: Resource>(&mut self, global_resource: R) {
        let type_id = TypeId::of::<R>();
        self.type_map.insert(type_id, self.mask_counter);
        self.global_resource
            .insert(type_id, RefCell::new(Box::new(global_resource)));
        self.mask_counter <<= 1;
    }

    pub fn entry_entity(&mut self) -> EntityBuilder {
        let (key, component_mask) = self.entity.entry_mut().unwrap();
        EntityBuilder::new(
            key,
            component_mask,
            &mut self.type_map,
            &mut self.component_data,
        )
    }

    pub fn get_component_data_ref<R: Resource>(&self) -> Ref<ComponentVector<R>> {
        let b = self
            .component_data
            .get(&TypeId::of::<R>())
            .unwrap()
            .borrow();
        Ref::map(b, |b| b.downcast_ref::<ComponentVector<R>>().unwrap())
    }

    pub fn get_component_data_mut<R: Resource>(&mut self) -> RefMut<ComponentVector<R>> {
        let b = self
            .component_data
            .get_mut(&TypeId::of::<R>())
            .unwrap()
            .borrow_mut();
        RefMut::map(b, |b| b.downcast_mut::<ComponentVector<R>>().unwrap())
    }

    pub fn get_global_resource_ref<R: Resource>(&self) -> Ref<R> {
        let b = self
            .global_resource
            .get(&TypeId::of::<R>())
            .unwrap()
            .borrow();
        Ref::map(b, |b| b.downcast_ref::<R>().unwrap())
    }

    pub fn get_global_resource_mut<R: Resource>(&mut self) -> RefMut<R> {
        let b = self
            .global_resource
            .get(&TypeId::of::<R>())
            .unwrap()
            .borrow_mut();
        RefMut::map(b, |b| b.downcast_mut::<R>().unwrap())
    }

    pub fn get_entity_ref(&self) -> &Entity {
        &self.entity
    }

    pub fn get_type_map(&self) -> &TypeMap {
        &self.type_map
    }
}
