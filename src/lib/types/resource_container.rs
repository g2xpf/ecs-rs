use crate::data_types::Resource;
use crate::types::ComponentVector;
use crate::types::{MutCD, MutGR, CD, GR};
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::rc::Rc;

pub trait ResourceContainer<'a, T> {
    fn to_container(&'a self) -> T;
}

impl<'a, R> ResourceContainer<'a, MutGR<'a, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container(&'a self) -> MutGR<'a, R> {
        let borrow = self.borrow_mut();
        MutGR(RefMut::map(borrow, |borrow| {
            borrow.downcast_mut::<R>().unwrap()
        }))
    }
}

impl<'a, R> ResourceContainer<'a, GR<'a, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container(&'a self) -> GR<'a, R> {
        let borrow = self.borrow();
        GR(Ref::map(borrow, |borrow| {
            borrow.downcast_ref::<R>().unwrap()
        }))
    }
}

impl<'a, R> ResourceContainer<'a, MutCD<'a, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container(&'a self) -> MutCD<'a, R> {
        let borrow = self.borrow_mut();
        MutCD(RefMut::map(borrow, |borrow| {
            borrow.downcast_mut::<ComponentVector<R>>().unwrap()
        }))
    }
}

impl<'a, R> ResourceContainer<'a, CD<'a, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container(&'a self) -> CD<'a, R> {
        let borrow = self.borrow();
        CD(Ref::map(borrow, |borrow| {
            borrow.downcast_ref::<ComponentVector<R>>().unwrap()
        }))
    }
}
