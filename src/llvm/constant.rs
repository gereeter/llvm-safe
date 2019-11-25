use llvm_sys::prelude::*;
use llvm_sys::core::*;

use std::marker::PhantomData;
use libc::{c_char, c_uint, c_int};

use id::IdRef;
use inheritance::{upcast, DerivesFrom};
use opaque::Opaque;

use llvm::{Context, Type, IntegerType, PointerType, ArrayType, Value};

pub struct Constant<'cid, Ty: ?Sized> {
    _context: IdRef<'cid>,
    _type: PhantomData<Ty>,
    _opaque: Opaque
}
unsafe impl<'cid, SpecificTy: DerivesFrom<GeneralTy> + ?Sized, GeneralTy: ?Sized> DerivesFrom<Constant<'cid, GeneralTy>> for Constant<'cid, SpecificTy> { }
unsafe impl<'cid, 'mid, 'fid, General: ?Sized, Ty: ?Sized> DerivesFrom<General> for Constant<'cid, Ty> where Value<'cid, 'mid, 'fid, Ty>: DerivesFrom<General> { }

impl<'cid> Constant<'cid, IntegerType<'cid>> {
    pub fn bool<'ctx>(value: bool, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid, IntegerType<'cid>> {
        Constant::integer(value as u64, Type::i1(context), false)
    }

    pub fn i8<'ctx>(value: i8, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid, IntegerType<'cid>> {
        Constant::integer(value as u64, Type::i8(context), false)
    }

    pub fn i32<'ctx>(value: i32, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid, IntegerType<'cid>> {
        Constant::integer(value as u64, Type::i32(context), false)
    }

    pub fn integer<'ctx>(value: u64, ty: &'ctx IntegerType<'cid>, signed: bool) -> &'ctx Constant<'cid, IntegerType<'cid>> {
        unsafe {
            &*(LLVMConstInt(upcast::<_,Type>(ty).as_raw(), value, signed as c_int) as *mut Constant<IntegerType>)
        }
    }
}

impl<'cid> Constant<'cid, Type<'cid>> {
    pub fn f64<'ctx>(value: f64, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid, Type<'cid>> {
        unsafe {
            &*(LLVMConstReal(Type::f64(context).as_raw(), value) as *mut Constant<Type>)
        }
    }
}

impl<'cid, Ty: ?Sized> Constant<'cid, Ty> {
    pub fn null<'ctx>(ty: &'ctx Ty) -> &'ctx Constant<'cid, Ty> where Ty: DerivesFrom<Type<'cid>> {
        unsafe {
            &*(LLVMConstNull(upcast(ty).as_raw()) as *mut Constant<Ty>)
        }
    }
}

impl<'cid, Ty: ?Sized> Constant<'cid, PointerType<'cid, Ty>> {
    pub fn null_pointer<'ctx>(ty: &'ctx PointerType<'cid, Ty>) -> &'ctx Constant<'cid, PointerType<'cid, Ty>> {
        unsafe {
            &*(LLVMConstPointerNull(upcast::<_,Type>(ty).as_raw()) as *mut Constant<PointerType<Ty>>)
        }
    }
}

impl<'cid> Constant<'cid, ArrayType<'cid, IntegerType<'cid>>> {
    pub fn string<'ctx>(data: &[u8], no_null_terminated: bool, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid, ArrayType<'cid, IntegerType<'cid>>> {
        unsafe {
            &*(LLVMConstStringInContext(context.as_raw(), data.as_ptr() as *const c_char, data.len() as c_uint, no_null_terminated as c_int) as *mut Constant<ArrayType<IntegerType>>)
        }
    }
}

impl<'cid, Ty: ?Sized> Constant<'cid, Ty> {
    pub fn downcast_value<'a, 'mid, 'fid>(value: &'a Value<'cid, 'mid, 'fid, Ty>) -> Result<&'a Constant<'cid, Ty>, ()> {
        unsafe {
            if LLVMIsConstant(value.as_raw()) != 0 {
                Ok(&*(value.as_raw() as *mut Constant<Ty>))
            } else {
                Err(())
            }
        }
    }

    pub fn as_value<'mid, 'fid>(&self) -> &Value<'cid, 'mid, 'fid, Ty> {
        unsafe {
            &*(self as *const _ as *const Value<Ty>)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.as_value().as_raw()
    }
}
