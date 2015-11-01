use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::{Id, IdRef};

use llvm::{Context, BasicBlock, Value};

pub struct Function<'cid, 'mid: 'module, 'module, 'fid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _module: PhantomData<&'module ()>,
    _id: Id<'fid>,
    llvm_function: LLVMValueRef
}

impl<'cid, 'mid: 'module, 'module, 'fid> Function<'cid, 'mid, 'module, 'fid> {
    pub fn builder<'function>(&'function mut self) -> FunctionBuilder<'cid, 'mid, 'module, 'fid, 'function> {
        FunctionBuilder {
            inner: self
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpValue(self.llvm_function);
        }
    }

    pub unsafe fn from_raw(id: Id<'fid>, raw: LLVMValueRef) -> Function<'cid, 'mid, 'module, 'fid> {
        Function {
            _context_id: IdRef::new(),
            _module_id: IdRef::new(),
            _module: PhantomData,
            _id: id,
            llvm_function: raw
        }
    }
}

pub struct FunctionBuilder<'cid: 'function, 'mid: 'module, 'module: 'function, 'fid: 'function, 'function> {
    inner: &'function mut Function<'cid, 'mid, 'module, 'fid>
}

impl<'cid, 'mid, 'module, 'fid, 'function> FunctionBuilder<'cid, 'mid, 'module, 'fid, 'function> {
    pub fn append_basic_block(&mut self, name: &CStr, context: &Context<'cid>) -> &'function mut BasicBlock<'cid, 'fid> {
        unsafe {
            &mut *(LLVMAppendBasicBlockInContext(context.as_raw(), self.inner.llvm_function, name.as_ptr()) as *mut BasicBlock)
        }
    }

    pub fn param(&self, index: u32) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMGetParam(self.inner.llvm_function, index))
        }
    }
}
