use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::{Id, IdRef};
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

    pub fn builder<'module>(&'module mut self) -> ModuleBuilder<'cid, 'mid, 'module> {
        ModuleBuilder {
            inner: self.as_raw(),
            _marker: PhantomData,
            _module_id: IdRef::new(),
            _context_id: IdRef::new()
        }
    }

    pub fn as_raw(&self) -> LLVMModuleRef {
        self as *const Module as *mut Module as LLVMModuleRef
    }
}

pub struct ModuleBuilder<'cid: 'module, 'mid: 'module, 'module> {
    inner: LLVMModuleRef,
    _marker: PhantomData<&'module mut ()>,
    _module_id: IdRef<'mid>,
    _context_id: IdRef<'cid>
}

impl<'cid, 'mid, 'module> ModuleBuilder<'cid, 'mid, 'module> {
    pub fn add_function(&mut self, name: &CStr, ty: &Type<'cid>) -> &'module mut Function<'cid, 'mid> {
        unsafe {
            &mut *(LLVMAddFunction(self.inner, name.as_ptr(), ty.as_raw()) as *mut Function)
        }
    }

    pub fn get_named_function(&self, name: &CStr) -> Option<&'module FunctionLabel<'cid, 'mid>> {
        unsafe {
            let old = LLVMGetNamedFunction(self.inner, name.as_ptr());
            if old.is_null() {
                None
            } else {
                Some(&*(old as *mut FunctionLabel))
            }
        }
    }

    pub fn reborrow<'a>(&'a mut self) -> ModuleBuilder<'cid, 'mid, 'a> {
        ModuleBuilder {
            inner: self.inner,
            _marker: PhantomData,
            _module_id: IdRef::new(),
            _context_id: IdRef::new()
        }
    }
}
