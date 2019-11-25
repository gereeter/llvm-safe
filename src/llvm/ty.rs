use llvm_sys::prelude::*;
use llvm_sys::core::*;

use libc::{c_int, c_uint};

use std::marker::PhantomData;
use std::mem::transmute_copy;

use id::IdRef;
use opaque::Opaque;
use inheritance::{upcast, DerivesFrom};

use llvm::value::Value;
use llvm::context::Context;

pub struct Type<'cid> {
    _context_id: IdRef<'cid>,
    _opaque: Opaque
}
unsafe impl<'cid> DerivesFrom<Type<'cid>> for Type<'cid> { }

pub struct PointerType<'cid, SubType: ?Sized> {
    _inner: PhantomData<SubType>,
    _super: Type<'cid>
}
unsafe impl<'cid, SubTypeSpecific: DerivesFrom<SubTypeGeneral> + ?Sized, SubTypeGeneral: ?Sized> DerivesFrom<PointerType<'cid, SubTypeGeneral>> for PointerType<'cid, SubTypeSpecific> { }
unsafe impl<'cid, General: ?Sized, SubType: ?Sized> DerivesFrom<General> for PointerType<'cid, SubType> where Type<'cid>: DerivesFrom<General> { }

pub struct ArrayType<'cid, SubType: ?Sized> {
    _inner: PhantomData<SubType>,
    _super: Type<'cid>
}
unsafe impl<'cid, SubTypeSpecific: DerivesFrom<SubTypeGeneral> + ?Sized, SubTypeGeneral: ?Sized> DerivesFrom<ArrayType<'cid, SubTypeGeneral>> for ArrayType<'cid, SubTypeSpecific> { }
unsafe impl<'cid, General: ?Sized, SubType: ?Sized> DerivesFrom<General> for ArrayType<'cid, SubType> where Type<'cid>: DerivesFrom<General> { }

pub struct FunctionType<'cid> {
    _super: Type<'cid>
}
unsafe impl<'cid> DerivesFrom<FunctionType<'cid>> for FunctionType<'cid> { }
unsafe impl<'cid, General: ?Sized> DerivesFrom<General> for FunctionType<'cid> where Type<'cid>: DerivesFrom<General> { }

pub struct IntegerType<'cid> {
    _super: Type<'cid>
}
unsafe impl<'cid> DerivesFrom<IntegerType<'cid>> for IntegerType<'cid> { }
unsafe impl<'cid, General: ?Sized> DerivesFrom<General> for IntegerType<'cid> where Type<'cid>: DerivesFrom<General> { }

impl<'cid> Type<'cid> {
    pub fn dump(&self) {
        unsafe {
            LLVMDumpType(self.as_raw())
        }
    }

    pub fn as_raw(&self) -> LLVMTypeRef {
        self as *const Type as *mut Type as LLVMTypeRef
    }

    pub fn f32<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMFloatTypeInContext(context.as_raw()) as *mut Type)
        }
    }

    pub fn f64<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMDoubleTypeInContext(context.as_raw()) as *mut Type)
        }
    }

    pub fn void<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMVoidTypeInContext(context.as_raw()) as *mut Type)
        }
    }

    pub fn of_value<'a, 'mid, 'fid, Ty: ?Sized>(value: &'a Value<'cid, 'mid, 'fid, Ty>) -> &'a Ty {
        unsafe {
            &*transmute_copy::<_,*mut Ty>(&LLVMTypeOf(value.as_raw()))
        }
    }

    pub fn pointer<'ctx, SubType: DerivesFrom<Type<'cid>> + ?Sized>(inner: &'ctx SubType, address_space: c_uint) -> &'ctx PointerType<'cid, SubType> {
        unsafe {
            &*(LLVMPointerType(upcast(inner).as_raw(), address_space) as *mut PointerType<SubType>)
        }
    }

    pub fn array<'ctx, SubType: DerivesFrom<Type<'cid>> + ?Sized>(inner: &'ctx SubType, count: c_uint) -> &'ctx ArrayType<'cid, SubType> {
        unsafe {
            &*(LLVMArrayType(upcast(inner).as_raw(), count) as *mut ArrayType<SubType>)
        }
    }

    pub fn function<'ctx>(params: &[&'ctx Type<'cid>], ret: &'ctx Type<'cid>, var_arg: bool) -> &'ctx FunctionType<'cid> {
        unsafe {
            &*(LLVMFunctionType(ret.as_raw(), params.as_ptr() as *mut LLVMTypeRef, params.len() as u32, var_arg as c_int) as *mut FunctionType)
        }
    }

    pub fn i1<'ctx>(context: &'ctx Context<'cid>) -> &'ctx IntegerType<'cid> {
        unsafe {
            &*(LLVMInt1TypeInContext(context.as_raw()) as *mut IntegerType)
        }
    }

    pub fn i8<'ctx>(context: &'ctx Context<'cid>) -> &'ctx IntegerType<'cid> {
        unsafe {
            &*(LLVMInt8TypeInContext(context.as_raw()) as *mut IntegerType)
        }
    }

    pub fn i16<'ctx>(context: &'ctx Context<'cid>) -> &'ctx IntegerType<'cid> {
        unsafe {
            &*(LLVMInt16TypeInContext(context.as_raw()) as *mut IntegerType)
        }
    }

    pub fn i32<'ctx>(context: &'ctx Context<'cid>) -> &'ctx IntegerType<'cid> {
        unsafe {
            &*(LLVMInt32TypeInContext(context.as_raw()) as *mut IntegerType)
        }
    }

    pub fn i64<'ctx>(context: &'ctx Context<'cid>) -> &'ctx IntegerType<'cid> {
        unsafe {
            &*(LLVMInt64TypeInContext(context.as_raw()) as *mut IntegerType)
        }
    }
}

impl<'cid> FunctionType<'cid> {
    pub fn is_var_arg(&self) -> bool {
        unsafe {
            LLVMIsFunctionVarArg(upcast::<_,Type>(self).as_raw()) != 0
        }
    }
}

impl<'cid, PointeeTy: ?Sized> PointerType<'cid, PointeeTy> {
    pub fn pointee_ty(&self) -> &PointeeTy {
        unsafe {
            &*transmute_copy::<_,*mut PointeeTy>(&LLVMGetElementType(upcast::<_,Type>(self).as_raw()))
        }
    }
}

impl<'cid, ElementTy: ?Sized> ArrayType<'cid, ElementTy> {
    pub fn element_ty(&self) -> &ElementTy {
        unsafe {
            &*transmute_copy::<_,*mut ElementTy>(&LLVMGetElementType(upcast::<_,Type>(self).as_raw()))
        }
    }
}
