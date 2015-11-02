use std::collections::HashMap;
use std::ffi::CString;
use std::iter::repeat;

use compiler::id::Id;
use compiler::llvm::{Context, Module, ModuleBuilder, Builder, PositionedBuilder, Value, Constant, Type, Function};
use compiler::llvm::LLVMRealPredicate;

use kaleidoscope_lib::ast;

pub fn trans_expr<'cid: 'context, 'context: 'block, 'mid, 'module, 'fid, 'block>(expr: &ast::Expr, context: &'context Context<'cid>, module: &ModuleBuilder<'cid, 'context, 'mid, 'module>, builder: &mut PositionedBuilder<'cid, 'context, 'fid, 'block>, named_values: &HashMap<&str, &'block Value<'cid, 'fid>>) -> Result<&'block Value<'cid, 'fid>, &'static str> {
    match *expr {
        ast::Expr::Number(value) => Ok(Constant::f64(value, context).as_value()),
        ast::Expr::Variable(ref name) => named_values.get(&**name).cloned().ok_or("Unknown name in trans"),
        ast::Expr::BinaryOp(op, ref lhs, ref rhs) => {
            let lhs_val = try!(trans_expr(lhs, context, module, builder, named_values));
            let rhs_val = try!(trans_expr(rhs, context, module, builder, named_values));

            match op {
                '+' => {
                    let name = CString::new("addtmp").unwrap();
                    Ok(builder.fadd(lhs_val, rhs_val, &name))
                },
                '-' => {
                    let name = CString::new("subtmp").unwrap();
                    Ok(builder.fsub(lhs_val, rhs_val, &name))
                },
                '*' => {
                    let name = CString::new("multmp").unwrap();
                    Ok(builder.fmul(lhs_val, rhs_val, &name))
                },
                '/' => {
                    let name = CString::new("divtmp").unwrap();
                    Ok(builder.fdiv(lhs_val, rhs_val, &name))
                },
                '<' => {
                    let cmp = builder.fcmp(LLVMRealPredicate::LLVMRealULT, lhs_val, rhs_val, &CString::new("cmptmp").unwrap());
                    Ok(builder.ui_to_fp(cmp, Type::f64(context), &CString::new("convtmp").unwrap()))
                },
                '>' => {
                    let cmp = builder.fcmp(LLVMRealPredicate::LLVMRealUGT, lhs_val, rhs_val, &CString::new("cmptmp").unwrap());
                    Ok(builder.ui_to_fp(cmp, Type::f64(context), &CString::new("convtmp").unwrap()))
                },
                _ => Err("Unknown operation in trans")
            }
        },
        ast::Expr::Call(ref name, ref args) => {
            let c_name = CString::new(&**name).unwrap();
            match module.get_named_function(&c_name) {
                Some(func) => {
//                  let num_args = LLVMCountParams(func) as usize;
//                  if num_args != args.len() {
//                      return Err("Calling function with incorrect arity");
//                  }

                    let arg_vals = args.iter().map(|arg| trans_expr(arg, context, module, builder, named_values).unwrap()).collect::<Vec<_>>();

                    let calltmp_name = CString::new("calltmp").unwrap();
                    Ok(builder.call(func, &arg_vals, &calltmp_name))
                },
                None => Err("Calling function that does not exist")
            }
        }
    }
}

pub fn trans_proto<'cid: 'context, 'context: 'module, 'mid: 'module, 'module, 'fid>(proto: &ast::Prototype, id: Id<'fid>, context: &'context Context<'cid>, module: &'module mut Module<'cid, 'context, 'mid>) -> Result<(&'module mut Function<'cid, 'mid, 'fid>, ModuleBuilder<'cid, 'context, 'mid, 'module>), &'static str> {
    let c_name = &CString::new(&*proto.name).unwrap();
    let (function, builder) = match module.get_named_function_mut(id, &c_name) {
        Ok((old_function, builder)) => {
//          if LLVMCountBasicBlocks(old_function) != 0 {
//              return Err("Redefinition of already defined function");
//          }
//          if LLVMCountParams(old_function) as usize != proto.args.len() {
//              return Err("Redefinition of function with differing arity");
//          }

            (old_function, builder)
        },
        Err((id, module)) => {
            let f64_type = Type::f64(context);
            let arg_types = repeat(f64_type).take(proto.args.len()).collect::<Vec<_>>();
            let func_type = Type::function(&arg_types, f64_type);

            let mut builder = module.builder();

            let function = builder.add_function(id, &c_name, func_type);
            (function, builder)
        }
    };

    for (index, arg_name) in proto.args.iter().enumerate() {
        let c_arg_name = CString::new(&**arg_name).unwrap();
        function.builder().param(index as u32).set_name(&c_arg_name);
    }

    Ok((function, builder))
}

pub fn trans_func<'cid: 'context, 'context: 'module, 'mid: 'module, 'module, 'fid>(func: &ast::Function, id: Id<'fid>, context: &'context Context<'cid>, module: &'module mut Module<'cid, 'context, 'mid>, builder: &mut Builder<'cid, 'context>) -> Result<&'module mut Function<'cid, 'mid, 'fid>, &'static str> {
    let (mut function, module_builder) = try!(trans_proto(&func.proto, id, context, module));
    {
        let mut function_builder = function.builder();
        let named_values = func.proto.args.iter().enumerate().map(|(index, name)| (&**name, function_builder.param(index as u32))).collect();
        let entry_name = CString::new("entry").unwrap();
        let (_, entry_bb) = function_builder.append_basic_block(&entry_name, context);
        let builder = builder.position_at_end(entry_bb);

        let ret_val = try!(trans_expr(&func.body, context, &module_builder, builder, &named_values));
        builder.ret(ret_val);
    }

    // LLVMVerifyFunction(function, LLVMVerifierFailureAction::LLVMAbortProcessAction);

    Ok(function)
}
