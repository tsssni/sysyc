use super::function::FunctionInfo;
use super::Error;
use super::Result;
use super::context::Context;
use crate::ast::*;
use koopa::ir::builder_traits::*;
use koopa::ir::{FunctionData, Program, Type};

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

        let info = &mut context.active_fcuntion_mut();
        let jump = info.create_value(program).jump(info.end());
        info.push_instruction(program, jump);
        info.push_basic_block(program, info.end());
        let value = info.return_value().map(|alloc| {
            let zero = info.create_value(program).integer(0);
            let store = info.create_value(program).store(zero, alloc);
            info.push_instruction(program, store);

            let value = info.create_value(program).load(alloc);
            info.push_instruction(program, value);
            value
        });
        let ret = info.create_value(program).ret(value);
        info.push_instruction(program, ret);
        Ok(())
    }
}
