use llvm_sys::prelude::*;
use llvm_sys::core::*;

use libc::{c_int, c_uint};

use std::marker::PhantomData;

use id::IdRef;
use opaque::Opaque;

use llvm::value::Value;
use llvm::context::Context;

pub struct Type<'cid, SubType> {
    _context_id: IdRef<'cid>,
    _marker: PhantomData<SubType>,
    _opaque: Opaque
}

impl<'cid> Type<'cid, AnyType> {
    pub fn f32<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid, AnyType> {
        unsafe {
            &*(LLVMFloatTypeInContext(context.as_raw()) as *mut Type<AnyType>)
        }
    }

    pub fn f64<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid, AnyType> {
        unsafe {
            &*(LLVMDoubleTypeInContext(context.as_raw()) as *mut Type<AnyType>)
        }
    }

    pub fn i1<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid, AnyType> {
        unsafe {
            &*(LLVMInt1TypeInContext(context.as_raw()) as *mut Type<AnyType>)
        }
    }

    pub fn i8<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid, AnyType> {
        unsafe {
            &*(LLVMInt8TypeInContext(context.as_raw()) as *mut Type<AnyType>)
        }
    }

    pub fn i16<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid, AnyType> {
        unsafe {
            &*(LLVMInt16TypeInContext(context.as_raw()) as *mut Type<AnyType>)
        }
    }

    pub fn i32<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid, AnyType> {
        unsafe {
            &*(LLVMInt32TypeInContext(context.as_raw()) as *mut Type<AnyType>)
        }
    }

    pub fn i64<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid, AnyType> {
        unsafe {
            &*(LLVMInt64TypeInContext(context.as_raw()) as *mut Type<AnyType>)
        }
    }

    pub fn void<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid, AnyType> {
        unsafe {
            &*(LLVMVoidTypeInContext(context.as_raw()) as *mut Type<AnyType>)
        }
    }

    pub fn of_value<'a, 'mid, 'fid>(value: &'a Value<'cid, 'mid, 'fid>) -> &'a Type<'cid, AnyType> {
        unsafe {
            &*(LLVMTypeOf(value.as_raw()) as *mut Type<AnyType>)
        }
    }
}

impl<'cid, SubType> Type<'cid, SubType> {
    pub fn pointer<'ctx>(inner: &'ctx Type<'cid, SubType>, address_space: c_uint) -> &'ctx Type<'cid, PointerType<SubType>> {
        unsafe {
            &*(LLVMPointerType(inner.as_raw(), address_space) as *mut Type<PointerType<SubType>>)
        }
    }

    pub fn array<'ctx>(inner: &'ctx Type<'cid, SubType>, count: c_uint) -> &'ctx Type<'cid, ArrayType<SubType>> {
        unsafe {
            &*(LLVMArrayType(inner.as_raw(), count) as *mut Type<ArrayType<SubType>>)
        }
    }

    pub fn function<'ctx, ArgTy>(params: &[&'ctx Type<'cid, ArgTy>], ret: &'ctx Type<'cid, SubType>, var_arg: bool) -> &'ctx Type<'cid, FunctionType> {
        unsafe {
            &*(LLVMFunctionType(ret.as_raw(), params.as_ptr() as *mut LLVMTypeRef, params.len() as u32, var_arg as c_int) as *mut Type<FunctionType>)
        }
    }

    pub fn upcast<TargetTy>(&self) -> &Type<'cid, TargetTy> where SubType: Upcast<TargetTy> {
        unsafe {
            &*(self.as_raw() as *mut Type<TargetTy>)
        }
    }

    pub unsafe fn cast_unchecked<TargetTy>(&self) -> &Type<'cid, TargetTy> {
        &*(self.as_raw() as *mut Type<TargetTy>)
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpType(self.as_raw())
        }
    }

    pub fn as_raw(&self) -> LLVMTypeRef {
        self as *const Type<SubType> as *mut Type<SubType> as LLVMTypeRef
    }
}

impl<'cid> Type<'cid, FunctionType> {
    pub fn is_var_arg(&self) -> bool {
        unsafe {
            LLVMIsFunctionVarArg(self.as_raw()) != 0
        }
    }
}

impl<'cid, PointeeTy> Type<'cid, PointerType<PointeeTy>> {
    pub fn pointee_ty(&self) -> &Type<'cid, PointeeTy> {
        unsafe {
            &*(LLVMGetElementType(self.as_raw()) as *mut Type<PointeeTy>)
        }
    }
}

impl<'cid, ElementTy> Type<'cid, ArrayType<ElementTy>> {
    pub fn element_ty(&self) -> &Type<'cid, ElementTy> {
        unsafe {
            &*(LLVMGetElementType(self.as_raw()) as *mut Type<ElementTy>)
        }
    }
}

// TODO: Consider blanket impls?
pub unsafe trait Upcast<Target> { }
unsafe impl Upcast<AnyType> for AnyType { }
unsafe impl<PointeeTy> Upcast<AnyType> for PointerType<PointeeTy> { }
unsafe impl<GeneralPointeeTy, PointeeTy: Upcast<GeneralPointeeTy>> Upcast<PointerType<GeneralPointeeTy>> for PointerType<PointeeTy> { }
unsafe impl<ElementTy> Upcast<AnyType> for ArrayType<ElementTy> { }
unsafe impl<GeneralElementTy, ElementTy: Upcast<GeneralElementTy>> Upcast<ArrayType<GeneralElementTy>> for ArrayType<ElementTy> { }
unsafe impl Upcast<AnyType> for FunctionType { }

pub enum AnyType { }
pub struct PointerType<PointeeTy> {
    _marker: PhantomData<PointeeTy>
}
pub struct ArrayType<ElementTy> {
    _marker: PhantomData<ElementTy>
}
pub enum FunctionType { }
