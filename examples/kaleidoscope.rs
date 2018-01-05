#![feature(io)]

extern crate llvm_safe;
#[macro_use] extern crate const_cstr_fork;

mod kaleidoscope_lib;

fn main() {
    kaleidoscope_lib::main();
}

