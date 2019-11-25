use llvm_sys::prelude::*;
use llvm_sys::core::*;

use std::marker::PhantomData;

use id::IdRef;
use inheritance::DerivesFrom;
use opaque::Opaque;

pub struct Value<'cid, 'mid, 'fid, Ty: ?Sized> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _function_id: IdRef<'fid>,
    _type: PhantomData<Ty>,
    _opaque: Opaque
}
unsafe impl<'cid, 'mid, 'fid, SpecificTy: DerivesFrom<GeneralTy> + ?Sized, GeneralTy: ?Sized> DerivesFrom<Value<'cid, 'mid, 'fid, GeneralTy>> for Value<'cid, 'mid, 'fid, SpecificTy> { }

impl<'cid, 'mid, 'fid, Ty: ?Sized> Value<'cid, 'mid, 'fid, Ty> {
    // FIXME: Should this require a mutable reference?
    pub fn set_name(&self, name: &str) {
        unsafe {
            LLVMSetValueName2(self.as_raw(), name.as_ptr() as *const std::os::raw::c_char, name.len());
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpValue(self.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self as *const Value<Ty> as *mut Value<Ty> as LLVMValueRef
    }
}
