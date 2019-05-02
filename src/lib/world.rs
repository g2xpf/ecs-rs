use crate::data_types::Resource;
use crate::types::{
    ComponentData, ComponentMask, ComponentVector, Entity, EntityBuilder, GlobalResource,
    ResourceBorrower, System, SystemContainer, TypeMap,
};
use std::any::TypeId;
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::rc::Rc;

pub struct World {
    mask_counter: ComponentMask,
    type_map: TypeMap,
    entity: Entity,
    component_data: ComponentData,
    global_resource: GlobalResource,
    system: SystemContainer,
}

impl World {
    #[inline]
    pub fn new() -> Self {
        World {
            mask_counter: 1,
            type_map: TypeMap::new(),
            entity: Entity::new(),
            component_data: ComponentData::new(),
            global_resource: GlobalResource::new(),
            system: SystemContainer::new(),
        }
    }

    #[inline]
    pub fn register_component<R: Resource>(&mut self) {
        let type_id = TypeId::of::<R>();
        self.type_map.insert(type_id, self.mask_counter);
        self.component_data.insert(
            type_id,
            Rc::new(RefCell::new(Box::new(ComponentVector::<R>::new()))),
        );
        self.mask_counter <<= 1;
    }

    #[inline]
    pub fn push_global_resource<R: Resource>(&mut self, global_resource: R) {
        let type_id = TypeId::of::<R>();
        self.type_map.insert(type_id, self.mask_counter);
        self.global_resource
            .insert(type_id, Rc::new(RefCell::new(Box::new(global_resource))));
        self.mask_counter <<= 1;
    }

    #[inline]
    pub fn entry_entity(&mut self) -> EntityBuilder {
        let (key, component_mask) = self.entity.entry_mut().unwrap();
        EntityBuilder::new(
            key,
            component_mask,
            &mut self.type_map,
            &mut self.component_data,
        )
    }

    #[inline]
    pub fn get_component_data_ref<R: Resource>(&self) -> Ref<ComponentVector<R>> {
        self.component_data.to_ref()
    }

    #[inline]
    pub fn get_component_data_mut<R: Resource>(&mut self) -> RefMut<ComponentVector<R>> {
        self.component_data.to_mut()
    }

    #[inline]
    pub fn get_global_resource_ref<R: Resource>(&self) -> Ref<R> {
        let b = self
            .global_resource
            .get(&TypeId::of::<R>())
            .unwrap()
            .borrow();
        Ref::map(b, |b| b.downcast_ref::<R>().unwrap())
    }

    #[inline]
    pub fn get_global_resource_mut<R: Resource>(&mut self) -> RefMut<R> {
        let b = self
            .global_resource
            .get_mut(&TypeId::of::<R>())
            .unwrap()
            .borrow_mut();
        RefMut::map(b, |b| b.downcast_mut::<R>().unwrap())
    }

    #[inline]
    pub fn get_entity_ref(&self) -> &Entity {
        &self.entity
    }

    #[inline]
    pub fn get_type_map(&self) -> &TypeMap {
        &self.type_map
    }

    #[inline]
    fn make_resource_borrower<'a>(&'a self) -> ResourceBorrower<'a> {
        ResourceBorrower::new(&self.component_data, &self.global_resource, &self.type_map)
    }

    #[inline]
    pub fn register_system<S: 'static>(&mut self)
    where
        S: System,
    {
        let resource_borrower = self.make_resource_borrower();
        let system = S::new(&resource_borrower);
        self.system.register(system);
    }

    #[inline]
    pub fn run(&mut self) {
        loop {
            self.system.run(&self.entity);
        }
    }
}
