use crate::ir::GenerateIR;
use crate::ir::Result;
use crate::ir::context::Context;
use crate::ast::var::*;
use koopa::ir::builder::LocalInstBuilder;
use koopa::ir::{Program, Value, Type};

impl<'ast> GenerateIR<'ast> for Decl {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        for def in &self.defs {
            def.generate(program, context)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for Def {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let val = self.val.as_ref().map(|v| v.generate(program, context));
        let active_func = context.active_function();
        let alloc = active_func.allocate(program, Type::get_i32(), Some(&self.id));
        if let Some(val) = val {
            let store = active_func.create_value(program).store(val?, alloc);
            active_func.push_instruction(program, store);
        }
        context.insert_value(&self.id, alloc)?;
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for InitVal {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        self.exp.generate(program, context)
    }
}

impl<'ast> GenerateIR<'ast> for LVal {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let active_func = context.active_function();
        let lval = context.get_value(&self.id)?;
        let load = active_func.create_value(program).load(lval);
        active_func.push_instruction(program, load);
        Ok(load)
    }
}
