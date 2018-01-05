use std::fmt;
use std::ptr::Unique;
use std::ops::{Deref, DerefMut};

// For some reason, the drop_in_place intrinsic is causing SIGTRAP on my machine
pub trait DropInPlace {
    unsafe fn drop_in_place(&mut self);
}

pub struct Owned<T: DropInPlace + ?Sized> {
    value: Unique<T>
}

impl<T: DropInPlace + ?Sized> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe {
            self.value.as_mut().drop_in_place();
        }
    }
}

impl<T: DropInPlace + ?Sized> Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            self.value.as_ref()
        }
    }
}

impl<T: DropInPlace + ?Sized> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            self.value.as_mut()
        }
    }
}

impl<T: DropInPlace + ?Sized + fmt::Debug> fmt::Debug for Owned<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        (&**self).fmt(f)
    }
}

impl<T: DropInPlace + ?Sized> Owned<T> {
    pub unsafe fn from_raw(ptr: *mut T) -> Owned<T> {
        Owned {
            value: Unique::new_unchecked(ptr)
        }
    }
}
