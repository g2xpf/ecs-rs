use crate::data_types::Resource;
use crate::types::{ComponentMask, ComponentVector, Entity};
use std::cell::{Ref, RefMut};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct MutGR<'a, 'b, R> {
    component_data: RefMut<'a, R>,
    _marker: PhantomData<&'b ()>,
}
#[derive(Debug)]
pub struct GR<'a, 'b, R> {
    component_data: Ref<'a, R>,
    _marker: PhantomData<&'b ()>,
}
#[derive(Debug)]
pub struct MutCD<'a, 'b, R> {
    pub(crate) component_data: RefMut<'a, ComponentVector<R>>,
    pub(crate) component_mask: ComponentMask,
    pub(crate) entity: &'b Entity,
    pub(crate) data_ptr: *mut Option<Box<R>>,
}
#[derive(Debug)]
pub struct CD<'a, 'b, R> {
    pub(crate) component_data: Ref<'a, ComponentVector<R>>,
    pub(crate) component_mask: ComponentMask,
    pub(crate) entity: &'b Entity,
    pub(crate) data_ptr: *const Option<Box<R>>,
}

macro_rules! impl_gr_new {
    ($resource:ident, $type:ident, $component_data:ident, $target:ty) => {
        impl<'a, 'b, R> $type<'a, 'b, R>
        where
            R: Resource,
        {
            #[inline]
            pub fn new(component_data: $component_data<'a, $target>) -> Self {
                $type {
                    component_data,
                    _marker: PhantomData,
                }
            }
        }
    };
}

macro_rules! impl_cd_new {
    ($resource:ident, $type:ident, $component_data:ident, $target:ty, $ptr_fn:ident $(, $mut:tt)?) => {
        impl<'a, 'b, R> $type<'a, 'b, R>
        where
            R: Resource,
        {
            #[inline]
            pub fn new(
                $($mut)? component_data: $component_data<'a, $target>,
                component_mask: ComponentMask,
                entity: &'b Entity,
            ) -> Self {
                $type {
                    data_ptr: component_data.$ptr_fn(),
                    component_data,
                    component_mask,
                    entity,
                }
            }
        }
    };
}

impl_gr_new!(R, MutGR, RefMut, R);
impl_gr_new!(R, GR, Ref, R);
impl_cd_new!(R, MutCD, RefMut, ComponentVector<R>, as_mut_ptr, mut);
impl_cd_new!(R, CD, Ref, ComponentVector<R>, as_ptr);

impl<'a, 'b, R> MutCD<'a, 'b, R> where R: Resource {}

impl<'a, 'b, R> Deref for MutGR<'a, 'b, R>
where
    R: Resource,
{
    type Target = R;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.component_data
    }
}

impl<'a, 'b, R> DerefMut for MutGR<'a, 'b, R>
where
    R: Resource,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component_data
    }
}

impl<'a, 'b, R> Deref for GR<'a, 'b, R>
where
    R: Resource,
{
    type Target = R;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.component_data
    }
}

// impl<'a, 'b, R> DerefMut for GR<'a, 'b, R>
// where
//     R: Resource,
// {
//     #[inline]
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.component_data
//     }
// }

impl<'a, 'b, R> Deref for MutCD<'a, 'b, R>
where
    R: Resource,
{
    type Target = RefMut<'a, ComponentVector<R>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.component_data
    }
}

impl<'a, 'b, R> DerefMut for MutCD<'a, 'b, R>
where
    R: Resource,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component_data
    }
}

impl<'a, 'b, R> Deref for CD<'a, 'b, R>
where
    R: Resource,
{
    type Target = Ref<'a, ComponentVector<R>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.component_data
    }
}

impl<'a, 'b, R> DerefMut for CD<'a, 'b, R>
where
    R: Resource,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component_data
    }
}
