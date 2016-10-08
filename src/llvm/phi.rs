use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::IdRef;

use llvm::{Label, Value};

pub struct Phi<'cid, 'fid> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>
}

impl<'cid, 'fid> Phi<'cid, 'fid> {
    // TODO: Expose bulk addition?
    pub fn add_incoming_branch(&mut self, value: &Value<'cid, 'fid>, block: &Label<'fid>) {
        unsafe {
            LLVMAddIncoming(self.as_raw(), [value.as_raw()].as_mut_ptr(), [block.as_raw()].as_mut_ptr(), 1);
        }
    }

    pub fn as_value(&self) -> &Value<'cid, 'fid> {
        unsafe {
            &*(self.as_raw() as *mut Value)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Phi as *mut Phi as LLVMValueRef
    }
}
