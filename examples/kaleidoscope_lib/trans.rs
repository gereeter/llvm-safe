use std::collections::HashMap;
use std::ffi::CString;
use std::iter::repeat;

use compiler::id;
use compiler::llvm::{Context, ModuleBuilder, Builder, PositionedBuilder, Value, Constant, Type, Function};
use compiler::llvm::LLVMRealPredicate;

use kaleidoscope_lib::ast;

pub fn trans_expr<'cid: 'context, 'context: 'block, 'module, 'fid, 'block>(expr: &ast::Expr, context: &'context Context<'cid>, module: &ModuleBuilder<'cid, 'context, 'module>, builder: &mut PositionedBuilder<'cid, 'context, 'fid, 'block>, named_values: &HashMap<&str, &'block Value<'cid, 'fid>>) -> Result<&'block Value<'cid, 'fid>, &'static str> {
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
                    if func.num_args() != args.len() {
                        return Err("Calling function with incorrect arity");
                    }

                    let arg_vals = args.iter().map(|arg| trans_expr(arg, context, module, builder, named_values).unwrap()).collect::<Vec<_>>();

                    let calltmp_name = CString::new("calltmp").unwrap();
                    Ok(builder.call(func, &arg_vals, &calltmp_name))
                },
                None => Err("Calling function that does not exist")
            }
        }
    }
}

pub fn trans_proto<'cid: 'context, 'context: 'module, 'module>(proto: &ast::Prototype, context: &'context Context<'cid>, module: &mut ModuleBuilder<'cid, 'context, 'module>) -> Result<&'module mut Function<'cid>, &'static str> {
    let c_name = &CString::new(&*proto.name).unwrap();
    if module.get_named_function(&c_name).is_some() {
        return Err("Redefinition of already defined function");
    }

    let f64_type = Type::f64(context);
    let arg_types = repeat(f64_type).take(proto.args.len()).collect::<Vec<_>>();
    let func_type = Type::function(&arg_types, f64_type);

    let mut function = module.add_function(&c_name, func_type);

    for (index, arg_name) in proto.args.iter().enumerate() {
        let c_arg_name = CString::new(&**arg_name).unwrap();
        id::with(|fid| {
            function.builder(fid).param(index as u32).set_name(&c_arg_name);
        });
    }

    Ok(function)
}

pub fn trans_func<'cid: 'context, 'context: 'module, 'module>(func: &ast::Function, context: &'context Context<'cid>, module: &mut ModuleBuilder<'cid, 'context, 'module>, builder: &mut Builder<'cid, 'context>) -> Result<&'module mut Function<'cid>, &'static str> {
    let mut function = try!(trans_proto(&func.proto, context, module));
    try!(id::with(|function_id| {
        let mut function_builder = function.builder(function_id);
        let named_values = func.proto.args.iter().enumerate().map(|(index, name)| (&**name, function_builder.param(index as u32))).collect();
        let entry_name = CString::new("entry").unwrap();
        let (_, entry_bb) = function_builder.append_basic_block(&entry_name, context);
        let builder = builder.position_at_end(entry_bb);

        let ret_val = try!(trans_expr(&func.body, context, module, builder, &named_values));
        builder.ret(ret_val);

        Ok(())
    }));

    function.verify();

    Ok(function)
}
