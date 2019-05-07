use super::{
    ComponentData, ComponentMask, GlobalResource, MutCD, MutGR, ResourceProvider, TypeMap, CD, GR,
};
use crate::data_types::Resource;
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ResourceBorrower<'a> {
    component_data: &'a ComponentData,
    global_resource: &'a GlobalResource,
    type_map: &'a TypeMap,
}

impl<'a> ResourceBorrower<'a> {
    #[inline]
    pub fn new(
        component_data: &'a ComponentData,
        global_resource: &'a GlobalResource,
        type_map: &'a TypeMap,
    ) -> Self {
        ResourceBorrower {
            component_data,
            global_resource,
            type_map,
        }
    }
}

impl<'a, 'b, R> ResourceProvider<MutGR<'a, 'b, R>> for ResourceBorrower<'a>
where
    R: Resource,
{
    #[inline]
    fn provide(&self) -> (Rc<RefCell<Box<Resource>>>, ComponentMask) {
        let type_id = TypeId::of::<R>();
        let mask = *self
            .type_map
            .get(&type_id)
            .unwrap_or_else(|err| panic!("The type was not registered: {:?}", err));
        (self.global_resource.clone_global_resource(&type_id), mask)
    }
}

impl<'a, 'b, R> ResourceProvider<GR<'a, 'b, R>> for ResourceBorrower<'a>
where
    R: Resource,
{
    #[inline]
    fn provide(&self) -> (Rc<RefCell<Box<Resource>>>, ComponentMask) {
        let type_id = TypeId::of::<R>();
        let mask = *self
            .type_map
            .get(&type_id)
            .unwrap_or_else(|err| panic!("The type was not registered: {:?}", err));
        (self.global_resource.clone_global_resource(&type_id), mask)
    }
}

impl<'a, 'b, R> ResourceProvider<MutCD<'a, 'b, R>> for ResourceBorrower<'a>
where
    R: Resource,
{
    #[inline]
    fn provide(&self) -> (Rc<RefCell<Box<Resource>>>, ComponentMask) {
        let type_id = TypeId::of::<R>();
        let mask = *self
            .type_map
            .get(&type_id)
            .unwrap_or_else(|err| panic!("The type was not registered: {:?}", err));
        (self.component_data.clone_component_data(&type_id), mask)
    }
}

impl<'a, 'b, R> ResourceProvider<CD<'a, 'b, R>> for ResourceBorrower<'a>
where
    R: Resource,
{
    #[inline]
    fn provide(&self) -> (Rc<RefCell<Box<Resource>>>, ComponentMask) {
        let type_id = TypeId::of::<R>();
        let mask = *self
            .type_map
            .get(&type_id)
            .unwrap_or_else(|err| panic!("The type was not registered: {:?}", err));
        (self.component_data.clone_component_data(&type_id), mask)
    }
}
