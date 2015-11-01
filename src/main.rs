#![feature(unique, core_intrinsics)]

extern crate llvm_sys;

use std::ffi::CString;

mod id;
mod owned;
mod llvm;

fn main() {
    id::with(|context_id| {
        let context = llvm::Context::new(context_id);
        id::with(|module_id| {
            let module = llvm::Module::new(module_id, &CString::new("mymodule").unwrap(), &context);

            let i32_ty = llvm::Type::i32(&context);
            let func_ty = llvm::Type::function(&[i32_ty], i32_ty);
            let mut builder = llvm::Builder::new(&context);

            id::with(|function_id| {
                let mut function = module.add_function(function_id, &CString::new("square").unwrap(), func_ty);
                {
                    let mut function_builder = function.builder();

                    let entry = function_builder.append_basic_block(&CString::new("entry").unwrap(), &context);
                    let builder = builder.position_at_end(entry);

                    let ret = builder.mul(function_builder.param(0), function_builder.param(0), &CString::new("square").unwrap());
                    builder.ret(ret);
                }

                function.dump();
            });

            id::with(|function_id| {
                let mut function = module.add_function(function_id, &CString::new("jumpy").unwrap(), func_ty);
                {
                    let mut function_builder = function.builder();
                    let entry = function_builder.append_basic_block(&CString::new("entry").unwrap(), &context);
                    let exit = function_builder.append_basic_block(&CString::new("exit").unwrap(), &context);

                    builder.position_at_end(entry).br(exit);
                    builder.position_at_end(exit).ret(function_builder.param(0));
                }

                function.dump()
            });
        });
    });
}
