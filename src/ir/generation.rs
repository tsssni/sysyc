use super::function::FunctionInfo;
use super::{Result, Error};
use super::context::Context;
use crate::ast::*;
use koopa::ir::{FunctionData, Program, Type, Value};
use koopa::ir::builder_traits::*;
use koopa::ir::values::BinaryOp;

pub trait GenerateIR<'ast> {
    type Out;
    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out>;
}

impl<'ast> GenerateIR<'ast> for CompUnit {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        self.func_def.generate(program, context)?;
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for FuncDef {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let params_type: Vec<Type> = vec![];
        let ret_type = self.func_type.generate(program, context)?;
        let mut data = FunctionData::new(format!("@{}", self.ident), params_type, ret_type);

        let entry = data.dfg_mut().new_bb().basic_block(Some("%entry".into()));
        let end = data.dfg_mut().new_bb().basic_block(Some("%end".into()));
        let active = data.dfg_mut().new_bb().basic_block(None);

        let mut ret_val = None;
        match self.func_type {
            FuncType::Int => {
                let alloc = data.dfg_mut().new_value().alloc(Type::get_i32());
                data.dfg_mut().set_value_name(alloc, Some("%ret".into()));
                ret_val = Some(alloc);
            }
        }

        let func = program.new_func(data);
        let mut info = FunctionInfo::new(func, entry, end, ret_val);
        info.push_basic_block(program, entry);
        if let Some(ret_val) = info.return_value() {
            info.push_instruction(program, ret_val);
        }

        let jump = info.create_value(program).jump(active);
        info.push_instruction(program, jump);
        info.push_basic_block(program, active);

        context.push();
        context.insert_function(&self.ident, func)?;
        context.active_fcuntion = Some(info);
        self.block.generate(program, context)?;
        context.pop();

        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for FuncType {
    type Out = Type;

    fn generate(&'ast self, _: &mut Program, _: &mut Context<'ast>) -> Result<Self::Out> {
        Ok(match self {
            Self::Int => Type::get_i32(),
        })
    }
}

impl<'ast> GenerateIR<'ast> for Block {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        context.push();
        self.stmt.generate(program, context)?;
        context.pop();
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for Stmt {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        if let None = context.active_fcuntion().return_value() {
            return Err(Error::ReturnInVoidFunction);
        }
        let exp = self.exp.generate(program, context)?;

        let active_func = context.active_fcuntion_mut();
        let jump = active_func.create_value(program).jump(active_func.end());
        active_func.push_instruction(program, jump);
        active_func.push_basic_block(program, active_func.end());

        let value = active_func.return_value().map(|alloc| {
            let store = active_func.create_value(program).store(exp, alloc);
            active_func.push_instruction(program, store);

            let value = active_func.create_value(program).load(alloc);
            active_func.push_instruction(program, value);
            value
        });
        let ret = active_func.create_value(program).ret(value);
        active_func.push_instruction(program, ret);
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for Exp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        self.exp.generate(program, context)
    }
}

impl<'ast> GenerateIR<'ast> for PrimaryExp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Self::Exp(exp) => exp.generate(program, context),
            Self::Number(number) => Ok(
                context
                .active_fcuntion()
                .create_value(program)
                .integer(*number)
            ),
        }
    }
}

impl<'ast> GenerateIR<'ast> for UnaryExp {
    type Out = koopa::ir::Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Self::Primary(exp) => exp.generate(program, context),
            Self::Unary(op, exp) => {
                let exp = exp.generate(program, context)?;
                let active_func = context.active_fcuntion();
                let zero = active_func.create_value(program).integer(0);
                let value = match op {
                    UnaryOp::Plus => active_func.create_value(program).binary(BinaryOp::Add, zero, exp),
                    UnaryOp::Neg => active_func.create_value(program).binary(BinaryOp::Sub, zero, exp),
                    UnaryOp::Not => active_func.create_value(program).binary(BinaryOp::Eq, zero, exp),
                };
                active_func.push_instruction(program, value);
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateIR<'ast> for MulExp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Self::Unary(exp) => exp.generate(program, context),
            Self::MulUnary(lhs, op, rhs) => {
                let lhs = lhs.generate(program, context)?;
                let rhs = rhs.generate(program, context)?;
                let active_func = context.active_fcuntion();
                let value = match op {
                    MulOp::Mul => active_func.create_value(program).binary(BinaryOp::Mul, lhs, rhs),
                    MulOp::Div => active_func.create_value(program).binary(BinaryOp::Div, lhs, rhs),
                    MulOp::Mod => active_func.create_value(program).binary(BinaryOp::Mod, lhs, rhs),
                };
                active_func.push_instruction(program, value);
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateIR<'ast> for AddExp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Self::Mul(exp) => exp.generate(program, context),
            Self::AddMul(lhs, op, rhs) => {
                let lhs = lhs.generate(program, context)?;
                let rhs = rhs.generate(program, context)?;
                let active_func = context.active_fcuntion();
                let value = match op {
                    AddOp::Add => active_func.create_value(program).binary(BinaryOp::Add, lhs, rhs),
                    AddOp::Sub => active_func.create_value(program).binary(BinaryOp::Sub, lhs, rhs),
                };
                active_func.push_instruction(program, value);
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateIR<'ast> for RelExp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Self::Add(exp) => exp.generate(program, context),
            Self::RelAdd(lhs, op, rhs) => {
                let lhs = lhs.generate(program, context)?;
                let rhs = rhs.generate(program, context)?;
                let active_func = context.active_fcuntion();
                let value = match op {
                    RelOp::Lt => active_func.create_value(program).binary(BinaryOp::Lt, lhs, rhs),
                    RelOp::Gt => active_func.create_value(program).binary(BinaryOp::Gt, lhs, rhs),
                    RelOp::Le => active_func.create_value(program).binary(BinaryOp::Le, lhs, rhs),
                    RelOp::Ge => active_func.create_value(program).binary(BinaryOp::Ge, lhs, rhs),
                };
                active_func.push_instruction(program, value);
                Ok(value)
            }
        }
    }
}
