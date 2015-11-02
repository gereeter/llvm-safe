use std::ffi::CStr;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::IdRef;

pub struct Value<'cid, 'fid> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>
}

impl<'cid, 'fid> Value<'cid, 'fid> {
    pub fn set_name(&self, name: &CStr) {
        unsafe {
            LLVMSetValueName(self.as_raw(), name.as_ptr());
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Value as *mut Value as LLVMValueRef
    }
}
