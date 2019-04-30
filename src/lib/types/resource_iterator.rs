use std::marker::PhantomData;

pub struct ResourceIterMut<'a, I, T> {
    indices: I,
    ptr: *mut T,
    _marker: PhantomData,
}

pub struct ResourceIter<'a, I, T> {
    indices: I,
    ptr: *const T,
    _marker: PhantomData,
}

impl ResourceIterMut<'a, I, T>
where
    I: Iterator<Item = &usize>,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = indices.next()?;
        unsafe { Some(&mut *self.ptr.add(index)) }
    }
}

impl ResourceIter<'a, I, T>
where
    I: Iterator<Item = &usize>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = indices.next()?;
        unsafe { Some(&*self.ptr.add(index)) }
    }
}

// use crate::data_types::Resource;
// use crate::types::{ComponentMask, Entity, MutCD, CD};
//
// pub trait IterGenerator<'b> {
//     type Ptr;
//     fn get_ptr(&self) -> Self::Ptr;
//     fn get_entity(&self) -> &'b Entity;
//     fn get_component_mask(&self) -> ComponentMask;
//     fn generate(&self) -> (&'b Entity, ComponentMask, Self::Ptr) {
//         (self.get_entity(), self.get_component_mask(), self.get_ptr())
//     }
// }
//
// impl<'a, 'b, R> IterGenerator<'b> for MutCD<'a, 'b, R>
// where
//     R: Resource,
// {
//     type Ptr = *mut Option<Box<R>>;
//
//     fn get_ptr(&self) -> Self::Ptr {
//         self.data_ptr.clone()
//     }
//     fn get_entity(&self) -> &'b Entity {
//         self.entity
//     }
//     fn get_component_mask(&self) -> ComponentMask {
//         self.component_mask
//     }
// }
//
// impl<'a, 'b, R> IterGenerator<'b> for CD<'a, 'b, R>
// where
//     R: Resource,
// {
//     type Ptr = *const Option<Box<R>>;
//
//     fn get_ptr(&self) -> Self::Ptr {
//         self.data_ptr.clone()
//     }
//     fn get_entity(&self) -> &'b Entity {
//         self.entity
//     }
//     fn get_component_mask(&self) -> ComponentMask {
//         self.component_mask
//     }
// }
//
// macro_rules! impl_iter_generator {
//     ($(($count:tt, $ty_param:ident)),+) => {
//         impl<'a, 'b, $($ty_param),+> IterGenerator<'b> for ($($ty_param),+)
//             where $($ty_param: IterGenerator<'b>),+ {
//                 type Ptr = ($($ty_param::Ptr),+);
//
//                 fn get_ptr(&self) -> Self::Ptr {
//                     ($(self.$count.get_ptr()),+)
//                 }
//
//                 fn get_entity(&self) -> &'b Entity {
//                     self.0.get_entity()
//                 }
//
//                 fn get_component_mask(&self) -> ComponentMask {
//                     impl_iter_generator!(expand_or; self; $($count),+)
//                 }
//             }
//     };
//
//     (expand_or; $self:ident; $head_count:tt, $($tail_count:tt),+) => {
//         impl_iter_generator!(expand_or; $self; $head_count) | impl_iter_generator!(expand_or; $self; $($tail_count),+);
//     };
//
//     (expand_or; $self:ident; $head_count:tt) => {
//         $self.$head_count.get_component_mask()
//     };
// }
//
// impl_iter_generator!((0, A), (1, B));
// impl_iter_generator!((0, A), (1, B), (2, C));
// impl_iter_generator!((0, A), (1, B), (2, C), (3, D));
// impl_iter_generator!((0, A), (1, B), (2, C), (3, D), (4, E));
// impl_iter_generator!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
// impl_iter_generator!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
// impl_iter_generator!(
//     (0, A),
//     (1, B),
//     (2, C),
//     (3, D),
//     (4, E),
//     (5, F),
//     (6, G),
//     (7, H)
// );
// impl_iter_generator!(
//     (0, A),
//     (1, B),
//     (2, C),
//     (3, D),
//     (4, E),
//     (5, F),
//     (6, G),
//     (7, H),
//     (8, I)
// );
// impl_iter_generator!(
//     (0, A),
//     (1, B),
//     (2, C),
//     (3, D),
//     (4, E),
//     (5, F),
//     (6, G),
//     (7, H),
//     (8, I),
//     (9, J)
// );
