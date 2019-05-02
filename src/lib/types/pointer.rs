pub trait Pointer<'a, T> {
    type Target;
    fn get_ref(&self, count: usize) -> Self::Target;
}

impl<'a, T: 'a> Pointer<'a, *mut Option<Box<T>>> for *mut Option<Box<T>> {
    type Target = &'a mut Box<T>;

    #[inline]
    fn get_ref(&self, count: usize) -> Self::Target {
        unsafe { (&mut *self.add(count)).as_mut().unwrap() }
    }
}

impl<'a, T: 'a> Pointer<'a, *const Option<Box<T>>> for *const Option<Box<T>> {
    type Target = &'a Box<T>;

    #[inline]
    fn get_ref(&self, count: usize) -> Self::Target {
        unsafe { (&*self.add(count)).as_ref().unwrap() }
    }
}

macro_rules! impl_pointer {
    ($(($count:tt, $ty_param:ident)),+) => {
        impl <'a, $($ty_param),+> Pointer<'a, ($($ty_param),+)> for ($($ty_param),+)
        where $($ty_param: Pointer<'a, $ty_param>),+ {
            type Target = ($($ty_param::Target),+);

            #[inline]
            fn get_ref(&self, count: usize) -> Self::Target {
                ($(self.$count.get_ref(count)),+)
            }
        }
    }
}

impl_pointer!((0, A), (1, B));
impl_pointer!((0, A), (1, B), (2, C));
impl_pointer!((0, A), (1, B), (2, C), (3, D));
impl_pointer!((0, A), (1, B), (2, C), (3, D), (4, E));
impl_pointer!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
impl_pointer!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
impl_pointer!(
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G),
    (7, H)
);
impl_pointer!(
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
impl_pointer!(
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
