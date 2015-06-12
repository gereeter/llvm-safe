use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::{Id, IdRef};

use llvm::{Context, Type, Function};

pub struct Module<'cid: 'context, 'context, 'mid> {
    _context: PhantomData<&'context Context<'cid>>,
    _id: Id<'mid>,
    llvm_module: LLVMModuleRef
}

impl<'cid, 'context, 'mid> Drop for Module<'cid, 'context, 'mid> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.llvm_module);
        }
    }
}

impl<'cid, 'context, 'mid> Module<'cid, 'context, 'mid> {
    pub fn new(id: Id<'mid>, name: &CStr, context: &'context Context<'cid>) -> Module<'cid, 'context, 'mid> {
        Module {
            _context: PhantomData,
            _id: id,
            llvm_module: unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), context.as_raw()) }
        }
    }

    pub fn add_function<'module, 'fid>(&'module self, id: Id<'fid>, name: &CStr, ty: Type<'cid>) -> Function<'cid, 'mid, 'module, 'fid> {
        Function {
            _context_id: IdRef::new(),
            _module_id: IdRef::new(),
            _module: PhantomData,
            _id: id,
            llvm_function: unsafe { LLVMAddFunction(self.llvm_module, name.as_ptr(), ty.as_raw()) }
        }
    }
}
