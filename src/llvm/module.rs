use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use owned::{Owned, DropInPlace};

use llvm::{Context, Type, Function, FunctionLabel};

pub struct Module<'cid: 'context, 'context> {
    _context: PhantomData<&'context Context<'cid>>
}

impl<'cid, 'context> DropInPlace for Module<'cid, 'context> {
    unsafe fn drop_in_place(&mut self) {
        LLVMDisposeModule(self.as_raw());
    }
}

impl<'cid, 'context> Module<'cid, 'context> {
    pub fn new(name: &CStr, context: &'context Context<'cid>) -> Owned<Module<'cid, 'context>> {
        unsafe {
            Owned::from_raw(
                LLVMModuleCreateWithNameInContext(name.as_ptr(), context.as_raw()) as *mut Module
            )
        }
    }

    pub fn builder<'module>(&'module mut self) -> ModuleBuilder<'cid, 'context, 'module> {
        ModuleBuilder {
            inner: self
        }
    }

    pub fn as_raw(&self) -> LLVMModuleRef {
        self as *const Module as *mut Module as LLVMModuleRef
    }
}

pub struct ModuleBuilder<'cid: 'context, 'context: 'module, 'module> {
    inner: &'module mut Module<'cid, 'context>
}

impl<'cid, 'context, 'module> ModuleBuilder<'cid, 'context, 'module> {
    pub fn add_function(&mut self, name: &CStr, ty: &Type<'cid>) -> &'module mut Function<'cid> {
        unsafe {
            &mut *(LLVMAddFunction(self.inner.as_raw(), name.as_ptr(), ty.as_raw()) as *mut Function)
        }
    }

    pub fn get_named_function(&self, name: &CStr) -> Option<&'module FunctionLabel<'cid>> {
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
