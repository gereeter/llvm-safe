use llvm_sys::prelude::*;
use llvm_sys::core::{LLVMSetInitializer, LLVMIsAGlobalVariable};

use std::marker::PhantomData;

use id::IdRef;
use inheritance::DerivesFrom;
use opaque::Opaque;

use llvm::{Constant, Value, PointerType};

pub struct Global<'cid, 'mid, Ty: ?Sized> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _type: PhantomData<Ty>,
    _opaque: Opaque
}
unsafe impl<'cid, 'mid, SpecificTy: DerivesFrom<GeneralTy> + ?Sized, GeneralTy: ?Sized> DerivesFrom<Global<'cid, 'mid, GeneralTy>> for Global<'cid, 'mid, SpecificTy> { }
unsafe impl<'cid, 'mid, 'fid, General: ?Sized, Ty: ?Sized> DerivesFrom<General> for Global<'cid, 'mid, Ty> where Value<'cid, 'mid, 'fid, PointerType<'cid, Ty>>: DerivesFrom<General> { }

impl<'cid, 'mid, Ty: ?Sized> Global<'cid, 'mid, Ty> {
    pub fn set_initializer(&mut self, value: &Constant<'cid, Ty>) {
        unsafe {
            LLVMSetInitializer(self.as_raw(), value.as_raw());
        }
    }

    pub fn downcast_value<'a, 'fid>(value: &'a Value<'cid, 'mid, 'fid, PointerType<'cid, Ty>>) -> Result<&'a Global<'cid, 'mid, Ty>, ()> {
        unsafe {
            let ret = LLVMIsAGlobalVariable(value.as_raw());
            if ret.is_null() {
                Err(())
            } else {
                Ok(&*(ret as *mut Global<Ty>))
            }
        }
    }

    pub fn as_value<'fid>(&self) -> &Value<'cid, 'mid, 'fid, PointerType<'cid, Ty>> {
        unsafe {
            &*(self as *const _ as *const Value<PointerType<Ty>>)
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.as_value().as_raw()
    }
}
