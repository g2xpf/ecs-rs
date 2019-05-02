use super::ComponentMask;
use crate::data_types::Resource;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ResourceProvider<T> {
    fn provide(&self) -> (Rc<RefCell<Box<Resource>>>, ComponentMask);
}
