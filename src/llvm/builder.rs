use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
pub use llvm_sys::{LLVMIntPredicate, LLVMRealPredicate};

use owned::{Owned, DropInPlace};

use llvm::{Context, BasicBlock, Value, Type};

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

    pub fn cond_br(&mut self, cond: &Value<'cid, 'fid>, then_block: &BasicBlock<'cid, 'fid>, else_block: &BasicBlock<'cid, 'fid>) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildCondBr(self.as_raw(), cond.as_raw(), then_block.as_raw(), else_block.as_raw()) as *const Value)
        }
    }

    pub fn add(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fadd(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildFAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn sub(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fsub(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildFSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn mul(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fmul(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildFMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn udiv(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildUDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn sdiv(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildSDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fdiv(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildFDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn urem(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildURem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn srem(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildSRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn frem(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildFRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn shl(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildShl(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn lshr(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildLShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn ashr(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildAShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn and(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildAnd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn or(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildOr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn xor(&mut self, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildXor(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn neg(&mut self, value: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildNeg(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fneg(&mut self, value: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildFNeg(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn not(&mut self, value: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildNot(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn icmp(&mut self, pred: LLVMIntPredicate, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildICmp(self.as_raw(), pred, lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fcmp(&mut self, pred: LLVMRealPredicate, lhs: &Value<'cid, 'fid>, rhs: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildFCmp(self.as_raw(), pred, lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn alloca(&mut self, ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildAlloca(self.as_raw(), ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn array_alloca(&mut self, ty: &Type<'cid>, len: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildArrayAlloca(self.as_raw(), ty.as_raw(), len.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn load(&mut self, ptr: &Value<'cid, 'fid>, name: &CStr) -> &'block Value<'cid, 'fid> {
        unsafe {
            &*(LLVMBuildLoad(self.as_raw(), ptr.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn store(&mut self, value: &Value<'cid, 'fid>, ptr: &Value<'cid, 'fid>) {
        unsafe {
            LLVMBuildStore(self.as_raw(), value.as_raw(), ptr.as_raw());
        }
    }

    pub fn ret(&mut self, value: &Value<'cid, 'fid>) {
        unsafe {
            LLVMBuildRet(self.as_raw(), value.as_raw());
        }
    }

    pub fn ret_void(&mut self) {
        unsafe {
            LLVMBuildRetVoid(self.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self as *const PositionedBuilder as *mut PositionedBuilder as LLVMBuilderRef
    }
}
