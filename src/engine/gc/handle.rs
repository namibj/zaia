use std::{cmp, fmt, hash};
use crate::util;

pub unsafe trait PtrTag {
    const PTR_TAG: usize;
}

pub struct Handle<T>
where
    T: ?Sized + PtrTag,
{
    ptr: *mut T,
}

impl<T> Handle<T>
where
    T: ?Sized + PtrTag,
{
    pub fn new(ptr: *mut T) -> Self {
        Handle { ptr }
    }

    pub unsafe fn get_unchecked<'a>(self) -> &'a T {
        &*self.ptr
    }

    pub unsafe fn get_unchecked_mut<'a>(self) -> &'a mut T {
        &mut *self.ptr
    }

    pub fn as_ptr(self) -> *mut T {
        self.ptr
    }

    pub fn tagged(self) -> TaggedHandle {
        TaggedHandle::new(self.ptr as *mut u8 as usize | T::PTR_TAG)
    }
}

impl<T> fmt::Debug for Handle<T>
where
    T: ?Sized + PtrTag,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Handle({:p})", self.ptr)
    }
}

impl<T> Clone for Handle<T>
where
    T: ?Sized + PtrTag,
{
    fn clone(&self) -> Self {
        Handle { ptr: self.ptr }
    }
}

impl<T> Copy for Handle<T> where T: ?Sized + PtrTag {}

impl<T> cmp::PartialEq for Handle<T>
where
    T: ?Sized + PtrTag,
{
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> cmp::Eq for Handle<T> where T: ?Sized + PtrTag {}

impl<T> hash::Hash for Handle<T>
where
    T: ?Sized + PtrTag,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

pub struct TaggedHandle {
    tagged: usize,
}

impl TaggedHandle {
    fn new(tagged: usize) -> Self {
        Self { tagged }
    }

    pub fn value(self) -> usize {
        self.tagged
    }

    pub fn hash(self) -> u64 {
        util::mix_u64(self.tagged as u64)
    }
}

impl fmt::Debug for TaggedHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TaggedHandle({:x})", self.tagged)
    }
}

impl Clone for TaggedHandle {
    fn clone(&self) -> Self {
        TaggedHandle {
            tagged: self.tagged,
        }
    }
}

impl Copy for TaggedHandle {}

impl cmp::PartialEq for TaggedHandle {
    fn eq(&self, other: &Self) -> bool {
        self.tagged == other.tagged
    }
}

impl cmp::Eq for TaggedHandle {}

impl hash::Hash for TaggedHandle {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.tagged.hash(state);
    }
}
