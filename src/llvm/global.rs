use llvm_sys::prelude::*;
use llvm_sys::core::LLVMSetInitializer;

use id::IdRef;
use inheritance::{upcast, DerivesFrom};
use opaque::Opaque;

use llvm::{Constant, Value};

pub struct Global<'cid, 'mid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _opaque: Opaque
}
unsafe impl<'cid, 'mid> DerivesFrom<Global<'cid, 'mid>> for Global<'cid, 'mid> { }
unsafe impl<'cid, 'mid, 'fid, General: ?Sized> DerivesFrom<General> for Global<'cid, 'mid> where Value<'cid, 'mid, 'fid>: DerivesFrom<General> { }

impl<'cid, 'mid> Global<'cid, 'mid> {
    pub fn set_initializer(&mut self, value: &Constant<'cid>) {
        unsafe {
            LLVMSetInitializer(self.as_raw(), value.as_raw());
        }
    }

    pub fn as_value<'fid>(&self) -> &Value<'cid, 'mid, 'fid> {
        upcast(self)
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.as_value().as_raw()
    }
}
