pub mod exp;
pub mod var;

pub use exp::*;
pub use var::*;

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

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(Assign),
    Return(Return),
}

#[derive(Debug)]
pub struct Assign {
    pub lval: LVal,
    pub exp: Exp,
}

#[derive(Debug)]
pub struct Return {
    pub exp: Exp,
}
