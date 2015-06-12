extern crate llvm_sys;

use std::ffi::CString;

mod id;
mod llvm;

fn main() {
    id::with(|context_id| {
        let context = llvm::Context::new(context_id);
        id::with(|module_id| {
            let module = llvm::Module::new(module_id, &CString::new("mymodule").unwrap(), &context);
            id::with(|function_id| {
                let f64_ty = llvm::Type::f64(&context);
                let func_ty = llvm::Type::function(&[f64_ty], f64_ty);
                let function = module.add_function(function_id, &CString::new("square").unwrap(), func_ty);
                let mut builder = llvm::Builder::new(&context);

                let entry = function.append_basic_block(&CString::new("entry").unwrap(), &context);
                let builder = builder.position_at_end(entry);
                let ret = builder.build_mul(function.param(0), function.param(0), &CString::new("square").unwrap());
                builder.build_ret(ret);

                function.dump();
            });
        });
    });
}
