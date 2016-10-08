use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::Id;
use owned::{Owned, DropInPlace};

use llvm::{Context, Type, Function, FunctionLabel, DataLayout};

pub struct Module<'cid: 'context, 'context, 'mid> {
    _id: Id<'mid>,
    _context: PhantomData<&'context Context<'cid>>
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

    pub fn set_data_layout(&mut self, layout: &DataLayout) {
        unsafe {
            // TODO(3.9): Use LLVMSetModuleDataLayout
            LLVMSetDataLayout(self.as_raw(), layout.as_string().as_ptr());
        }
    }

    pub fn set_target_triple(&mut self, triple: &CStr) {
        unsafe {
            LLVMSetTarget(self.as_raw(), triple.as_ptr());
        }
    }

    pub fn builder<'module>(&'module mut self) -> ModuleBuilder<'cid, 'context, 'mid, 'module> {
        ModuleBuilder {
            inner: self
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
    pub fn add_function(&mut self, name: &CStr, ty: &Type<'cid>) -> &'module mut Function<'cid, 'mid> {
        unsafe {
            &mut *(LLVMAddFunction(self.inner.as_raw(), name.as_ptr(), ty.as_raw()) as *mut Function)
        }
    }

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
