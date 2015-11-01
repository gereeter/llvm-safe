use std::ffi::CStr;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::{Id, IdRef};

use llvm::{Context, BasicBlock, Value};

pub struct Function<'cid, 'mid, 'fid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _id: Id<'fid>
}

impl<'cid, 'mid, 'fid> Function<'cid, 'mid, 'fid> {
    pub fn builder<'function>(&'function mut self) -> FunctionBuilder<'cid, 'mid, 'fid, 'function> {
        FunctionBuilder {
            inner: self
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

pub struct FunctionBuilder<'cid: 'function, 'mid: 'function, 'fid: 'function, 'function> {
    inner: &'function mut Function<'cid, 'mid, 'fid>
}

impl<'cid, 'mid, 'fid, 'function> FunctionBuilder<'cid, 'mid, 'fid, 'function> {
    pub fn append_basic_block(&mut self, name: &CStr, context: &Context<'cid>) -> &'function mut BasicBlock<'cid, 'fid> {
        unsafe {
            &mut *(LLVMAppendBasicBlockInContext(context.as_raw(), self.inner.as_raw(), name.as_ptr()) as *mut BasicBlock)
        }
    }

    pub fn param(&self, index: u32) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMGetParam(self.inner.as_raw(), index))
        }
    }
}
