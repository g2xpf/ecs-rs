use crate::data_types::Resource;
use crate::types::ComponentVector;
use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct ComponentData(HashMap<TypeId, Rc<RefCell<Box<Resource>>>>);

impl ComponentData {
    pub fn new() -> Self {
        ComponentData(HashMap::new())
    }

    pub fn clone_component_data(&self, type_id: &TypeId) -> Rc<RefCell<Box<Resource>>> {
        Rc::clone(self.0.get(type_id).unwrap())
    }

    pub fn to_ref<R: Resource>(&self) -> Ref<ComponentVector<R>> {
        let b = self.get(&TypeId::of::<R>()).unwrap().borrow();
        Ref::map(b, |b| b.downcast_ref::<ComponentVector<R>>().unwrap())
    }

    pub fn to_mut<R: Resource>(&mut self) -> RefMut<ComponentVector<R>> {
        let b = self.get_mut(&TypeId::of::<R>()).unwrap().borrow_mut();
        RefMut::map(b, |b| b.downcast_mut::<ComponentVector<R>>().unwrap())
    }
}

impl Deref for ComponentData {
    type Target = HashMap<TypeId, Rc<RefCell<Box<Resource>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ComponentData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
