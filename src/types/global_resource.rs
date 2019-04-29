use crate::data_types::Resource;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type GlobalResource = HashMap<TypeId, Rc<RefCell<Box<Resource>>>>;
