use super::ComponentMask;
use std::any::TypeId;
use std::collections::HashMap;

pub type TypeMap = HashMap<TypeId, ComponentMask>;
