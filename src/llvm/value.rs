use llvm_sys::prelude::*;

use id::IdRef;

pub struct Value<'cid, 'fid> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>
}

impl<'cid, 'fid> Value<'cid, 'fid> {
    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Value as *mut Value as LLVMValueRef
    }
}
