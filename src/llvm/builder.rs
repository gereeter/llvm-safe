use std::ffi::CStr;
use std::marker::PhantomData;

use libc::c_uint;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
pub use llvm_sys::{LLVMIntPredicate, LLVMRealPredicate};

use opaque::Opaque;
use owned::{Owned, DropInPlace};

use llvm::{Context, BasicBlock, Label, Value, Phi, Alloca, Type};

pub struct Builder<'cid: 'context, 'context> {
    _context: PhantomData<&'context Context<'cid>>,
    _opaque: Opaque
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

    pub fn position_at_end<'mid: 'block, 'fid: 'block, 'block, 'builder>(&'builder mut self, block: &'block mut BasicBlock<'cid, 'mid, 'fid>) -> &'builder mut PositionedBuilder<'cid, 'context, 'mid, 'fid, 'block> {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_raw(), block.as_raw());
            &mut *(self as *mut Builder as *mut PositionedBuilder)
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self as *const Builder as *mut Builder as LLVMBuilderRef
    }
}

pub struct PositionedBuilder<'cid: 'context, 'context: 'block, 'mid: 'block, 'fid: 'block, 'block> {
    _block: PhantomData<&'block mut BasicBlock<'cid, 'mid, 'fid>>,
    _builder: PhantomData<Builder<'cid, 'context>>,
    _opaque: Opaque
}

impl<'cid, 'context, 'mid, 'fid, 'block> PositionedBuilder<'cid, 'context, 'mid, 'fid, 'block> {
    pub fn br(&mut self, target: &Label<'fid>) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildBr(self.as_raw(), target.as_raw()) as *const Value)
        }
    }

    pub fn cond_br(&mut self, cond: &Value<'cid, 'mid, 'fid>, then_block: &Label<'fid>, else_block: &Label<'fid>) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildCondBr(self.as_raw(), cond.as_raw(), then_block.as_raw(), else_block.as_raw()) as *const Value)
        }
    }

    pub fn add(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fadd(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn sub(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fsub(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn mul(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fmul(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn udiv(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildUDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn sdiv(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildSDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fdiv(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn urem(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildURem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn srem(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildSRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn frem(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn shl(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildShl(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn lshr(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildLShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn ashr(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildAShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn and(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildAnd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn or(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildOr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn xor(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildXor(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn neg(&mut self, value: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildNeg(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fneg(&mut self, value: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFNeg(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn not(&mut self, value: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildNot(self.as_raw(), value.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn icmp(&mut self, pred: LLVMIntPredicate, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildICmp(self.as_raw(), pred, lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fcmp(&mut self, pred: LLVMRealPredicate, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFCmp(self.as_raw(), pred, lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn trunc(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildTrunc(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fp_trunc(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFPTrunc(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn zext(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildZExt(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn sext(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildSExt(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fp_ext(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFPExt(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fp_to_ui(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFPToUI(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn fp_to_si(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildFPToSI(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn ui_to_fp(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildUIToFP(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn si_to_fp(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildSIToFP(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn bitcast(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildBitCast(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn get_element_ptr(&mut self, ptr: &Value<'cid, 'mid, 'fid>, indices: &[&Value<'cid, 'mid, 'fid>], name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildGEP(self.as_raw(), ptr.as_raw(), indices.as_ptr() as *mut LLVMValueRef, indices.len() as c_uint, name.as_ptr()) as *const Value)
        }
    }

    pub fn get_element_ptr_in_bounds(&mut self, ptr: &Value<'cid, 'mid, 'fid>, indices: &[&Value<'cid, 'mid, 'fid>], name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildInBoundsGEP(self.as_raw(), ptr.as_raw(), indices.as_ptr() as *mut LLVMValueRef, indices.len() as c_uint, name.as_ptr()) as *const Value)
        }
    }

    pub fn alloca(&mut self, ty: &Type<'cid>, name: &CStr) -> &'block mut Alloca<'cid, 'mid, 'fid> {
        unsafe {
            &mut *(LLVMBuildAlloca(self.as_raw(), ty.as_raw(), name.as_ptr()) as *mut Alloca)
        }
    }

    pub fn array_alloca(&mut self, ty: &Type<'cid>, len: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block mut Alloca<'cid, 'mid, 'fid> {
        unsafe {
            &mut *(LLVMBuildArrayAlloca(self.as_raw(), ty.as_raw(), len.as_raw(), name.as_ptr()) as *mut Alloca)
        }
    }

    pub fn load(&mut self, ptr: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildLoad(self.as_raw(), ptr.as_raw(), name.as_ptr()) as *const Value)
        }
    }

    pub fn store(&mut self, value: &Value<'cid, 'mid, 'fid>, ptr: &Value<'cid, 'mid, 'fid>) {
        unsafe {
            LLVMBuildStore(self.as_raw(), value.as_raw(), ptr.as_raw());
        }
    }

    pub fn phi(&mut self, ty: &Type<'cid>, name: &CStr) -> &'block mut Phi<'cid, 'mid, 'fid> {
        unsafe {
            &mut *(LLVMBuildPhi(self.as_raw(), ty.as_raw(), name.as_ptr()) as *mut Phi)
        }
    }

    pub fn call(&mut self, func: &Value<'cid, 'mid, 'fid>, args: &[&Value<'cid, 'mid, 'fid>], name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildCall(self.as_raw(), func.as_raw(), args.as_ptr() as *const LLVMValueRef as *mut LLVMValueRef, args.len() as u32, name.as_ptr()) as *const Value)
        }
    }

    pub fn ret(&mut self, value: &Value<'cid, 'mid, 'fid>) {
        unsafe {
            LLVMBuildRet(self.as_raw(), value.as_raw());
        }
    }

    pub fn ret_void(&mut self) {
        unsafe {
            LLVMBuildRetVoid(self.as_raw());
        }
    }


    pub fn unreachable(&mut self) {
        unsafe {
            LLVMBuildUnreachable(self.as_raw());
        }
    }


    pub fn get_position(&self) -> &'block Label<'fid> {
        unsafe {
            &*(LLVMGetInsertBlock(self.as_raw()) as *mut Label as *const Label)
        }
    }

    pub fn position_at_end(&mut self, block: &'block mut BasicBlock<'cid, 'mid, 'fid>) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_raw(), block.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self as *const PositionedBuilder as *mut PositionedBuilder as LLVMBuilderRef
    }
}
