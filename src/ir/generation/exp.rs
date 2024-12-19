use crate::ir::GenerateIR;
use crate::ir::Result;
use crate::ir::context::Context;
use crate::ast::*;
use koopa::ir::{Program, Value};
use koopa::ir::builder_traits::*;
use koopa::ir::values::BinaryOp;

impl<'ast> GenerateIR<'ast> for Exp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        self.exp.generate(program, context)
    }
}

impl<'ast> GenerateIR<'ast> for ConstExp {
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
            Self::LVal(lval) => {
                let lval = context.get_value(&lval.id)?;
                let active_func = context.active_function();
                let load = active_func.create_value(program).load(lval);
                active_func.push_instruction(program, load);
                Ok(load)
            },
            Self::Number(number) => Ok(
                context
                .active_function()
                .create_value(program)
                .integer(*number)
            ),
        }
    }
}

impl<'ast> GenerateIR<'ast> for UnaryExp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Self::Primary(exp) => exp.generate(program, context),
            Self::Unary(op, exp) => {
                let exp = exp.generate(program, context)?;
                let active_func = context.active_function();
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
                let active_func = context.active_function();
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
                let active_func = context.active_function();
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
                let active_func = context.active_function();
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

impl<'ast> GenerateIR<'ast> for EqExp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Self::Rel(exp) => exp.generate(program, context),
            Self::EqRel(lhs, op, rhs) => {
                let lhs = lhs.generate(program, context)?;
                let rhs = rhs.generate(program, context)?;
                let active_func = context.active_function();
                let value = match op {
                    EqOp::Eq => active_func.create_value(program).binary(BinaryOp::Eq, lhs, rhs),
                    EqOp::Ne => active_func.create_value(program).binary(BinaryOp::NotEq, lhs, rhs),
                };
                active_func.push_instruction(program, value);
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateIR<'ast> for LAndExp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Self::Eq(exp) => exp.generate(program, context),
            Self::LAndEq(lhs, rhs) => {
                let lhs = lhs.generate(program, context)?;
                let rhs = rhs.generate(program, context)?;
                let active_func = context.active_function();
                let value = active_func.create_value(program).binary(BinaryOp::And, lhs, rhs);
                active_func.push_instruction(program, value);
                Ok(value)
            }
        }
    }
}

impl<'ast> GenerateIR<'ast> for LOrExp {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Self::LAnd(exp) => exp.generate(program, context),
            Self::LOrLAnd(lhs, rhs) => {
                let lhs = lhs.generate(program, context)?;
                let rhs = rhs.generate(program, context)?;
                let active_func = context.active_function();
                let value = active_func.create_value(program).binary(BinaryOp::Or, lhs, rhs);
                active_func.push_instruction(program, value);
                Ok(value)
            }
        }
    }
}
