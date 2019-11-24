use llvm_sys::prelude::*;
use llvm_sys::core::*;

use libc::{c_char, c_uint, c_int};

use id::IdRef;
use inheritance::{upcast, DerivesFrom};
use opaque::Opaque;

use llvm::{Context, Type, IntegerType, PointerType, Value};

pub struct Constant<'cid> {
    _context: IdRef<'cid>,
    _opaque: Opaque
}
unsafe impl<'cid> DerivesFrom<Constant<'cid>> for Constant<'cid> { }
unsafe impl<'cid, 'mid, 'fid, General: ?Sized> DerivesFrom<General> for Constant<'cid> where Value<'cid, 'mid, 'fid>: DerivesFrom<General> { }

impl<'cid> Constant<'cid> {
    pub fn bool<'ctx>(value: bool, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid> {
        Constant::integer(value as u64, Type::i1(context), false)
    }

    pub fn i8<'ctx>(value: i8, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid> {
        Constant::integer(value as u64, Type::i8(context), false)
    }

    pub fn i32<'ctx>(value: i32, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid> {
        Constant::integer(value as u64, Type::i32(context), false)
    }

    pub fn integer<'ctx>(value: u64, ty: &'ctx IntegerType<'cid>, signed: bool) -> &'ctx Constant<'cid> {
        unsafe {
            &*(LLVMConstInt(upcast::<_,Type>(ty).as_raw(), value, signed as c_int) as *mut Constant)
        }
    }

    pub fn f64<'ctx>(value: f64, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid> {
        unsafe {
            &*(LLVMConstReal(Type::f64(context).as_raw(), value) as *mut Constant)
        }
    }

    pub fn null<'ctx>(ty: &'ctx Type<'cid>) -> &'ctx Constant<'cid> {
        unsafe {
            &*(LLVMConstNull(ty.as_raw()) as *mut Constant)
        }
    }

    pub fn null_pointer<'ctx, PointeeTy: ?Sized>(ty: &'ctx PointerType<'cid, PointeeTy>) -> &'ctx Constant<'cid> {
        unsafe {
            &*(LLVMConstPointerNull(upcast::<_,Type>(ty).as_raw()) as *mut Constant)
        }
    }

    pub fn string<'ctx>(data: &[u8], no_null_terminated: bool, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid> {
        unsafe {
            &*(LLVMConstStringInContext(context.as_raw(), data.as_ptr() as *const c_char, data.len() as c_uint, no_null_terminated as c_int) as *mut Constant)
        }
    }

    pub fn downcast_value<'a, 'mid, 'fid>(value: &'a Value<'cid, 'mid, 'fid>) -> Result<&'a Constant<'cid>, ()> {
        unsafe {
            if LLVMIsConstant(value.as_raw()) != 0 {
                Ok(&*(value.as_raw() as *mut Constant))
            } else {
                Err(())
            }
        }
    }

    pub fn as_value<'mid, 'fid>(&self) -> &Value<'cid, 'mid, 'fid> {
        upcast(self)
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.as_value().as_raw()
    }
}
