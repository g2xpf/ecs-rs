use crate::data_types::Resource;
use crate::types::{MutCD, MutGR, ResourceContainer, CD, GR};
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

pub trait InnerType<'a, 'b>: Sized
where
    Rc<RefCell<Box<Resource>>>: ResourceContainer<'a, 'b, Self>,
{
    type InnerType: Resource;

    #[inline]
    fn to_inner_type() -> TypeId {
        TypeId::of::<Self::InnerType>()
    }
}

impl<'a, 'b: 'a, R> InnerType<'a, 'b> for MutCD<'a, 'b, R>
where
    R: Resource,
{
    type InnerType = R;
}

impl<'a, 'b: 'a, R> InnerType<'a, 'b> for CD<'a, 'b, R>
where
    R: Resource,
{
    type InnerType = R;
}

impl<'a, 'b: 'a, R> InnerType<'a, 'b> for MutGR<'a, 'b, R>
where
    R: Resource,
{
    type InnerType = R;
}

impl<'a, 'b: 'a, R> InnerType<'a, 'b> for GR<'a, 'b, R>
where
    R: Resource,
{
    type InnerType = R;
}
