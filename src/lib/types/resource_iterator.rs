use crate::data_types::Resource;
use crate::types::{ComponentMask, Entity, MutCD, CD};

pub trait IterableContainer<'b> {
    type Ptr;

    fn get_ptr(&self) -> Self::Ptr;
    fn get_entity(&self) -> &'b Entity;
    fn get_component_mask(&self) -> ComponentMask;
}

impl<'a, 'b, R> IterableContainer<'b> for MutCD<'a, 'b, R>
where
    R: Resource,
{
    type Ptr = *mut Option<Box<R>>;

    #[inline]
    fn get_ptr(&self) -> Self::Ptr {
        self.data_ptr.clone()
    }

    #[inline]
    fn get_entity(&self) -> &'b Entity {
        &self.entity
    }

    #[inline]
    fn get_component_mask(&self) -> ComponentMask {
        self.component_mask
    }
}
impl<'a, 'b, R> IterableContainer<'b> for CD<'a, 'b, R>
where
    R: Resource,
{
    type Ptr = *const Option<Box<R>>;

    #[inline]
    fn get_ptr(&self) -> Self::Ptr {
        self.data_ptr.clone()
    }

    #[inline]
    fn get_entity(&self) -> &'b Entity {
        &self.entity
    }

    #[inline]
    fn get_component_mask(&self) -> ComponentMask {
        self.component_mask
    }
}

macro_rules! impl_iterable_container {
    ($(($count:tt, $ty_param:ident)),+) => {
        impl<'a, 'b, $($ty_param),+> IterableContainer<'b> for ($($ty_param),+)
            where $($ty_param: IterableContainer<'b>),+ {
                type Ptr = ($($ty_param::Ptr),+);

                #[inline]
                fn get_ptr(&self) -> Self::Ptr {
                    ($(self.$count.get_ptr()),+)
                }

                #[inline]
                fn get_entity(&self) -> &'b Entity {
                    self.0.get_entity()
                }

                #[inline]
                fn get_component_mask(&self) -> ComponentMask {
                    impl_iterable_container!(expand_or; self; $($count),+)
                }
            }
    };

    (expand_or; $self:ident; $head_count:tt, $($tail_count:tt),+) => {
        impl_iterable_container!(expand_or; $self; $head_count) | impl_iterable_container!(expand_or; $self; $($tail_count),+);
    };

    (expand_or; $self:ident; $head_count:tt) => {
        $self.$head_count.get_component_mask()
    };
}

impl_iterable_container!((0, A), (1, B));
impl_iterable_container!((0, A), (1, B), (2, C));
impl_iterable_container!((0, A), (1, B), (2, C), (3, D));
impl_iterable_container!((0, A), (1, B), (2, C), (3, D), (4, E));
impl_iterable_container!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
impl_iterable_container!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
impl_iterable_container!(
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G),
    (7, H)
);
impl_iterable_container!(
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G),
    (7, H),
    (8, I)
);
impl_iterable_container!(
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G),
    (7, H),
    (8, I),
    (9, J)
);
