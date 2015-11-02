use std::iter::Peekable;
use std::io::{self, Read, Write};
use std::ffi::CString;

use compiler::id;
use compiler::llvm::{Context, Module, Builder};

use kaleidoscope_lib::lexer::{Token, Tokens};
use kaleidoscope_lib::parser::{parse_definition, parse_extern, parse_top_level_expr};

mod util;
mod lexer;
mod ast;
mod parser;
mod trans;

// Driver //

fn handle_definition<'cid: 'context, 'context, I: Iterator<Item=Token>>(iter: &mut Peekable<I>, context: &'context Context<'cid>, module: &mut Module<'cid, 'context>, builder: &mut Builder<'cid, 'context>) {
    match parse_definition(iter) {
        Ok(def) => {
            match trans::trans_func(&def, context, &mut module.builder(), builder) {
                Ok(function) => {
                    println!("Read a function definition:");
                    function.dump();
                },
                Err(err) => println!("Compilation error: {}", err)
            }
        },
        Err(err) => println!("Parse error: {}", err)
    }
}

fn handle_extern<'cid: 'context, 'context, I: Iterator<Item=Token>>(iter: &mut Peekable<I>, context: &'context Context<'cid>, module: &mut Module<'cid, 'context>) {
    match parse_extern(iter) {
        Ok(proto) => {
            match trans::trans_proto(&proto, context, &mut module.builder()) {
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

fn handle_top_level_expr<'cid: 'context, 'context, I: Iterator<Item=Token>>(iter: &mut Peekable<I>, context: &'context Context<'cid>, module: &mut Module<'cid, 'context>, builder: &mut Builder<'cid, 'context>) {
    match parse_top_level_expr(iter) {
        Ok(def) => {
            match trans::trans_func(&def, context, &mut module.builder(), builder) {
                Ok(function) => {
                    println!("Read a top level expression:");
                    function.dump();
                },
                Err(err) => println!("Compilation error: {}", err)
            }
        },
        Err(err) => println!("Parse error: {}", err)
    }
}

pub fn main() {
    let stdout = io::stdout();

    print!("ready> ");
    stdout.lock().flush().unwrap();

    let stdin = io::stdin();
    let mut tokens = Tokens::new(stdin.lock().chars().map(|c| c.unwrap())).peekable();

    id::with(|context_id| {
        let context = Context::new(context_id);
        let mut module = Module::new(&CString::new("mymodule").unwrap(), &context);
        let mut builder = Builder::new(&context);
        loop {
            match tokens.peek().cloned() {
                None => return,
                Some(Token::Other(';')) => {
                    tokens.next();
                    continue;
                },
                Some(Token::Def) => {
                    handle_definition(&mut tokens, &context, &mut module, &mut builder);
                },
                Some(Token::Extern) => {
                    handle_extern(&mut tokens, &context, &mut module);
                },
                _ => {
                    handle_top_level_expr(&mut tokens, &context, &mut module, &mut builder);
                }
            }

            print!("ready> ");
            stdout.lock().flush().unwrap();
        }
    });
}
