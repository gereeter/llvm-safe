use std::iter::Peekable;
use std::io::{self, Read, Write};

use llvm_safe::id;
use llvm_safe::llvm::init;
use llvm_safe::llvm::{Context, Module, ModuleBuilder, Builder, FunctionPassManager, InitializedFunctionPassManager};
use llvm_safe::llvm::target;

use kaleidoscope_lib::lexer::{Token, Tokens};
use kaleidoscope_lib::parser::{parse_definition, parse_extern, parse_top_level_expr};

mod util;
mod lexer;
mod ast;
mod parser;
mod trans;

// Driver //

fn handle_definition<'cid: 'context, 'context, 'mid: 'module + 'fpm, 'module, 'fpm, I: Iterator<Item=Token>>(iter: &mut Peekable<I>, context: &'context Context<'cid>, module: &mut ModuleBuilder<'cid, 'mid, 'module>, fpm: &mut InitializedFunctionPassManager<'mid, 'module, 'fpm>, builder: &mut Builder<'cid, 'context>) {
    match parse_definition(iter) {
        Ok(def) => {
            match trans::Context::new(context, module.reborrow()).trans_func(&def, builder) {
                Ok(function) => {
                    println!("Read a function definition:");
                    fpm.run(function);
                    function.dump();
                },
                Err(err) => println!("Compilation error: {}", err)
            }
        },
        Err(err) => println!("Parse error: {}", err)
    }
}

fn handle_extern<'cid: 'context, 'context, 'mid: 'module, 'module, I: Iterator<Item=Token>>(iter: &mut Peekable<I>, context: &'context Context<'cid>, module: &mut ModuleBuilder<'cid, 'mid, 'module>) {
    match parse_extern(iter) {
        Ok(proto) => {
            match trans::Context::new(context, module.reborrow()).trans_proto(&proto) {
                Ok(function) => {
                    println!("Read an extern:");
                    function.dump();
                },
                Err(err) => println!("Compilation error: {}", err)
            }
        },
        Err(err) => println!("Parse error: {}", err)
    }
}

fn handle_top_level_expr<'cid: 'context, 'context, 'mid: 'module + 'fpm, 'module, 'fpm, I: Iterator<Item=Token>>(iter: &mut Peekable<I>, context: &'context Context<'cid>, module: &mut ModuleBuilder<'cid, 'mid, 'module>, fpm: &mut InitializedFunctionPassManager<'mid, 'module, 'fpm>, builder: &mut Builder<'cid, 'context>) {
    match parse_top_level_expr(iter) {
        Ok(def) => {
            match trans::Context::new(context, module.reborrow()).trans_func(&def, builder) {
                Ok(function) => {
                    println!("Read a top level expression:");
                    fpm.run(function);
                    function.dump();
                },
                Err(err) => println!("Compilation error: {}", err)
            }
        },
        Err(err) => println!("Parse error: {}", err)
    }
}

pub fn main() {
    unsafe {
        init::init_target_infos();
        init::init_targets();
        init::init_target_mcs();
        init::init_asm_printers();
    }

    let target_triple = target::default_triple();
    println!("{:?}", &target_triple);
    let target = target::Target::from_triple(&target_triple).unwrap();
    let target_machine = target::TargetMachine::new(target, &target_triple, const_cstr!("generic").as_cstr(), const_cstr!("").as_cstr(), target::LLVMCodeGenOptLevel::LLVMCodeGenLevelNone, target::LLVMRelocMode::LLVMRelocDefault, target::LLVMCodeModel::LLVMCodeModelDefault);
    let data_layout = target_machine.data_layout();
    println!("{:?}", data_layout.as_string());

    let stdout = io::stdout();

    print!("ready> ");
    stdout.lock().flush().unwrap();

    let stdin = io::stdin();
    let mut tokens = Tokens::new(stdin.lock().chars().map(|c| c.unwrap())).peekable();

    id::with2(|context_id, module_id| {
        let context = Context::new(context_id);
        let mut module = Module::new(module_id, const_cstr!("mymodule").as_cstr(), &context);
        module.set_data_layout(&data_layout);
        module.set_target_triple(&target_triple);

        let mut fpm = FunctionPassManager::new(&module);
        fpm.add_basic_alias_analysis();
        fpm.add_instruction_combine();
        fpm.add_reassociate();
        fpm.add_gvn();
        fpm.add_simplify_cfg();

        {
            let mut module_builder = module.builder();
            let mut fpm = fpm.initialize(&module_builder);

            let mut builder = Builder::new(&context);
            loop {
                match tokens.peek().cloned() {
                    None => break,
                    Some(Token::Other(';')) => {
                        tokens.next();
                        continue;
                    },
                    Some(Token::Def) => {
                        handle_definition(&mut tokens, &context, &mut module_builder, &mut fpm, &mut builder);
                    },
                    Some(Token::Extern) => {
                        handle_extern(&mut tokens, &context, &mut module_builder);
                    },
                    _ => {
                        handle_top_level_expr(&mut tokens, &context, &mut module_builder, &mut fpm, &mut builder);
                    }
                }

                print!("ready> ");
                stdout.lock().flush().unwrap();
            }
        }

        print!("Writing to file...");
        target_machine.emit_module_to_file(&module, const_cstr!("output.o").as_cstr(), target::LLVMCodeGenFileType::LLVMObjectFile).unwrap();
        println!(" done.");

    });
}
