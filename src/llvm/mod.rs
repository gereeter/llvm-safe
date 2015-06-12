use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::{Id, IdRef};

pub use self::context::Context;
pub use self::module::Module;
pub use self::function::Function;

pub use self::value::Value;
pub use self::ty::Type;

pub mod context;
pub mod module;
pub mod function;

pub mod value;
pub mod ty;


//
// TODO: Error Checking
//   This is currently a safety hole.
//
// TODO: Reduce the number of lifetimes.
//   It isn't completely clear that this is possible, but this is a first attempt.
//
// TODO: Docs
//
// TODO: See what safety guarentees LLVM actually needs.
//   The official LLVM docs aren't very clear on, e.g., when different contexts/modules/function/builders
//   can be mixed, or what things can outlive other things.
//
// TODO: Consider wrapping the C++ API in another way.
//   The official wrapper is useful, since it requires no effort and is maintained by LLVM. However,
//   it has a number of annoying issues. For example, given the similarity between LLVM's Twine and Rust's
//   &str, there should be no reason to use CStr.
//
// TODO: Missing functionality
//

pub struct Builder<'cid> {
    _context_id: IdRef<'cid>,
    llvm_builder: LLVMBuilderRef
}

impl<'cid> Drop for Builder<'cid> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.llvm_builder);
        }
    }
}

impl<'cid> Builder<'cid> {
    pub fn new(context: &Context<'cid>) -> Builder<'cid> {
        Builder {
            _context_id: IdRef::new(),
            llvm_builder: unsafe { LLVMCreateBuilderInContext(context.as_raw()) }
        }
    }

    pub fn position_at_end<'fid, 'function, 'builder>(&'builder mut self, block: BasicBlock<'cid, 'fid, 'function>) -> PositionedBuilder<'cid, 'fid, 'function, 'builder> {
        unsafe {
            LLVMPositionBuilderAtEnd(self.llvm_builder, block.llvm_basic_block);
        }
        PositionedBuilder {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            _builder: PhantomData,
            llvm_builder: self.llvm_builder
        }
    }
}

pub struct PositionedBuilder<'cid: 'builder, 'fid, 'function, 'builder> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>,
    _function: PhantomData<&'function ()>,
    _builder: PhantomData<&'builder mut Builder<'cid>>,
    llvm_builder: LLVMBuilderRef
}

impl<'cid: 'builder, 'fid, 'function, 'builder> PositionedBuilder<'cid, 'fid, 'function, 'builder> {
    pub fn br(&self, target: BasicBlock<'cid, 'fid, 'function>) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildBr(self.llvm_builder, target.llvm_basic_block))
        }
    }

    pub fn or(&self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildOr(self.llvm_builder, lhs.as_raw(), rhs.as_raw(), name.as_ptr()))
        }
    }

    pub fn add(&self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildAdd(self.llvm_builder, lhs.as_raw(), rhs.as_raw(), name.as_ptr()))
        }
    }

    pub fn and(&self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildAnd(self.llvm_builder, lhs.as_raw(), rhs.as_raw(), name.as_ptr()))
        }
    }

    pub fn mul(&self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildMul(self.llvm_builder, lhs.as_raw(), rhs.as_raw(), name.as_ptr()))
        }
    }

    pub fn neg(&self, value: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildNeg(self.llvm_builder, value.as_raw(), name.as_ptr()))
        }
    }

    pub fn not(&self, value: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildNot(self.llvm_builder, value.as_raw(), name.as_ptr()))
        }
    }

    pub fn ret(&self, value: Value<'cid, 'fid, 'function>) -> Value<'cid, 'fid, 'function> {
        unsafe {
            Value::from_raw(LLVMBuildRet(self.llvm_builder, value.as_raw()))
        }
    }
}

#[derive(Copy, Clone)]
pub struct BasicBlock<'cid, 'fid, 'function> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>,
    _function: PhantomData<&'function ()>,
    llvm_basic_block: LLVMBasicBlockRef
}
