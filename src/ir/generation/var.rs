use crate::ir::GenerateIR;
use crate::ir::Result;
use crate::ir::context::Context;
use crate::ast::*;
use koopa::ir::builder::LocalInstBuilder;
use koopa::ir::{Program, Value, Type};

impl<'ast> GenerateIR<'ast> for Decl {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Decl::Const(decl) => decl.generate(program, context),
            Decl::Var(decl) => decl.generate(program, context),
        }
    }
}

impl<'ast> GenerateIR<'ast> for ConstDecl {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        for def in &self.defs {
            def.generate(program, context)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for ConstDef {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let val = self.val.generate(program, context)?;
        context.insert_value(&self.id, val)?;
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for ConstInitVal {
    type Out = Value;

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        self.exp.generate(program, context)
    }
}

impl<'ast> GenerateIR<'ast> for VarDecl {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        for def in &self.defs {
            def.generate(program, context)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for VarDef {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let val = self.val.as_ref().map(|v| v.generate(program, context));
        let active_func = context.active_fcuntion();
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
