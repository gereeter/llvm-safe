use llvm_sys::prelude::*;
use llvm_sys::core::LLVMSetInitializer;

use id::IdRef;
use opaque::Opaque;

use llvm::{Constant, Value};

pub struct Global<'cid, 'mid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _opaque: Opaque
}

impl<'cid, 'mid> Global<'cid, 'mid> {
    pub fn set_initializer(&mut self, value: &Constant<'cid>) {
        unsafe {
            LLVMSetInitializer(self.as_raw(), value.as_raw());
        }
    }

    pub fn as_value<'fid>(&self) -> &Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(self.as_raw() as *mut Value)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Global as *mut Global as LLVMValueRef
    }
}
