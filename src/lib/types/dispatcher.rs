use crate::types::{ComponentDispatcher, IterableContainer, Pointer};
use std::vec::IntoIter;

pub trait Dispatcher<'a, I, T> {
    fn dispatch(&self) -> ComponentDispatcher<'a, I, T>;
}

impl<'a, 'b, T> Dispatcher<'a, IntoIter<&'b usize>, T::Ptr> for T
where
    T: IterableContainer<'b>,
    <T as IterableContainer<'b>>::Ptr: Pointer<'a, <T as IterableContainer<'b>>::Ptr>,
{
    #[inline]
    fn dispatch(&self) -> ComponentDispatcher<'a, IntoIter<&'b usize>, T::Ptr> {
        let mask = self.get_component_mask();
        let v: Vec<_> = self
            .get_entity()
            .iter()
            .filter_map(|(ent_index, ent_mask)| {
                if ent_mask & mask == mask {
                    Some(ent_index)
                } else {
                    None
                }
            })
            .collect();
        ComponentDispatcher::new(v.into_iter(), self.get_ptr())
    }
}
