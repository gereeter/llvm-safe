use std::collections::HashMap;
use std::ffi::CString;
use std::iter::repeat;

use compiler::id;
use compiler::llvm;
use compiler::llvm::{Constant, Type, Function, Value};
use compiler::llvm::LLVMRealPredicate;

use kaleidoscope_lib::ast;

pub struct Context<'cid: 'context, 'context: 'module, 'module> {
    context: &'context llvm::Context<'cid>,
    module: llvm::ModuleBuilder<'cid, 'context, 'module>
}

impl<'cid, 'context, 'module> Context<'cid, 'context, 'module> {
    pub fn new(context: &'context llvm::Context<'cid>, module: llvm::ModuleBuilder<'cid, 'context, 'module>) -> Self {
        Context {
            context: context,
            module: module
        }
    }

    pub fn trans_expr<'fid, 'block>(&mut self, expr: &ast::Expr, builder: &mut llvm::PositionedBuilder<'cid, 'context, 'fid, 'block>, named_values: &HashMap<&str, &'block Value<'cid, 'fid>>) -> Result<&'block Value<'cid, 'fid>, &'static str> {
        match *expr {
            ast::Expr::Number(value) => Ok(Constant::f64(value, &self.context).as_value()),
            ast::Expr::Variable(ref name) => named_values.get(&**name).cloned().ok_or("Unknown name in trans"),
            ast::Expr::BinaryOp(op, ref lhs, ref rhs) => {
                let lhs_val = try!(self.trans_expr(lhs, builder, named_values));
                let rhs_val = try!(self.trans_expr(rhs, builder, named_values));

                match op {
                    '+' => {
                        Ok(builder.fadd(lhs_val, rhs_val, const_cstr!("addtmp").as_cstr()))
                    },
                    '-' => {
                        Ok(builder.fsub(lhs_val, rhs_val, const_cstr!("subtmp").as_cstr()))
                    },
                    '*' => {
                        Ok(builder.fmul(lhs_val, rhs_val, const_cstr!("multmp").as_cstr()))
                    },
                    '/' => {
                        Ok(builder.fdiv(lhs_val, rhs_val, const_cstr!("divtmp").as_cstr()))
                    },
                    '<' => {
                        let cmp = builder.fcmp(LLVMRealPredicate::LLVMRealULT, lhs_val, rhs_val, const_cstr!("cmptmp").as_cstr());
                        Ok(builder.ui_to_fp(cmp, Type::f64(&self.context), const_cstr!("convtmp").as_cstr()))
                    },
                    '>' => {
                        let cmp = builder.fcmp(LLVMRealPredicate::LLVMRealUGT, lhs_val, rhs_val, const_cstr!("cmptmp").as_cstr());
                        Ok(builder.ui_to_fp(cmp, Type::f64(&self.context), const_cstr!("convtmp").as_cstr()))
                    },
                    _ => Err("Unknown operation in trans")
                }
            },
            ast::Expr::Call(ref name, ref args) => {
                let c_name = CString::new(&**name).unwrap();
                match self.module.get_named_function(&c_name) {
                    Some(func) => {
                        if func.num_args() != args.len() {
                            return Err("Calling function with incorrect arity");
                        }

                        let arg_vals = args.iter().map(|arg| self.trans_expr(arg, builder, named_values).unwrap()).collect::<Vec<_>>();

                        Ok(builder.call(func, &arg_vals, const_cstr!("calltmp").as_cstr()))
                    },
                     None => Err("Calling function that does not exist")
                }
            }
        }
    }

    pub fn trans_proto(&mut self, proto: &ast::Prototype) -> Result<&'module mut Function<'cid>, &'static str> {
        let c_name = &CString::new(&*proto.name).unwrap();
        if self.module.get_named_function(&c_name).is_some() {
            return Err("Redefinition of already defined function");
         }

        let f64_type = Type::f64(&self.context);
        let arg_types = repeat(f64_type).take(proto.args.len()).collect::<Vec<_>>();
        let func_type = Type::function(&arg_types, f64_type);

        let mut function = self.module.add_function(&c_name, func_type);

        for (index, arg_name) in proto.args.iter().enumerate() {
            let c_arg_name = CString::new(&**arg_name).unwrap();
            id::with(|fid| {
                function.builder(fid).param(index as u32).set_name(&c_arg_name);
            });
        }

        Ok(function)
    }

    pub fn trans_func(&mut self, func: &ast::Function, builder: &mut llvm::Builder<'cid, 'context>) -> Result<&'module mut Function<'cid>, &'static str> {
        let mut function = try!(self.trans_proto(&func.proto));
        try!(id::with(|function_id| {
            let mut function_builder = function.builder(function_id);
            let named_values = func.proto.args.iter().enumerate().map(|(index, name)| (&**name, function_builder.param(index as u32))).collect();
            let (_, entry_bb) = function_builder.append_basic_block(const_cstr!("entry").as_cstr(), &self.context);
            let builder = builder.position_at_end(entry_bb);

            let ret_val = try!(self.trans_expr(&func.body, builder, &named_values));
            builder.ret(ret_val);

            Ok(())
        }));

        function.verify();

        Ok(function)
    }
}
