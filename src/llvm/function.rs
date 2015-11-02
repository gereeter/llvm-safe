use std::ffi::CStr;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::{Id, IdRef};

use llvm::{Context, BasicBlock, Label, Value};

pub struct Function<'cid> {
    _context_id: IdRef<'cid>
}

impl<'cid> Function<'cid> {
    pub fn builder<'fid, 'function>(&'function mut self, id: Id<'fid>) -> FunctionBuilder<'cid, 'fid, 'function> {
        FunctionBuilder {
            inner: self,
            _id: id
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpValue(self.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Function as *mut Function as LLVMValueRef
    }
}

pub struct FunctionBuilder<'cid: 'function, 'fid, 'function> {
    inner: &'function mut Function<'cid>,
    _id: Id<'fid>
}

impl<'cid, 'fid, 'function> FunctionBuilder<'cid, 'fid, 'function> {
    pub fn append_basic_block(&mut self, name: &CStr, context: &Context<'cid>) -> (&'function Label<'cid, 'fid>, &'function mut BasicBlock<'cid, 'fid>) {
        unsafe {
            let bb_ref = LLVMAppendBasicBlockInContext(context.as_raw(), self.inner.as_raw(), name.as_ptr());
            (&*(bb_ref as *mut Label), &mut *(bb_ref as *mut BasicBlock))
        }
    }

    pub fn param(&self, index: u32) -> &'function Value<'cid, 'fid> {
        unsafe {
            &*(LLVMGetParam(self.inner.as_raw(), index) as *const Value)
        }
    }
}

pub struct FunctionLabel<'cid> {
    _context_id: IdRef<'cid>
}

impl<'cid> FunctionLabel<'cid> {
    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const FunctionLabel as *mut FunctionLabel as LLVMValueRef
    }
}
