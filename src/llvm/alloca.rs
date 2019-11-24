use llvm_sys::prelude::*;
use llvm_sys::core::*;

use libc::c_uint;

use inheritance::{upcast, DerivesFrom};

use llvm::Value;

pub struct Alloca<'cid, 'mid, 'fid> {
    _super: Value<'cid, 'mid, 'fid>
}
unsafe impl<'cid, 'mid, 'fid> DerivesFrom<Alloca<'cid, 'mid, 'fid>> for Alloca<'cid, 'mid, 'fid> { }
unsafe impl<'cid, 'mid, 'fid, General: ?Sized> DerivesFrom<General> for Alloca<'cid, 'mid, 'fid> where Value<'cid, 'mid, 'fid>: DerivesFrom<General> { }

impl<'cid, 'mid, 'fid> Alloca<'cid, 'mid, 'fid> {
    pub fn set_alignment(&mut self, alignment: c_uint) {
        unsafe {
            LLVMSetAlignment(self.as_raw(), alignment);
        }
    }

    pub fn downcast_value<'a>(value: &'a Value<'cid, 'mid, 'fid>) -> Result<&'a Alloca<'cid, 'mid, 'fid>, ()> {
        unsafe {
            let ret = LLVMIsAAllocaInst(value.as_raw());
            if ret.is_null() {
                Err(())
            } else {
                Ok(&*(ret as *mut Alloca))
            }
        }
    }

    pub fn as_value(&self) -> &Value<'cid, 'mid, 'fid> {
        upcast(self)
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.as_value().as_raw()
    }
}
