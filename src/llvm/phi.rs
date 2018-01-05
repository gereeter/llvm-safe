use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::IdRef;
use opaque::Opaque;

use llvm::{Label, Value};

pub struct Phi<'cid, 'mid, 'fid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _function_id: IdRef<'fid>,
    _opaque: Opaque
}

impl<'cid, 'mid, 'fid> Phi<'cid, 'mid, 'fid> {
    // TODO: Expose bulk addition?
    pub fn add_incoming_branch(&mut self, value: &Value<'cid, 'mid, 'fid>, block: &Label<'fid>) {
        unsafe {
            LLVMAddIncoming(self.as_raw(), [value.as_raw()].as_mut_ptr(), [block.as_raw()].as_mut_ptr(), 1);
        }
    }

    pub fn as_value(&self) -> &Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(self.as_raw() as *mut Value)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Phi as *mut Phi as LLVMValueRef
    }
}
