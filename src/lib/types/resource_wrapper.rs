use crate::data_types::Resource;
use crate::types::{ComponentMask, ComponentVector};
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct MutGR<'a, R> {
    pub component_data: RefMut<'a, R>,
}
#[derive(Debug)]
pub struct GR<'a, R> {
    pub component_data: Ref<'a, R>,
}
#[derive(Debug)]
pub struct MutCD<'a, R> {
    pub component_data: RefMut<'a, ComponentVector<R>>,
    pub component_mask: ComponentMask,
}
#[derive(Debug)]
pub struct CD<'a, R> {
    pub component_data: Ref<'a, ComponentVector<R>>,
    pub component_mask: ComponentMask,
}

macro_rules! impl_gr_new {
    ($resource:ident, $type:ident, $component_data:ident, $target:ty) => {
        impl<'a, R> $type<'a, R>
        where
            R: Resource,
        {
            pub fn new(component_data: $component_data<'a, $target>) -> Self {
                $type { component_data }
            }
        }
    };
}

macro_rules! impl_cd_new {
    ($resource:ident, $type:ident, $component_data:ident, $target:ty) => {
        impl<'a, R> $type<'a, R>
        where
            R: Resource,
        {
            pub fn new(
                component_data: $component_data<'a, $target>,
                component_mask: ComponentMask,
            ) -> Self {
                $type {
                    component_data,
                    component_mask,
                }
            }
        }
    };
}

impl_gr_new!(R, MutGR, RefMut, R);
impl_gr_new!(R, GR, Ref, R);
impl_cd_new!(R, MutCD, RefMut, ComponentVector<R>);
impl_cd_new!(R, CD, Ref, ComponentVector<R>);

impl<'a, R> Deref for MutGR<'a, R>
where
    R: Resource,
{
    type Target = RefMut<'a, R>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.component_data
    }
}

impl<'a, R> DerefMut for MutGR<'a, R>
where
    R: Resource,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component_data
    }
}

impl<'a, R> Deref for GR<'a, R>
where
    R: Resource,
{
    type Target = Ref<'a, R>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.component_data
    }
}

impl<'a, R> DerefMut for GR<'a, R>
where
    R: Resource,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component_data
    }
}

impl<'a, R> Deref for MutCD<'a, R>
where
    R: Resource,
{
    type Target = RefMut<'a, ComponentVector<R>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.component_data
    }
}

impl<'a, R> DerefMut for MutCD<'a, R>
where
    R: Resource,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component_data
    }
}

impl<'a, R> Deref for CD<'a, R>
where
    R: Resource,
{
    type Target = Ref<'a, ComponentVector<R>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.component_data
    }
}

impl<'a, R> DerefMut for CD<'a, R>
where
    R: Resource,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component_data
    }
}
