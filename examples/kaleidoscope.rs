#![feature(io)]

extern crate compiler;
#[macro_use] extern crate const_cstr;

mod kaleidoscope_lib;

fn main() {
    kaleidoscope_lib::main();
}

