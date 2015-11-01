use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::Id;
use owned::{Owned, DropInPlace};

pub struct Context<'cid> {
    _id: Id<'cid>
}

impl<'cid> DropInPlace for Context<'cid> {
   unsafe fn drop_in_place(&mut self) {
       LLVMContextDispose(self.as_raw());
   }
}

impl<'cid> Context<'cid> {
    pub fn new(_id: Id<'cid>) -> Owned<Context<'cid>> {
        unsafe {
            Owned::from_raw(
                LLVMContextCreate() as *mut Context
            )
        }
    }

    pub fn as_raw(&self) -> LLVMContextRef {
        self as *const Context as *mut Context as LLVMContextRef
    }
}
