extern crate llvm_safe;
#[macro_use] extern crate const_cstr_fork;

use llvm_safe::{id, llvm};
use llvm_safe::inheritance::upcast;

fn main() {
    id::with2(|context_id, module_id| {
        let context = llvm::Context::new(context_id);
        let mut module = llvm::Module::new(module_id, const_cstr!("mymodule").as_cstr(), &context);
        let module_builder = module.builder();

        let i32_ty = upcast::<_,llvm::Type>(llvm::Type::i32(&context));
        let func_ty = llvm::Type::function(&[i32_ty], i32_ty, false);
        let mut builder = llvm::Builder::new(&context);

        {
            let function = module_builder.add_function(const_cstr!("square").as_cstr(), func_ty);
            id::with(|function_id| {
                let function_builder = function.builder(function_id);
                let arg = function_builder.params().next().unwrap();

                let (_, entry) = function_builder.append_basic_block(const_cstr!("entry").as_cstr(), &context);
                let builder = builder.position_at_end(entry);

                let ret = builder.mul(arg, arg, const_cstr!("square").as_cstr());
                builder.ret(ret);
            });

            function.dump();
        }

        {
            let function = module_builder.add_function(const_cstr!("jumpy").as_cstr(), func_ty);
            id::with(|function_id| {
                let function_builder = function.builder(function_id);
                let arg = function_builder.params().next().unwrap();
                let (_, entry) = function_builder.append_basic_block(const_cstr!("entry").as_cstr(), &context);
                let (exit_label, exit) = function_builder.append_basic_block(const_cstr!("exit").as_cstr(), &context);

                builder.position_at_end(entry).br(exit_label);
                builder.position_at_end(exit).ret(arg);
            });

            function.dump()
        }

        {
            let function = module_builder.add_function(const_cstr!("consts").as_cstr(), func_ty);
            id::with(|function_id| {
                let function_builder = function.builder(function_id);
                let arg = function_builder.params().next().unwrap();
                let (_, entry) = function_builder.append_basic_block(const_cstr!("entry").as_cstr(), &context);
                let builder = builder.position_at_end(entry);

                let const_6 = builder.mul(upcast(llvm::Constant::i32(2, &context)), upcast(llvm::Constant::i32(3, &context)), const_cstr!("const_6").as_cstr());
                let xplus4 = builder.add(arg, upcast(llvm::Constant::i32(4, &context)), const_cstr!("xplus4").as_cstr());
                let ret = builder.add(const_6, xplus4, const_cstr!("final").as_cstr());
                builder.ret(ret);
            });

            function.dump()
        }

        {
            let function = module_builder.add_function(const_cstr!("abs").as_cstr(), func_ty);
            id::with(|function_id| {
                let function_builder = function.builder(function_id);
                let arg = function_builder.params().next().unwrap();
                let (entry_label, entry) = function_builder.append_basic_block(const_cstr!("entry").as_cstr(), &context);
                let (negative_label, negative) = function_builder.append_basic_block(const_cstr!("negative").as_cstr(), &context);
                let (exit_label, exit) = function_builder.append_basic_block(const_cstr!("exit").as_cstr(), &context);

                {
                    let builder = builder.position_at_end(entry);
                    let cmp = builder.icmp(llvm::LLVMIntPredicate::LLVMIntSLT, arg, upcast(llvm::Constant::i32(0, &context)), const_cstr!("isneg").as_cstr());
                    builder.cond_br(cmp, negative_label, exit_label);
                }

                let negated = {
                    let builder = builder.position_at_end(negative);
                    let negated = builder.neg(arg, const_cstr!("negated").as_cstr());
                    builder.br(exit_label);
                    negated
                };

                {
                    let builder = builder.position_at_end(exit);
                    let phi = builder.phi(i32_ty, const_cstr!("phi").as_cstr());
                    phi.add_incoming_branch(negated, negative_label);
                    phi.add_incoming_branch(arg, entry_label);
                    builder.ret(phi.as_value());
                }
            });

            function.dump()
        }
    });
}
