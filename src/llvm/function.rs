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
    pub fn append_basic_block<'function>(&'function self, name: &CStr, context: &Context<'cid>) -> BasicBlock<'cid, 'fid, 'function> {
        BasicBlock {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_basic_block: unsafe { LLVMAppendBasicBlockInContext(context.as_raw(), self.llvm_function, name.as_ptr()) }
        }
    }

    pub fn param<'function>(&'function self, index: u32) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMGetParam(self.llvm_function, index) }
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
