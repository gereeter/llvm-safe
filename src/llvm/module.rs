use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::Id;
use owned::{Owned, DropInPlace};

use llvm::{Context, Type, Function};

pub struct Module<'cid: 'context, 'context, 'mid> {
    _context: PhantomData<&'context Context<'cid>>,
    _id: Id<'mid>
}

impl<'cid, 'context, 'mid> DropInPlace for Module<'cid, 'context, 'mid> {
    unsafe fn drop_in_place(&mut self) {
        LLVMDisposeModule(self.as_raw());
    }
}

impl<'cid, 'context, 'mid> Module<'cid, 'context, 'mid> {
    pub fn new(_id: Id<'mid>, name: &CStr, context: &'context Context<'cid>) -> Owned<Module<'cid, 'context, 'mid>> {
        unsafe {
            Owned::from_raw(
                LLVMModuleCreateWithNameInContext(name.as_ptr(), context.as_raw()) as *mut Module
            )
        }
    }

    // TODO: use a ModuleBuilder?
    pub fn add_function<'module, 'fid>(&'module mut self, _id: Id<'fid>, name: &CStr, ty: &Type<'cid>) -> &'module mut Function<'cid, 'mid, 'fid> {
        unsafe {
            &mut *(LLVMAddFunction(self.as_raw(), name.as_ptr(), ty.as_raw()) as *mut Function)
        }
    }

    pub fn as_raw(&self) -> LLVMModuleRef {
        self as *const Module as *mut Module as LLVMModuleRef
    }
}
