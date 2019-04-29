use crate::data_types::Resource;
use crate::types::{MutCD, MutGR, ResourceContainer, CD, GR};
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

pub trait InnerType<'a>: Sized
where
    Rc<RefCell<Box<Resource>>>: ResourceContainer<'a, Self>,
{
    type InnerType: Resource;
    fn to_inner_type() -> TypeId {
        TypeId::of::<Self::InnerType>()
    }
}

impl<'a, R> InnerType<'a> for MutCD<'a, R>
where
    R: Resource,
{
    type InnerType = R;
}

impl<'a, R> InnerType<'a> for CD<'a, R>
where
    R: Resource,
{
    type InnerType = R;
}

impl<'a, R> InnerType<'a> for MutGR<'a, R>
where
    R: Resource,
{
    type InnerType = R;
}

impl<'a, R> InnerType<'a> for GR<'a, R>
where
    R: Resource,
{
    type InnerType = R;
}
