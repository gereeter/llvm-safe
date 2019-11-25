use llvm_sys::prelude::*;
use llvm_sys::core::*;

use inheritance::DerivesFrom;

use llvm::{Label, Value};

pub struct Phi<'cid, 'mid, 'fid, Ty: ?Sized> {
    _super: Value<'cid, 'mid, 'fid, Ty>
}
unsafe impl<'cid, 'mid, 'fid, SpecificTy: DerivesFrom<GeneralTy> + ?Sized, GeneralTy: ?Sized> DerivesFrom<Phi<'cid, 'mid, 'fid, GeneralTy>> for Phi<'cid, 'mid, 'fid, SpecificTy> { }
unsafe impl<'cid, 'mid, 'fid, General: ?Sized, Ty: ?Sized> DerivesFrom<General> for Phi<'cid, 'mid, 'fid, Ty> where Value<'cid, 'mid, 'fid, Ty>: DerivesFrom<General> { }

impl<'cid, 'mid, 'fid, Ty: ?Sized> Phi<'cid, 'mid, 'fid, Ty> {
    // TODO: Expose bulk addition?
    pub fn add_incoming_branch(&mut self, value: &Value<'cid, 'mid, 'fid, Ty>, block: &Label<'fid>) {
        unsafe {
            LLVMAddIncoming(self.as_raw(), [value.as_raw()].as_mut_ptr(), [block.as_raw()].as_mut_ptr(), 1);
        }
    }

    pub fn downcast_value<'a>(value: &'a Value<'cid, 'mid, 'fid, Ty>) -> Result<&'a Phi<'cid, 'mid, 'fid, Ty>, ()> {
        unsafe {
            let ret = LLVMIsAPHINode(value.as_raw());
            if ret.is_null() {
                Err(())
            } else {
                Ok(&*(ret as *mut Phi<Ty>))
            }
        }
    }

    pub fn as_value(&self) -> &Value<'cid, 'mid, 'fid, Ty> {
        unsafe {
            &*(self as *const _ as *const Value<Ty>)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.as_value().as_raw()
    }
}
