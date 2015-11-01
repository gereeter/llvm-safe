use std::ptr::Unique;
use std::ops::{Deref, DerefMut};

// For some reason, the drop_in_place intrinsic is causing SIGTRAP on my machine
pub trait DropInPlace {
    unsafe fn drop_in_place(&mut self);
}

pub struct Owned<T: DropInPlace> {
    value: Unique<T>
}

impl<T: DropInPlace> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe {
            self.value.get_mut().drop_in_place();
        }
    }
}

impl<T: DropInPlace> Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            self.value.get()
        }
    }
}

impl<T: DropInPlace> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            self.value.get_mut()
        }
    }
}

impl<T: DropInPlace> Owned<T> {
    pub unsafe fn from_raw(ptr: *mut T) -> Owned<T> {
        Owned {
            value: Unique::new(ptr)
        }
    }
}
