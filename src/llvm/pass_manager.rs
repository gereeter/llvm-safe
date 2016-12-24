use std::marker::PhantomData;

use id::IdRef;
use owned::{DropInPlace, Owned};

use llvm_sys::prelude::LLVMPassManagerRef;
use llvm_sys::core::{LLVMCreateFunctionPassManagerForModule, LLVMDisposePassManager};
use llvm_sys::core::{LLVMInitializeFunctionPassManager, LLVMFinalizeFunctionPassManager};
use llvm_sys::core::LLVMRunFunctionPassManager;
use llvm_sys::transforms::scalar::{
    LLVMAddAggressiveDCEPass,
    LLVMAddBasicAliasAnalysisPass,
    LLVMAddGVNPass,
    LLVMAddInstructionCombiningPass,
    LLVMAddMemCpyOptPass,
    LLVMAddPromoteMemoryToRegisterPass,
    LLVMAddReassociatePass,
    LLVMAddScalarReplAggregatesPass,
    LLVMAddCFGSimplificationPass
};

use llvm::{Function, Module, ModuleBuilder};

pub struct FunctionPassManager<'mid> {
    _module_id: IdRef<'mid>
}

impl<'mid> DropInPlace for FunctionPassManager<'mid> {
    unsafe fn drop_in_place(&mut self) {
        LLVMDisposePassManager(self.as_raw());
    }
}

impl<'mid> FunctionPassManager<'mid> {
    pub fn new<'cid, 'context>(module: &Module<'cid, 'context, 'mid>) -> Owned<FunctionPassManager<'mid>> {
        unsafe {
            Owned::from_raw(LLVMCreateFunctionPassManagerForModule(module.as_raw()) as *mut FunctionPassManager)
        }
    }

    pub fn add_aggressive_dce(&mut self) {
        unsafe {
            LLVMAddAggressiveDCEPass(self.as_raw());
        }
    }

    pub fn add_basic_alias_analysis(&mut self) {
        unsafe {
            LLVMAddBasicAliasAnalysisPass(self.as_raw());
        }
    }

    pub fn add_gvn(&mut self) {
        unsafe {
            LLVMAddGVNPass(self.as_raw());
        }
    }

    pub fn add_instruction_combine(&mut self) {
        unsafe {
            LLVMAddInstructionCombiningPass(self.as_raw());
        }
    }

    pub fn add_mem_cpy_opt(&mut self) {
        unsafe {
            LLVMAddMemCpyOptPass(self.as_raw());
        }
    }

    pub fn add_memory_to_register(&mut self) {
        unsafe {
            LLVMAddPromoteMemoryToRegisterPass(self.as_raw());
        }
    }

    pub fn add_reassociate(&mut self) {
        unsafe {
            LLVMAddReassociatePass(self.as_raw());
        }
    }

    pub fn add_scalar_repl_aggregates(&mut self) {
        unsafe {
            LLVMAddScalarReplAggregatesPass(self.as_raw());
        }
    }

    pub fn add_simplify_cfg(&mut self) {
        unsafe {
            LLVMAddCFGSimplificationPass(self.as_raw());
        }
    }

    pub fn initialize<'fpm, 'cid, 'module>(&'fpm mut self, _module: &ModuleBuilder<'cid, 'mid, 'module>) -> Owned<InitializedFunctionPassManager<'mid, 'module, 'fpm>> {
        unsafe {
            LLVMInitializeFunctionPassManager(self.as_raw());
            Owned::from_raw(self.as_raw() as *mut InitializedFunctionPassManager)
        }
    }

    pub fn as_raw(&self) -> LLVMPassManagerRef {
        self as *const FunctionPassManager as *mut FunctionPassManager as LLVMPassManagerRef
    }
}

pub struct InitializedFunctionPassManager<'mid: 'module + 'fpm, 'module, 'fpm> {
    _marker: PhantomData<(&'module mut Module<'static, 'static, 'mid>, &'fpm mut FunctionPassManager<'mid>)>
}

impl<'mid, 'module, 'fpm> DropInPlace for InitializedFunctionPassManager<'mid, 'module, 'fpm> {
    unsafe fn drop_in_place(&mut self) {
        LLVMFinalizeFunctionPassManager(self.as_raw());
    }
}

impl<'mid, 'module, 'fpm> InitializedFunctionPassManager<'mid, 'module, 'fpm> {
    pub fn run<'cid>(&mut self, function: &mut Function<'cid, 'mid>) {
        unsafe {
            LLVMRunFunctionPassManager(self.as_raw(), function.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMPassManagerRef {
        self as *const InitializedFunctionPassManager as *mut InitializedFunctionPassManager as LLVMPassManagerRef
    }
}
