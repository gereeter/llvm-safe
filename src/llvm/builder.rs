use std::ffi::CStr;
use std::marker::PhantomData;

use libc::c_uint;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
pub use llvm_sys::{LLVMIntPredicate, LLVMRealPredicate};

use inheritance::upcast;
use opaque::Opaque;
use owned::{Owned, DropInPlace};

use llvm::{Context, BasicBlock, Label, Value, Phi, Alloca, Type, FunctionType};

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

macro_rules! binop_impl {
    ( $rust_name:ident, $c_name:ident )  => {
        pub fn $rust_name(&mut self, lhs: &Value<'cid, 'mid, 'fid>, rhs: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
            unsafe {
                &*($c_name(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr()) as *const Value)
            }
        }
    };
}

macro_rules! cast_impl {
    ( $rust_name:ident, $c_name:ident )  => {
        pub fn $rust_name(&mut self, value: &Value<'cid, 'mid, 'fid>, dest_ty: &Type<'cid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
            unsafe {
                &*($c_name(self.as_raw(), value.as_raw(), dest_ty.as_raw(), name.as_ptr()) as *const Value)
            }
        }
    };
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

    binop_impl!{ add,     LLVMBuildAdd }
    binop_impl!{ add_nsw, LLVMBuildNSWAdd }
    binop_impl!{ add_nuw, LLVMBuildNUWAdd }
    binop_impl!{ fadd,    LLVMBuildFAdd }

    binop_impl!{ sub,     LLVMBuildSub }
    binop_impl!{ sub_nsw, LLVMBuildNSWSub }
    binop_impl!{ sub_nuw, LLVMBuildNUWSub }
    binop_impl!{ fsub,    LLVMBuildFSub }

    binop_impl!{ mul,     LLVMBuildMul }
    binop_impl!{ mul_nsw, LLVMBuildNSWMul }
    binop_impl!{ mul_nuw, LLVMBuildNUWMul }
    binop_impl!{ fmul,    LLVMBuildFMul }

    binop_impl!{ udiv,       LLVMBuildUDiv }
    binop_impl!{ udiv_exact, LLVMBuildExactUDiv }
    binop_impl!{ sdiv,       LLVMBuildSDiv }
    binop_impl!{ sdiv_exact, LLVMBuildExactSDiv }
    binop_impl!{ fdiv,       LLVMBuildFDiv }

    binop_impl!{ urem, LLVMBuildURem }
    binop_impl!{ srem, LLVMBuildSRem }
    binop_impl!{ frem, LLVMBuildFRem }

    binop_impl!{ shl,  LLVMBuildShl }
    binop_impl!{ lshr, LLVMBuildLShr }
    binop_impl!{ ashr, LLVMBuildAShr }
    binop_impl!{ and,  LLVMBuildAnd }
    binop_impl!{ or,   LLVMBuildOr }
    binop_impl!{ xor,  LLVMBuildXor }

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

    cast_impl!{ trunc,    LLVMBuildTrunc }
    cast_impl!{ fp_trunc, LLVMBuildFPTrunc }

    cast_impl!{ zext,   LLVMBuildZExt }
    cast_impl!{ sext,   LLVMBuildSExt }
    cast_impl!{ fp_ext, LLVMBuildFPExt }

    cast_impl!{ fp_to_ui,   LLVMBuildFPToUI }
    cast_impl!{ fp_to_si,   LLVMBuildFPToSI }
    cast_impl!{ ui_to_fp,   LLVMBuildUIToFP }
    cast_impl!{ si_to_fp,   LLVMBuildSIToFP }
    cast_impl!{ ptr_to_int, LLVMBuildPtrToInt }
    cast_impl!{ int_to_ptr, LLVMBuildIntToPtr }
    cast_impl!{ bitcast,    LLVMBuildBitCast }

    pub fn get_element_ptr(&mut self, ty: &Type<'cid>, ptr: &Value<'cid, 'mid, 'fid>, indices: &[&Value<'cid, 'mid, 'fid>], name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildGEP2(self.as_raw(), ty.as_raw(), ptr.as_raw(), indices.as_ptr() as *mut LLVMValueRef, indices.len() as c_uint, name.as_ptr()) as *const Value)
        }
    }

    pub fn get_element_ptr_in_bounds(&mut self, ty: &Type<'cid>, ptr: &Value<'cid, 'mid, 'fid>, indices: &[&Value<'cid, 'mid, 'fid>], name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildInBoundsGEP2(self.as_raw(), ty.as_raw(), ptr.as_raw(), indices.as_ptr() as *mut LLVMValueRef, indices.len() as c_uint, name.as_ptr()) as *const Value)
        }
    }

    pub fn memset(&mut self, ptr: &Value<'cid, 'mid, 'fid>, value: &Value<'cid, 'mid, 'fid>, len: &Value<'cid, 'mid, 'fid>, align: c_uint) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildMemSet(self.as_raw(), ptr.as_raw(), value.as_raw(), len.as_raw(), align) as *mut Value)
        }
    }

    pub fn memcpy(&mut self, dest: &Value<'cid, 'mid, 'fid>, dest_align: c_uint, src: &Value<'cid, 'mid, 'fid>, src_align: c_uint, size: &Value<'cid, 'mid, 'fid>) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildMemCpy(self.as_raw(), dest.as_raw(), dest_align, src.as_raw(), src_align, size.as_raw()) as *mut Value)
        }
    }

    pub fn memmove(&mut self, dest: &Value<'cid, 'mid, 'fid>, dest_align: c_uint, src: &Value<'cid, 'mid, 'fid>, src_align: c_uint, size: &Value<'cid, 'mid, 'fid>) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildMemMove(self.as_raw(), dest.as_raw(), dest_align, src.as_raw(), src_align, size.as_raw()) as *mut Value)
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

    pub fn load(&mut self, ty: &Type<'cid>, ptr: &Value<'cid, 'mid, 'fid>, name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildLoad2(self.as_raw(), ty.as_raw(), ptr.as_raw(), name.as_ptr()) as *const Value)
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

    pub fn call(&mut self, ty: &FunctionType<'cid>, func: &Value<'cid, 'mid, 'fid>, args: &[&Value<'cid, 'mid, 'fid>], name: &CStr) -> &'block Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(LLVMBuildCall2(self.as_raw(), upcast::<_,Type>(ty).as_raw(), func.as_raw(), args.as_ptr() as *const LLVMValueRef as *mut LLVMValueRef, args.len() as u32, name.as_ptr()) as *const Value)
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
