use std::marker::PhantomData;

use llvm_sys::prelude::*;

use id::IdRef;

#[derive(Copy, Clone)]
pub struct BasicBlock<'cid, 'fid, 'function> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>,
    _function: PhantomData<&'function ()>,
    llvm_basic_block: LLVMBasicBlockRef
}

impl<'cid, 'fid, 'function> BasicBlock<'cid, 'fid, 'function> {
    pub unsafe fn from_raw(raw: LLVMBasicBlockRef) -> BasicBlock<'cid, 'fid, 'function> {
        BasicBlock {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_basic_block: raw
        }
    }

    pub fn as_raw(&self) -> LLVMBasicBlockRef {
        self.llvm_basic_block
    }
}
