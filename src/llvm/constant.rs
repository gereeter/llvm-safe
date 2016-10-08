use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::IdRef;

use llvm::{Context, Type, Value};

pub struct Constant<'cid> {
    _context: IdRef<'cid>
}

impl<'cid> Constant<'cid> {
    pub fn i32<'ctx>(value: i32, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid> {
        unsafe {
            &*(LLVMConstInt(Type::i32(context).as_raw(), value as u64, 0) as *mut Constant)
        }
    }

    pub fn f64<'ctx>(value: f64, context: &'ctx Context<'cid>) -> &'ctx Constant<'cid> {
        unsafe {
            &*(LLVMConstReal(Type::f64(context).as_raw(), value) as *mut Constant)
        }
    }

    pub fn as_value<'mid, 'fid>(&self) -> &Value<'cid, 'mid, 'fid> {
        unsafe {
            &*(self.as_raw() as *mut Value)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Constant as *mut Constant as LLVMValueRef
    }
}
