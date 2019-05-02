use crate::data_types::Resource;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct GlobalResource(HashMap<TypeId, Rc<RefCell<Box<Resource>>>>);

impl GlobalResource {
    pub fn new() -> Self {
        GlobalResource(HashMap::new())
    }

    pub fn insert(&mut self, type_id: TypeId, resource: Rc<RefCell<Box<Resource>>>) {
        self.0.insert(type_id, resource);
    }

    pub fn get(&self, type_id: &TypeId) -> Option<&Rc<RefCell<Box<Resource>>>> {
        self.0.get(type_id)
    }

    pub fn get_mut(&mut self, type_id: &TypeId) -> Option<&mut Rc<RefCell<Box<Resource>>>> {
        self.0.get_mut(type_id)
    }

    pub fn clone_global_resource(&self, type_id: &TypeId) -> Rc<RefCell<Box<Resource>>> {
        Rc::clone(self.get(type_id).unwrap())
    }
}
