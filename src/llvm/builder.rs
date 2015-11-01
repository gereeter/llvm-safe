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

    pub fn position_at_end<'fid, 'block, 'builder>(&'builder mut self, block: &'block mut BasicBlock<'cid, 'fid>) -> &'builder mut PositionedBuilder<'cid, 'context, 'fid, 'block> {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_raw(), block.as_raw());
            &mut *(self as *mut Builder as *mut PositionedBuilder)
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self as *const Builder as *mut Builder as LLVMBuilderRef
    }
}

pub struct PositionedBuilder<'cid: 'context, 'context: 'block, 'fid: 'block, 'block> {
    _block: PhantomData<&'block mut BasicBlock<'cid, 'fid>>,
    _builder: PhantomData<Builder<'cid, 'context>>
}

impl<'cid, 'context, 'fid, 'block> PositionedBuilder<'cid, 'context, 'fid, 'block> {
    pub fn br(&mut self, target: &BasicBlock<'cid, 'fid>) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildBr(self.as_raw(), target.as_raw()) as *const Value)
        }
    }

    pub fn or(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildOr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn add(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn and(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildAnd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn mul(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn neg(&mut self, value: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildNeg(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn not(&mut self, value: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildNot(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn ret(&mut self, value: &Value<'cid, 'fid>) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildRet(self.as_raw(), value.as_raw()) as *const Value)
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self as *const PositionedBuilder as *mut PositionedBuilder as LLVMBuilderRef
    }
}
