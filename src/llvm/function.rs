use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::analysis::*;

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

    pub fn params(&self) -> FunctionParams<'cid, 'fid, 'function> {
        FunctionParams {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            inner: unsafe { LLVMGetFirstParam(self.inner.as_raw()) }
        }
    }
}

pub struct FunctionParams<'cid: 'function, 'fid: 'function, 'function> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>,
    _function: PhantomData<&'function Function<'cid>>,
    inner: LLVMValueRef
}

impl<'cid: 'function, 'fid: 'function, 'function> Iterator for FunctionParams<'cid, 'fid, 'function> {
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

pub struct FunctionLabel<'cid> {
    _context_id: IdRef<'cid>
}

impl<'cid> FunctionLabel<'cid> {
    pub fn num_args(&self) -> usize {
        unsafe {
            LLVMCountParams(self.as_raw()) as usize
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const FunctionLabel as *mut FunctionLabel as LLVMValueRef
    }
}
