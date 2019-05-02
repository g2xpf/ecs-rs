use crate::data_types::Resource;
use crate::types::{ComponentMask, ComponentVector, Entity, MutCD, MutGR, CD, GR};
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::rc::Rc;

pub trait ResourceContainer<'a, 'b, T>
where
    Self: Sized,
{
    fn to_container(self_and_mask: &'a (Self, ComponentMask), entity: &'b Entity) -> T;
}

impl<'a, 'b: 'a, R> ResourceContainer<'a, 'b, MutGR<'a, 'b, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container(
        self_and_mask: &'a (Self, ComponentMask),
        _entity: &'b Entity,
    ) -> MutGR<'a, 'b, R> {
        let borrow = self_and_mask.0.borrow_mut();
        MutGR::new(RefMut::map(borrow, |borrow| {
            borrow.downcast_mut::<R>().unwrap()
        }))
    }
}

impl<'a, 'b: 'a, R> ResourceContainer<'a, 'b, GR<'a, 'b, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container(
        self_and_mask: &'a (Self, ComponentMask),
        _entity: &'b Entity,
    ) -> GR<'a, 'b, R> {
        let borrow = self_and_mask.0.borrow();
        GR::new(Ref::map(borrow, |borrow| {
            borrow.downcast_ref::<R>().unwrap()
        }))
    }
}

impl<'a, 'b: 'a, R> ResourceContainer<'a, 'b, MutCD<'a, 'b, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container(
        self_and_mask: &'a (Self, ComponentMask),
        entity: &'b Entity,
    ) -> MutCD<'a, 'b, R> {
        let borrow = self_and_mask.0.borrow_mut();
        MutCD::new(
            RefMut::map(borrow, |borrow| {
                borrow.downcast_mut::<ComponentVector<R>>().unwrap()
            }),
            self_and_mask.1,
            entity,
        )
    }
}

impl<'a, 'b: 'a, R> ResourceContainer<'a, 'b, CD<'a, 'b, R>> for Rc<RefCell<Box<Resource>>>
where
    R: Resource,
{
    #[inline]
    fn to_container(self_and_mask: &'a (Self, ComponentMask), entity: &'b Entity) -> CD<'a, 'b, R> {
        let borrow = self_and_mask.0.borrow();
        CD::new(
            Ref::map(borrow, |borrow| {
                borrow.downcast_ref::<ComponentVector<R>>().unwrap()
            }),
            self_and_mask.1,
            entity,
        )
    }
}
