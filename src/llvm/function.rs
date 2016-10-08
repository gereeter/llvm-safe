use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::analysis::*;

use id::{Id, IdRef};

use llvm::{Context, BasicBlock, Label, Value};

pub struct Function<'cid, 'mid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>
}

impl<'cid, 'mid> Function<'cid, 'mid> {
    pub fn builder<'fid, 'function>(&'function mut self, id: Id<'fid>) -> FunctionBuilder<'cid, 'mid, 'fid, 'function> {
        FunctionBuilder {
            inner: self,
            _id: id
        }
    }

    pub fn verify(&self) {
        unsafe {
            LLVMVerifyFunction(self.as_raw(), LLVMVerifierFailureAction::LLVMAbortProcessAction);
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

pub struct FunctionBuilder<'cid: 'function, 'mid: 'function, 'fid, 'function> {
    inner: &'function mut Function<'cid, 'mid>,
    _id: Id<'fid>
}

impl<'cid, 'mid, 'fid, 'function> FunctionBuilder<'cid, 'mid, 'fid, 'function> {
    pub fn append_basic_block(&mut self, name: &CStr, context: &Context<'cid>) -> (&'function Label<'cid, 'fid>, &'function mut BasicBlock<'cid, 'fid>) {
        unsafe {
            let bb_ref = LLVMAppendBasicBlockInContext(context.as_raw(), self.inner.as_raw(), name.as_ptr());
            (&*(bb_ref as *mut Label), &mut *(bb_ref as *mut BasicBlock))
        }
    }

    pub fn params(&self) -> FunctionParams<'cid, 'mid, 'fid, 'function> {
        FunctionParams {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            inner: unsafe { LLVMGetFirstParam(self.inner.as_raw()) }
        }
    }
}

pub struct FunctionParams<'cid: 'function, 'mid: 'function, 'fid: 'function, 'function> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>,
    _function: PhantomData<&'function Function<'cid, 'mid>>,
    inner: LLVMValueRef
}

impl<'cid: 'function, 'mid: 'function, 'fid: 'function, 'function> Iterator for FunctionParams<'cid, 'mid, 'fid, 'function> {
    type Item = &'function Value<'cid, 'fid>;

    fn next(&mut self) -> Option<&'function Value<'cid, 'fid>> {
        if self.inner.is_null() {
            None
        } else {
            unsafe {
                let ret = Some(&*(self.inner as *const Value));
                self.inner = LLVMGetNextParam(self.inner);
                ret
            }
        }
    }
}

pub struct FunctionLabel<'cid, 'mid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>
}

impl<'cid, 'mid> FunctionLabel<'cid, 'mid> {
    pub fn num_args(&self) -> usize {
        unsafe {
            LLVMCountParams(self.as_raw()) as usize
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const FunctionLabel as *mut FunctionLabel as LLVMValueRef
    }
}
