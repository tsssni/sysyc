mod generation;
mod context;
mod function;

use core::fmt;

use crate::ast::CompUnit;
use context::Context;
use generation::GenerateIR;
use koopa::ir::Program;

pub enum Error {
    Unknown,
    DuplicateDefinition,
    SymbolNotFound,
    ReturnInVoidFunction,
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown error"),
            Self::DuplicateDefinition => write!(f, "duplicate definition"),
            Self::SymbolNotFound => write!(f, "symbol not found"),
            Self::ReturnInVoidFunction => write!(f, "return in void function"),
        }
    }
}

pub fn generate_program(comp_unit: &CompUnit) -> Result<Program> {
    let mut program = Program::new();
    comp_unit.generate(&mut program, &mut Context::new())?;
    Ok(program)
}
