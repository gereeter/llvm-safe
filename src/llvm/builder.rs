use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use owned::{Owned, DropInPlace};

use llvm::{Context, BasicBlock, Value};

pub struct Builder<'cid: 'context, 'context> {
    _context: PhantomData<&'context Context<'cid>>
}

impl<'cid, 'context> DropInPlace for Builder<'cid, 'context> {
    unsafe fn drop_in_place(&mut self) {
        LLVMDisposeBuilder(self.as_raw());
    }
}

impl<'cid, 'context> Builder<'cid, 'context> {
    pub fn new(context: &'context Context<'cid>) -> Owned<Builder<'cid, 'context>> {
        unsafe {
            Owned::from_raw(
                LLVMCreateBuilderInContext(context.as_raw()) as *mut Builder
            )
        }
    }

    pub fn position_at_end<'fid, 'function, 'builder>(&'builder mut self, block: &'function mut BasicBlock<'cid, 'fid>) -> &'builder mut PositionedBuilder<'cid, 'context, 'fid, 'function> {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_raw(), block.as_raw());
            &mut *(self as *mut Builder as *mut PositionedBuilder)
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self as *const Builder as *mut Builder as LLVMBuilderRef
    }
}

pub struct PositionedBuilder<'cid: 'context, 'context: 'function, 'fid: 'function, 'function> {
    _block: PhantomData<&'function mut BasicBlock<'cid, 'fid>>,
    _builder: PhantomData<Builder<'cid, 'context>>
}

impl<'cid, 'context, 'fid, 'function> PositionedBuilder<'cid, 'context, 'fid, 'function> {
    pub fn br(&mut self, target: &BasicBlock<'cid, 'fid>) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildBr(self.as_raw(), target.as_raw()))
        }
    }

    pub fn or(&mut self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildOr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()))
        }
    }

    pub fn add(&mut self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()))
        }
    }

    pub fn and(&mut self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildAnd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()))
        }
    }

    pub fn mul(&mut self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()))
        }
    }

    pub fn neg(&mut self, value: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildNeg(self.as_raw(), value.as_raw(), name.as_ptr()))
        }
    }

    pub fn not(&mut self, value: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildNot(self.as_raw(), value.as_raw(), name.as_ptr()))
        }
    }

    pub fn ret(&mut self, value: Value<'cid, 'fid, 'function>) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildRet(self.as_raw(), value.as_raw()))
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self as *const PositionedBuilder as *mut PositionedBuilder as LLVMBuilderRef
    }
}
