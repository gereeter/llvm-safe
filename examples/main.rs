extern crate compiler;

use std::ffi::CString;

use compiler::{id, llvm};

fn main() {
    id::with(|context_id| {
        let context = llvm::Context::new(context_id);
        let mut module = llvm::Module::new(&CString::new("mymodule").unwrap(), &context);
        let mut module_builder = module.builder();

        let i32_ty = llvm::Type::i32(&context);
        let func_ty = llvm::Type::function(&[i32_ty], i32_ty);
        let mut builder = llvm::Builder::new(&context);

        id::with(|function_id| {
            let mut function = module_builder.add_function(function_id, &CString::new("square").unwrap(), func_ty);
            {
                let mut function_builder = function.builder();

                let (_, entry) = function_builder.append_basic_block(&CString::new("entry").unwrap(), &context);
                let builder = builder.position_at_end(entry);

                let ret = builder.mul(function_builder.param(0), function_builder.param(0), &CString::new("square").unwrap());
                builder.ret(ret);
            }

            function.dump();
        });

        id::with(|function_id| {
            let mut function = module_builder.add_function(function_id, &CString::new("jumpy").unwrap(), func_ty);
            {
                let mut function_builder = function.builder();
                let (_, entry) = function_builder.append_basic_block(&CString::new("entry").unwrap(), &context);
                let (exit_label, exit) = function_builder.append_basic_block(&CString::new("exit").unwrap(), &context);

                builder.position_at_end(entry).br(exit_label);
                builder.position_at_end(exit).ret(function_builder.param(0));
            }

            function.dump()
        });

        id::with(|function_id| {
            let mut function = module_builder.add_function(function_id, &CString::new("consts").unwrap(), func_ty);
            {
                let mut function_builder = function.builder();
                let (_, entry) = function_builder.append_basic_block(&CString::new("entry").unwrap(), &context);
                let mut builder = builder.position_at_end(entry);

                let const_6 = builder.mul(llvm::Constant::i32(2, &context).as_value(), llvm::Constant::i32(3, &context).as_value(), &CString::new("const_6").unwrap());
                let xplus4 = builder.add(function_builder.param(0), llvm::Constant::i32(4, &context).as_value(), &CString::new("xplus4").unwrap());
                let ret = builder.add(const_6, xplus4, &CString::new("final").unwrap());
                builder.ret(ret);
            }

            function.dump()
        });

        id::with(|function_id| {
            let mut function = module_builder.add_function(function_id, &CString::new("abs").unwrap(), func_ty);
            {
                let mut function_builder = function.builder();
                let (entry_label, entry) = function_builder.append_basic_block(&CString::new("entry").unwrap(), &context);
                let (negative_label, negative) = function_builder.append_basic_block(&CString::new("negative").unwrap(), &context);
                let (exit_label, exit) = function_builder.append_basic_block(&CString::new("exit").unwrap(), &context);

                {
                    let mut builder = builder.position_at_end(entry);
                    let cmp = builder.icmp(llvm::LLVMIntPredicate::LLVMIntSLT, function_builder.param(0), llvm::Constant::i32(0, &context).as_value(), &CString::new("isneg").unwrap());
                    builder.cond_br(cmp, negative_label, exit_label);
                }

                let negated = {
                    let mut builder = builder.position_at_end(negative);
                    let negated = builder.neg(function_builder.param(0), &CString::new("negated").unwrap());
                    builder.br(exit_label);
                    negated
                };

                {
                    let mut builder = builder.position_at_end(exit);
                    let mut phi = builder.phi(i32_ty, &CString::new("phi").unwrap());
                    phi.add_incoming_branch(negated, negative_label);
                    phi.add_incoming_branch(function_builder.param(0), entry_label);
                    builder.ret(phi.as_value());
                }
            }

            function.dump()
        });
    });
}
