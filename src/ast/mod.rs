pub mod exp;
pub mod var;
pub mod stmt;

pub use exp::*;
pub use var::*;
pub use stmt::*;

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Int
}

#[derive(Debug)]
pub enum BType {
    Int
}
