use llvm_sys::prelude::*;
use llvm_sys::core::*;

use libc::c_uint;

use inheritance::DerivesFrom;

use llvm::Value;
use llvm::PointerType;

pub struct Alloca<'cid, 'mid, 'fid, Ty: ?Sized> {
    _super: Value<'cid, 'mid, 'fid, PointerType<'cid, Ty>>
}
unsafe impl<'cid, 'mid, 'fid, SpecificTy: DerivesFrom<GeneralTy> + ?Sized, GeneralTy: ?Sized> DerivesFrom<Alloca<'cid, 'mid, 'fid, GeneralTy>> for Alloca<'cid, 'mid, 'fid, SpecificTy> { }
unsafe impl<'cid, 'mid, 'fid, Ty: ?Sized, General: ?Sized> DerivesFrom<General> for Alloca<'cid, 'mid, 'fid, Ty> where Value<'cid, 'mid, 'fid, PointerType<'cid, Ty>>: DerivesFrom<General> { }

impl<'cid, 'mid, 'fid, Ty: ?Sized> Alloca<'cid, 'mid, 'fid, Ty> {
    pub fn set_alignment(&mut self, alignment: c_uint) {
        unsafe {
            LLVMSetAlignment(self.as_raw(), alignment);
        }
    }

    pub fn downcast_value<'a>(value: &'a Value<'cid, 'mid, 'fid, PointerType<'cid, Ty>>) -> Result<&'a Alloca<'cid, 'mid, 'fid, Ty>, ()> {
        unsafe {
            let ret = LLVMIsAAllocaInst(value.as_raw());
            if ret.is_null() {
                Err(())
            } else {
                Ok(&*(ret as *mut Alloca<Ty>))
            }
        }
    }

    pub fn as_value(&self) -> &Value<'cid, 'mid, 'fid, PointerType<'cid, Ty>> {
        unsafe {
            &*(self as *const _ as *const Value<PointerType<Ty>>)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.as_value().as_raw()
    }
}
