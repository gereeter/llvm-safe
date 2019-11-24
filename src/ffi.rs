use std::ffi::CStr;
use std::ops::Deref;

use libc::{self, c_char, c_void};

use owned::{Owned, DropInPlace};

#[repr(transparent)]
#[derive(Debug)]
pub struct MallocCStr {
    inner: CStr
}

impl DropInPlace for MallocCStr {
    unsafe fn drop_in_place(&mut self) {
        libc::free(&mut self.inner as *mut _ as *mut c_void);
    }
}

impl Deref for MallocCStr {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        &self.inner
    }
}

impl MallocCStr {
    pub unsafe fn from_raw(ptr: *const c_char) -> Owned<MallocCStr> {
        Owned::from_raw(CStr::from_ptr(ptr) as *const CStr as *mut CStr as *mut MallocCStr)
    }
}
