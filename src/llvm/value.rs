use std::marker::PhantomData;

use llvm_sys::prelude::*;

use id::IdRef;

#[derive(Copy, Clone)]
pub struct Value<'cid, 'fid, 'function> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>,
    _function: PhantomData<&'function ()>,
    llvm_value: LLVMValueRef
}

impl<'cid, 'fid, 'function> Value<'cid, 'fid, 'function> {
    pub unsafe fn from_raw(raw: LLVMValueRef) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: raw
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.llvm_value
    }
}
