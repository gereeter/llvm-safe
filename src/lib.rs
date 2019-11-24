#![feature(ptr_internals, extern_types)]

extern crate llvm_sys;
extern crate libc;

pub mod id;
pub mod inheritance;
mod opaque;
pub mod owned;
pub mod llvm;
pub mod ffi;
