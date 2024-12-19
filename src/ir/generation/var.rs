use crate::ir::GenerateIR;
use crate::ir::Result;
use crate::ir::context::Context;
use crate::ast::*;
use koopa::ir::{Program, Value, Type};

impl<'ast> GenerateIR<'ast> for Decl {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        self.decl.generate(program, context)
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
