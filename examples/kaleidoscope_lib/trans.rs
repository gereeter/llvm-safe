use std::collections::HashMap;
use std::ffi::CString;
use std::iter::repeat;

use llvm_safe::id;
use llvm_safe::llvm;
use llvm_safe::llvm::{Constant, Type, Function, Value};
use llvm_safe::llvm::LLVMRealPredicate;

use kaleidoscope_lib::ast;

pub struct Context<'cid: 'context, 'context: 'module, 'mid: 'module, 'module> {
    context: &'context llvm::Context<'cid>,
    module: &'module mut llvm::ModuleBuilder<'cid, 'mid, 'module>
}

impl<'cid, 'context, 'mid, 'module> Context<'cid, 'context, 'mid, 'module> {
    pub fn new<'rmodule>(context: &'context llvm::Context<'cid>, module: &'module mut llvm::ModuleBuilder<'cid, 'mid, 'rmodule>) -> Self {
        Context {
            context: context,
            module: module.reborrow()
        }
    }

    pub fn trans_expr<'fid: 'block, 'block>(&mut self, expr: &ast::Expr, fbuilder: &mut llvm::FunctionBuilder<'cid, 'mid, 'fid, 'block>, builder: &mut llvm::PositionedBuilder<'cid, 'context, 'mid, 'fid, 'block>, named_values: &HashMap<&str, &'block Value<'cid, 'mid, 'fid>>) -> Result<&'block Value<'cid, 'mid, 'fid>, &'static str> where 'module: 'block {
        match *expr {
            ast::Expr::Number(value) => Ok(Constant::f64(value, self.context).as_value()),
            ast::Expr::Variable(ref name) => named_values.get(&**name).cloned().ok_or("Unknown name in trans"),
            ast::Expr::BinaryOp(op, ref lhs, ref rhs) => {
                let lhs_val = try!(self.trans_expr(lhs, fbuilder, builder, named_values));
                let rhs_val = try!(self.trans_expr(rhs, fbuilder, builder, named_values));

                match op {
                    '+' => Ok(builder.fadd(lhs_val, rhs_val, const_cstr!("addtmp").as_cstr())),
                    '-' => Ok(builder.fsub(lhs_val, rhs_val, const_cstr!("subtmp").as_cstr())),
                    '*' => Ok(builder.fmul(lhs_val, rhs_val, const_cstr!("multmp").as_cstr())),
                    '/' => Ok(builder.fdiv(lhs_val, rhs_val, const_cstr!("divtmp").as_cstr())),
                    '<' => {
                        let cmp = builder.fcmp(LLVMRealPredicate::LLVMRealULT, lhs_val, rhs_val, const_cstr!("cmptmp").as_cstr());
                        Ok(builder.ui_to_fp(cmp, Type::f64(self.context), const_cstr!("convtmp").as_cstr()))
                    },
                    '>' => {
                        let cmp = builder.fcmp(LLVMRealPredicate::LLVMRealUGT, lhs_val, rhs_val, const_cstr!("cmptmp").as_cstr());
                        Ok(builder.ui_to_fp(cmp, Type::f64(self.context), const_cstr!("convtmp").as_cstr()))
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

                        let arg_vals = args.iter().map(|arg| self.trans_expr(arg, fbuilder, builder, named_values).unwrap()).collect::<Vec<_>>();

                        Ok(builder.call(func.function_type(), func.as_value(), &arg_vals, const_cstr!("calltmp").as_cstr()))
                    },
                    None => Err("Calling function that does not exist")
                }
            },
            ast::Expr::If(ref cond_expr, ref then_expr, ref else_expr) => {
                let (then_label, then_block) = fbuilder.append_basic_block(const_cstr!("then").as_cstr(), self.context);
                let (else_label, else_block) = fbuilder.append_basic_block(const_cstr!("else").as_cstr(), self.context);
                let (cont_label, cont_block) = fbuilder.append_basic_block(const_cstr!("ifcont").as_cstr(), self.context);

                let cond_val = try!(self.trans_expr(cond_expr, fbuilder, builder, named_values));
                let cond_val = builder.fcmp(LLVMRealPredicate::LLVMRealONE, cond_val, Constant::f64(0.0, self.context).as_value(), const_cstr!("ifcond").as_cstr());
                builder.cond_br(cond_val, then_label, else_label);

                builder.position_at_end(then_block);
                let then_val = try!(self.trans_expr(then_expr, fbuilder, builder, named_values));
                builder.br(cont_label);
                let then_label = builder.get_position();

                builder.position_at_end(else_block);
                let else_val = try!(self.trans_expr(else_expr, fbuilder, builder, named_values));
                builder.br(cont_label);
                let else_label = builder.get_position();

                builder.position_at_end(cont_block);
                let phi = builder.phi(Type::f64(self.context), const_cstr!("iftmp").as_cstr());
                phi.add_incoming_branch(then_val, then_label);
                phi.add_incoming_branch(else_val, else_label);
                Ok(phi.as_value())
            }
        }
    }

    pub fn trans_proto(&mut self, proto: &ast::Prototype) -> Result<&'module mut Function<'cid, 'mid>, &'static str> {
        let c_name = &CString::new(&*proto.name).unwrap();
        if self.module.get_named_function(&c_name).is_some() {
            return Err("Redefinition of already defined function");
         }

        let f64_type = Type::f64(self.context);
        let arg_types = repeat(f64_type).take(proto.args.len()).collect::<Vec<_>>();
        let func_type = Type::function(&arg_types, f64_type, false);

        let function = self.module.add_function(&c_name, func_type);

        id::with(|fid| {
            for (param, arg_name) in function.builder(fid).params().zip(proto.args.iter()) {
                param.set_name(arg_name);
            }
        });

        Ok(function)
    }

    pub fn trans_func(&mut self, func: &ast::Function, builder: &mut llvm::Builder<'cid, 'context>) -> Result<&'module mut Function<'cid, 'mid>, &'static str> {
        let function = try!(self.trans_proto(&func.proto));
        try!(id::with(|function_id| {
            let mut function_builder = function.builder(function_id);
            let named_values = func.proto.args.iter().map(|s| &**s).zip(function_builder.params()).collect();
            let (_, entry_bb) = function_builder.append_basic_block(const_cstr!("entry").as_cstr(), self.context);
            let builder = builder.position_at_end(entry_bb);

            let ret_val = try!(self.trans_expr(&func.body, &mut function_builder, builder, &named_values));
            builder.ret(ret_val);

            Ok(())
        }));

        function.verify();

        Ok(function)
    }
}
