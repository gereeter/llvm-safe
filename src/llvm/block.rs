use llvm_sys::prelude::*;

use id::IdRef;
use opaque::Opaque;

pub struct BasicBlock<'cid, 'mid, 'fid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _function_id: IdRef<'fid>,
    _opaque: Opaque
}

impl<'cid, 'mid, 'fid> BasicBlock<'cid, 'mid, 'fid> {
    pub fn as_raw(&self) -> LLVMBasicBlockRef {
        self as *const BasicBlock as *mut BasicBlock as LLVMBasicBlockRef
    }
}

pub struct Label<'fid> {
    _function_id: IdRef<'fid>
}

impl<'fid> Label<'fid> {
    pub fn as_raw(&self) -> LLVMBasicBlockRef {
        self as *const Label as *mut Label as LLVMBasicBlockRef
    }
}
