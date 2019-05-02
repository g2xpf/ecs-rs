use crate::types::Pointer;
use std::marker::PhantomData;

pub struct ComponentDispatcher<'a, I, T> {
    indices: I,
    value: T,
    _marker: PhantomData<&'a ()>,
}

impl<'a, 'b, I, T> ComponentDispatcher<'a, I, T>
where
    I: Iterator<Item = &'b usize>,
    T: Pointer<'a, T>,
{
    #[inline]
    pub fn new(indices: I, value: T) -> Self {
        ComponentDispatcher {
            indices,
            value,
            _marker: PhantomData,
        }
    }
}

impl<'a, 'b, I, T: 'a> Iterator for ComponentDispatcher<'a, I, T>
where
    I: Iterator<Item = &'b usize>,
    T: Pointer<'a, T>,
{
    type Item = <T as Pointer<'a, T>>::Target;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indices.next()?;
        Some(self.value.get_ref(*index))
    }
}
