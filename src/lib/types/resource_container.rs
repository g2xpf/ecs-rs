use crate::data_types::Resource;
use crate::types::{ComponentMask, ComponentVector, Entity, MutCD, MutGR, CD, GR};
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::rc::Rc;

pub trait ResourceContainer<'a, T> {
    fn to_container<'b>(self_and_mask: &(&'a Self, ComponentMask), entity: &'b Entity) -> T;
}

impl<'a, R> ResourceContainer<'a, MutGR<'a, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container<'b>(
        self_and_mask: &(&'a Self, ComponentMask),
        entity: &'b Entity,
    ) -> MutGR<'a, R> {
        let borrow = self_and_mask.0.borrow_mut();
        MutGR::new(RefMut::map(borrow, |borrow| {
            borrow.downcast_mut::<R>().unwrap()
        }))
    }
}

impl<'a, R> ResourceContainer<'a, GR<'a, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container<'b>(
        self_and_mask: &(&'a Self, ComponentMask),
        entity: &'b Entity,
    ) -> GR<'a, R> {
        let borrow = self_and_mask.0.borrow();
        GR::new(Ref::map(borrow, |borrow| {
            borrow.downcast_ref::<R>().unwrap()
        }))
    }
}

impl<'a, R> ResourceContainer<'a, MutCD<'a, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container<'b>(
        self_and_mask: &(&'a Self, ComponentMask),
        entity: &'b Entity,
    ) -> MutCD<'a, R> {
        let borrow = self_and_mask.0.borrow_mut();
        MutCD::new(
            RefMut::map(borrow, |borrow| {
                borrow.downcast_mut::<ComponentVector<R>>().unwrap()
            }),
            self_and_mask.1,
        )
    }
}

impl<'a, R> ResourceContainer<'a, CD<'a, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container<'b>(
        self_and_mask: &(&'a Self, ComponentMask),
        entity: &'b Entity,
    ) -> CD<'a, R> {
        let borrow = self_and_mask.0.borrow();
        CD::new(
            Ref::map(borrow, |borrow| {
                borrow.downcast_ref::<ComponentVector<R>>().unwrap()
            }),
            self_and_mask.1,
        )
    }
}
