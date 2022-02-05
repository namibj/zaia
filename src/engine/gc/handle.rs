use std::{cmp, fmt, hash};

pub struct Handle<T> {
    ptr: *mut T,
}

impl<T> Handle<T> {
    pub fn new(ptr: *mut T) -> Self {
        Handle { ptr }
    }

    pub fn unmanaged(item: T) -> Self {
        Handle {
            ptr: Box::into_raw(Box::new(item)),
        }
    }

    pub unsafe fn get_unchecked<'a>(self) -> &'a T {
        &*self.ptr
    }

    pub unsafe fn get_unchecked_mut<'a>(self) -> &'a mut T {
        &mut *self.ptr
    }

    pub unsafe fn destroy(self) {
        Box::from_raw(self.ptr);
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Handle({:p})", self.ptr)
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle { ptr: self.ptr }
    }
}

impl<T> Copy for Handle<T> {}

impl<T> cmp::PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> cmp::Eq for Handle<T> {}

impl<T> hash::Hash for Handle<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}
