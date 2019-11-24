use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::analysis::*;

use id::{Id, IdRef};
use inheritance::downcast_unchecked;
use opaque::Opaque;

use llvm::{Context, BasicBlock, Label, Value, Type, FunctionType, PointerType};

pub struct Function<'cid, 'mid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _opaque: Opaque
}

impl<'cid, 'mid> Function<'cid, 'mid> {
    pub fn label<'function>(&'function self) -> &'function FunctionLabel<'cid, 'mid> {
        unsafe {
            &*(self.as_raw() as *mut FunctionLabel)
        }
    }

    pub fn builder<'fid, 'function>(&'function mut self, _id: Id<'fid>) -> &'function mut FunctionBuilder<'cid, 'mid, 'fid, 'function> {
        unsafe {
            &mut *(self.as_raw() as *mut FunctionBuilder)
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
    _inner: PhantomData<&'function mut Function<'cid, 'mid>>,
    _id: Id<'fid>,
    _opaque: Opaque
}

impl<'cid, 'mid, 'fid, 'function> FunctionBuilder<'cid, 'mid, 'fid, 'function> {
    pub fn append_basic_block(&mut self, name: &CStr, context: &Context<'cid>) -> (&'function Label<'fid>, &'function mut BasicBlock<'cid, 'mid, 'fid>) {
        unsafe {
            let bb_ref = LLVMAppendBasicBlockInContext(context.as_raw(), self.as_raw(), name.as_ptr());
            (&*(bb_ref as *mut Label), &mut *(bb_ref as *mut BasicBlock))
        }
    }

    pub fn params(&self) -> FunctionParams<'cid, 'mid, 'fid, 'function> {
        FunctionParams {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            inner: unsafe { LLVMGetFirstParam(self.as_raw()) }
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const FunctionBuilder as *mut FunctionBuilder as LLVMValueRef
    }
}

pub struct FunctionParams<'cid: 'function, 'mid: 'function, 'fid: 'function, 'function> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>,
    _function: PhantomData<&'function Function<'cid, 'mid>>,
    inner: LLVMValueRef
}

impl<'cid: 'function, 'mid: 'function, 'fid: 'function, 'function> Iterator for FunctionParams<'cid, 'mid, 'fid, 'function> {
    type Item = &'function Value<'cid, 'mid, 'fid>;

    fn next(&mut self) -> Option<&'function Value<'cid, 'mid, 'fid>> {
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
    _module_id: IdRef<'mid>,
    _opaque: Opaque
}

impl<'cid, 'mid> FunctionLabel<'cid, 'mid> {
    pub fn num_args(&self) -> usize {
        unsafe {
            LLVMCountParams(self.as_raw()) as usize
        }
    }

    pub fn function_type(&self) -> &FunctionType<'cid> {
        unsafe {
            // FIXME: return the pointer type?
            downcast_unchecked::<PointerType<FunctionType>,_>(Type::of_value(self.as_value())).pointee_ty()
        }
    }

    pub fn as_value<'fid>(&self) -> &Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(self.as_raw() as *mut Value)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const FunctionLabel as *mut FunctionLabel as LLVMValueRef
    }
}
