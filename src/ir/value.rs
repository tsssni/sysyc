use super::{Result, Error};

pub enum ExpValue {
    Void,
    Int(koopa::ir::Value),
}

impl ExpValue {
    pub fn value(self) -> Result<koopa::ir::Value> {
        match self {
            Self::Void => Err(Error::UseVoidValue),
            Self::Int(value) => Ok(value),
        }
    }
}
