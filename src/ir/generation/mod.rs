mod exp;
mod var;
mod stmt;
mod func;

use crate::ir::Result;
use crate::ir::context::Context;
use crate::ast::*;
use koopa::ir::Program;

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
