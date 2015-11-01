use std::marker::PhantomData;

use llvm_sys::prelude::*;

use id::IdRef;

#[derive(Copy, Clone)]
pub struct BasicBlock<'cid, 'fid> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>
}

impl<'cid, 'fid> BasicBlock<'cid, 'fid> {
    pub fn as_raw(&self) -> LLVMBasicBlockRef {
        self as *const BasicBlock as *mut BasicBlock as LLVMBasicBlockRef
    }
}
