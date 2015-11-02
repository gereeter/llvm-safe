use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::Id;
use owned::{Owned, DropInPlace};

use llvm::{Context, Type, Function, FunctionLabel};

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

    pub fn builder<'module>(&'module mut self) -> ModuleBuilder<'cid, 'context, 'mid, 'module> {
        ModuleBuilder {
            inner: self
        }
    }

    // TODO: eww
    pub fn get_named_function_mut<'module, 'fid: 'module>(&'module mut self, id: Id<'fid>, name: &CStr) -> Result<(&'module mut Function<'cid, 'mid, 'fid>, ModuleBuilder<'cid, 'context, 'mid, 'module>), (Id<'fid>, &'module mut Module<'cid, 'context, 'mid>)> {
        unsafe {
            let old = LLVMGetNamedFunction(self.as_raw(), name.as_ptr());
            if old.is_null() {
                Err((id, self))
            } else {
                Ok((&mut *(old as *mut Function), self.builder()))
            }
        }
    }

    pub fn as_raw(&self) -> LLVMModuleRef {
        self as *const Module as *mut Module as LLVMModuleRef
    }
}

pub struct ModuleBuilder<'cid: 'context, 'context: 'module, 'mid: 'module, 'module> {
    inner: &'module mut Module<'cid, 'context, 'mid>
}

impl<'cid, 'context, 'mid, 'module> ModuleBuilder<'cid, 'context, 'mid, 'module> {
    pub fn add_function<'fid>(&mut self, _id: Id<'fid>, name: &CStr, ty: &Type<'cid>) -> &'module mut Function<'cid, 'mid, 'fid> {
        unsafe {
            &mut *(LLVMAddFunction(self.inner.as_raw(), name.as_ptr(), ty.as_raw()) as *mut Function)
        }
    }

    // TODO: eww
    pub fn get_named_function(&self, name: &CStr) -> Option<&'module FunctionLabel<'cid, 'mid>> {
        unsafe {
            let old = LLVMGetNamedFunction(self.inner.as_raw(), name.as_ptr());
            if old.is_null() {
                None
            } else {
                Some(&*(old as *mut FunctionLabel))
            }
        }
    }
}
