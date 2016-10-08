use llvm_sys::prelude::*;

use id::IdRef;

pub struct BasicBlock<'cid, 'mid, 'fid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _function_id: IdRef<'fid>
}

impl<'cid, 'mid, 'fid> BasicBlock<'cid, 'mid, 'fid> {
    pub fn as_raw(&self) -> LLVMBasicBlockRef {
        self as *const BasicBlock as *mut BasicBlock as LLVMBasicBlockRef
    }
}

pub struct Label<'cid, 'fid> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>
}

impl<'cid, 'fid> Label<'cid, 'fid> {
    pub fn as_raw(&self) -> LLVMBasicBlockRef {
        self as *const Label as *mut Label as LLVMBasicBlockRef
    }
}
