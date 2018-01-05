use llvm_sys::prelude::*;
use llvm_sys::core::*;

use libc::c_uint;

use id::IdRef;
use opaque::Opaque;

use llvm::Value;

pub struct Alloca<'cid, 'mid, 'fid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _function_id: IdRef<'fid>,
    _opaque: Opaque
}

impl<'cid, 'mid, 'fid> Alloca<'cid, 'mid, 'fid> {
    pub fn set_alignment(&mut self, alignment: c_uint) {
        unsafe {
            LLVMSetAlignment(self.as_raw(), alignment);
        }
    }

    pub fn as_value(&self) -> &Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(self.as_raw() as *mut Value)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Alloca as *mut Alloca as LLVMValueRef
    }
}
