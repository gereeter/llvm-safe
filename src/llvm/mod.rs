use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::prelude::*;
use llvm_sys::core::*;

use id::{Id, IdRef};

pub use self::context::Context;

pub mod context;



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

pub struct Module<'cid: 'context, 'context, 'mid> {
    _context: PhantomData<&'context Context<'cid>>,
    _id: Id<'mid>,
    llvm_module: LLVMModuleRef
}

impl<'cid, 'context, 'mid> Drop for Module<'cid, 'context, 'mid> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.llvm_module);
        }
    }
}

impl<'cid, 'context, 'mid> Module<'cid, 'context, 'mid> {
    pub fn new(id: Id<'mid>, name: &CStr, context: &'context Context<'cid>) -> Module<'cid, 'context, 'mid> {
        Module {
            _context: PhantomData,
            _id: id,
            llvm_module: unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), context.as_raw()) }
        }
    }

    pub fn add_function<'module, 'fid>(&'module self, id: Id<'fid>, name: &CStr, ty: Type<'cid>) -> Function<'cid, 'mid, 'module, 'fid> {
        Function {
            _context_id: IdRef::new(),
            _module_id: IdRef::new(),
            _module: PhantomData,
            _id: id,
            llvm_function: unsafe { LLVMAddFunction(self.llvm_module, name.as_ptr(), ty.llvm_type) }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Type<'cid> {
    _context_id: IdRef<'cid>,
    llvm_type: LLVMTypeRef
}

impl<'cid> Type<'cid> {
    pub fn f64(context: &Context<'cid>) -> Type<'cid> {
       Type {
           _context_id: IdRef::new(),
           llvm_type: unsafe { LLVMDoubleTypeInContext(context.as_raw()) }
       }
    }

    pub fn function(params: &[Type<'cid>], ret: Type<'cid>) -> Type<'cid> {
        Type {
            _context_id: IdRef::new(),
            // FIXME: actually enforce that Type and LLVMTypeRef are the same in memory
            llvm_type: unsafe { LLVMFunctionType(ret.llvm_type, params.as_ptr() as *mut LLVMTypeRef, params.len() as u32, 0) }
        }
    }
}

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
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMBuildBr(self.llvm_builder, target.llvm_basic_block) }
        }
    }

    pub fn or(&self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMBuildOr(self.llvm_builder, lhs.llvm_value, rhs.llvm_value, name.as_ptr()) }
        }
    }

    pub fn add(&self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMBuildAdd(self.llvm_builder, lhs.llvm_value, rhs.llvm_value, name.as_ptr()) }
        }
    }

    pub fn and(&self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMBuildAnd(self.llvm_builder, lhs.llvm_value, rhs.llvm_value, name.as_ptr()) }
        }
    }

    pub fn mul(&self, lhs: Value<'cid, 'fid, 'function>, rhs: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMBuildMul(self.llvm_builder, lhs.llvm_value, rhs.llvm_value, name.as_ptr()) }
        }
    }

    pub fn neg(&self, value: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMBuildNeg(self.llvm_builder, value.llvm_value, name.as_ptr()) }
        }
    }

    pub fn not(&self, value: Value<'cid, 'fid, 'function>, name: &CStr) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMBuildNot(self.llvm_builder, value.llvm_value, name.as_ptr()) }
        }
    }

    pub fn ret(&self, value: Value<'cid, 'fid, 'function>) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMBuildRet(self.llvm_builder, value.llvm_value) }
        }
    }
}

pub struct Function<'cid, 'mid: 'module, 'module, 'fid> {
    _context_id: IdRef<'cid>,
    _module_id: IdRef<'mid>,
    _module: PhantomData<&'module ()>,
    _id: Id<'fid>,
    llvm_function: LLVMValueRef
}

impl<'cid, 'mid: 'module, 'module, 'fid> Function<'cid, 'mid, 'module, 'fid> {
    pub fn append_basic_block<'function>(&'function self, name: &CStr, context: &Context<'cid>) -> BasicBlock<'cid, 'fid, 'function> {
        BasicBlock {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_basic_block: unsafe { LLVMAppendBasicBlockInContext(context.as_raw(), self.llvm_function, name.as_ptr()) }
        }
    }

    pub fn param<'function>(&'function self, index: u32) -> Value<'cid, 'fid, 'function> {
        Value {
            _context_id: IdRef::new(),
            _function_id: IdRef::new(),
            _function: PhantomData,
            llvm_value: unsafe { LLVMGetParam(self.llvm_function, index) }
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpValue(self.llvm_function);
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

#[derive(Copy, Clone)]
pub struct Value<'cid, 'fid, 'function> {
    _context_id: IdRef<'cid>,
    _function_id: IdRef<'fid>,
    _function: PhantomData<&'function ()>,
    llvm_value: LLVMValueRef
}
