use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::Id;

pub struct Context<'cid> {
    _id: Id<'cid>,
    llvm_context: LLVMContextRef
}

impl<'cid> Drop for Context<'cid> {
   fn drop(&mut self) {
       unsafe {
           LLVMContextDispose(self.llvm_context);
       }
   }
}

impl<'cid> Context<'cid> {
    pub fn new(id: Id<'cid>) -> Context<'cid> {
        Context {
            _id: id,
            llvm_context: unsafe { LLVMContextCreate() }
        }
    }

    pub fn as_raw(&self) -> LLVMContextRef {
        self.llvm_context
    }
}
