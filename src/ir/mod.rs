mod generation;
mod context;
mod function;
mod value;

use core::fmt;

use crate::ast::CompUnit;
use context::Context;
use generation::GenerateIR;
use koopa::ir::Program;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Unknown,
    DuplicateDefinition,
    SymbolNotFound,
    ReturnInVoidFunction,
    UseVoidValue,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown error"),
            Self::DuplicateDefinition => write!(f, "duplicate definition"),
            Self::SymbolNotFound => write!(f, "symbol not found"),
            Self::ReturnInVoidFunction => write!(f, "return in void function"),
            Self::UseVoidValue => write!(f, "use void value"),
        }
    }
}

pub fn generate_program(comp_unit: &CompUnit) -> Result<Program> {
    let mut program = Program::new();
    comp_unit.generate(&mut program, &mut Context::new())?;
    println!("{:#?}", comp_unit);
    Ok(program)
}
