use std::{cmp, fmt, hash};

use super::super::util;

/// # Safety
/// `PtrTag` must be implemented directly using the functions
/// found in the `encoding` submodule.
pub unsafe trait PtrTag {
    fn is(x: u64) -> bool;
    fn tag(x: usize) -> u64;
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

    /// # Safety
    /// - Handle must point to a living instance of `T`.
    pub unsafe fn get_unchecked<'a>(self) -> &'a T {
        &*self.ptr
    }

    /// # Safety
    /// - Handle must point to a living instance of `T`.
    pub unsafe fn get_unchecked_mut<'a>(self) -> &'a mut T {
        &mut *self.ptr
    }

    pub fn as_ptr(self) -> *mut T {
        self.ptr
    }

    pub fn tagged(self) -> TaggedHandle {
        let tagged = T::tag(self.ptr as *mut u8 as usize);
        TaggedHandle::new(tagged)
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
    tagged: u64,
}

impl TaggedHandle {
    pub fn new(tagged: u64) -> Self {
        Self { tagged }
    }

    pub fn value(self) -> u64 {
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
