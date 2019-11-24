use llvm_sys::prelude::*;
use llvm_sys::core::*;

use inheritance::{upcast, DerivesFrom};

use llvm::{Label, Value};

pub struct Phi<'cid, 'mid, 'fid> {
    _super: Value<'cid, 'mid, 'fid>
}
unsafe impl<'cid, 'mid, 'fid> DerivesFrom<Phi<'cid, 'mid, 'fid>> for Phi<'cid, 'mid, 'fid> { }
unsafe impl<'cid, 'mid, 'fid, General: ?Sized> DerivesFrom<General> for Phi<'cid, 'mid, 'fid> where Value<'cid, 'mid, 'fid>: DerivesFrom<General> { }

impl<'cid, 'mid, 'fid> Phi<'cid, 'mid, 'fid> {
    // TODO: Expose bulk addition?
    pub fn add_incoming_branch(&mut self, value: &Value<'cid, 'mid, 'fid>, block: &Label<'fid>) {
        unsafe {
            LLVMAddIncoming(self.as_raw(), [value.as_raw()].as_mut_ptr(), [block.as_raw()].as_mut_ptr(), 1);
        }
    }

    pub fn downcast_value<'a>(value: &'a Value<'cid, 'mid, 'fid>) -> Result<&'a Phi<'cid, 'mid, 'fid>, ()> {
        unsafe {
            let ret = LLVMIsAPHINode(value.as_raw());
            if ret.is_null() {
                Err(())
            } else {
                Ok(&*(ret as *mut Phi))
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
