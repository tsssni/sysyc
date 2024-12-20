use crate::ir::GenerateIR;
use crate::ir::Result;
use crate::ir::context::Context;
use crate::ir::function::FunctionInfo;
use crate::ast::func::*;
use crate::ast::var::BType;
use koopa::ir::{Program, FunctionData, Type, Value};
use koopa::ir::builder_traits::*;

impl<'ast> GenerateIR<'ast> for FuncDef {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let params: Vec<(Option<String>, Type)> = match &self.params {
            None => Vec::new(),
            Some(params) => params.generate(program, context)?,
        };
        let ret_type = self.func_type.generate(program, context)?;
        let mut data = FunctionData::with_param_names(format!("@{}", self.ident), params, ret_type);

        let entry = data.dfg_mut().new_bb().basic_block(Some("%entry".into()));
        let end = data.dfg_mut().new_bb().basic_block(Some("%end".into()));
        let body = data.dfg_mut().new_bb().basic_block(Some("%body".into()));

        let ret_val = {
            let alloc = data.dfg_mut().new_value().alloc(match self.func_type {
                FuncType::Int => Type::get_i32(),
            });
            data.dfg_mut().set_value_name(alloc, Some("%ret".into()));
            alloc
        };

        let params_val = data.params().to_owned();
        let func = program.new_func(data);
        let mut info = FunctionInfo::new(func, entry, end, ret_val);
        info.push_basic_block(program, entry);
        info.push_instruction(program, ret_val);

        context.push();
        info.push_basic_block(program, body);
        if let Some(params) = &self.params {
            for (param, value) in params.params.iter().zip(params_val) {
                let ptype = program.func(func).dfg().value(value).ty().clone();
                let alloc = info.allocate(program, ptype, Some(&param.id));
                let store = info.create_value(program).store(value, alloc);
                info.push_instruction(program, store);
                context.insert_value(&param.id, alloc)?;
            }
        }

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


impl<'ast> GenerateIR<'ast> for FuncFParams {
    type Out = Vec<(Option<String>, Type)>;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let mut params = Self::Out::new();
        for param in &self.params {
            params.push((Some(format!("@{}", &param.id)), param.generate(program, context)?));
        }
        Ok(params)
    }
}

impl<'ast> GenerateIR<'ast> for FuncFParam {
    type Out = Type;

    fn generate(&'ast self, _: &mut Program, _: &mut Context<'ast>) -> Result<Self::Out> {
        Ok(match self.param_type {
            BType::Int => Type::get_i32(),
        })
    }
}

impl<'ast> GenerateIR<'ast> for FuncRParams {
    type Out = Vec<Value>;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let mut params = Self::Out::new();
        for param in &self.params {
            params.push(param.generate(program, context)?);
        }
        Ok(params)
    }
}
