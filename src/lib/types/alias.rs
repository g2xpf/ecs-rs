use crate::data_types::Resource;
use crate::types::ComponentVector;
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct MutGR<'a, R>(pub RefMut<'a, R>);
#[derive(Debug)]
pub struct GR<'a, R>(pub Ref<'a, R>);
#[derive(Debug)]
pub struct MutCD<'a, R>(pub RefMut<'a, ComponentVector<R>>);
#[derive(Debug)]
pub struct CD<'a, R>(pub Ref<'a, ComponentVector<R>>);

impl<'a, R> Deref for MutGR<'a, R>
where
    R: Resource,
{
    type Target = RefMut<'a, R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, R> DerefMut for MutGR<'a, R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, R> Deref for GR<'a, R>
where
    R: Resource,
{
    type Target = Ref<'a, R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, R> DerefMut for GR<'a, R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, R> Deref for MutCD<'a, R>
where
    R: Resource,
{
    type Target = RefMut<'a, ComponentVector<R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, R> DerefMut for MutCD<'a, R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, R> Deref for CD<'a, R>
where
    R: Resource,
{
    type Target = Ref<'a, ComponentVector<R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, R> DerefMut for CD<'a, R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
