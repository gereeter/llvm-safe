use llvm_sys::prelude::*;
use llvm_sys::core::*;

use libc::c_uint;

use id::IdRef;

use llvm::context::Context;

pub struct Type<'cid> {
    _context_id: IdRef<'cid>
}

impl<'cid> Type<'cid> {
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

    pub fn i1<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMInt1TypeInContext(context.as_raw()) as *mut Type)
        }
    }

    pub fn i8<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMInt8TypeInContext(context.as_raw()) as *mut Type)
        }
    }

    pub fn i16<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMInt16TypeInContext(context.as_raw()) as *mut Type)
        }
    }

    pub fn i32<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMInt32TypeInContext(context.as_raw()) as *mut Type)
        }
    }

    pub fn i64<'ctx>(context: &'ctx Context<'cid>) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMInt64TypeInContext(context.as_raw()) as *mut Type)
        }
    }

    pub fn pointer<'ctx>(inner: &'ctx Type<'cid>, address_space: c_uint) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMPointerType(inner.as_raw(), address_space) as *mut Type)
        }
    }

    pub fn function<'ctx>(params: &[&'ctx Type<'cid>], ret: &'ctx Type<'cid>) -> &'ctx Type<'cid> {
        unsafe {
            &*(LLVMFunctionType(ret.as_raw(), params.as_ptr() as *mut LLVMTypeRef, params.len() as u32, 0) as *mut Type)
        }
    }

    pub fn as_raw(&self) -> LLVMTypeRef {
        self as *const Type as *mut Type as LLVMTypeRef
    }
}
