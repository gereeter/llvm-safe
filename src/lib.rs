#![feature(unique, extern_types)]

extern crate llvm_sys;
extern crate libc;

pub mod id;
mod opaque;
pub mod owned;
pub mod llvm;
pub mod ffi;
