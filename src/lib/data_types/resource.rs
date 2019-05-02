use std::any::{Any, TypeId};

pub trait Resource: Any + 'static {
    fn type_id(&self) -> TypeId;
}

impl<T> Resource for T
where
    T: 'static + ?Sized,
{
    #[inline]
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

impl dyn Resource {
    #[inline]
    pub fn is<T>(&self) -> bool
    where
        T: Any,
    {
        TypeId::of::<T>() == Resource::type_id(self)
    }

    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Resource as *const T)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Resource as *mut T)) }
        } else {
            None
        }
    }
}
