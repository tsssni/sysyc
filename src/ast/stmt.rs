use crate::ast::exp::Exp;
use crate::ast::var::{Decl, LVal};

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
    Sblock(Sblock),
    If(If),
    While(While),
    Return(Return),
}

#[derive(Debug)]
pub struct Assign {
    pub lval: LVal,
    pub exp: Exp,
}

#[derive(Debug)]
pub struct Sblock {
    pub block: Block,
}

#[derive(Debug)]
pub struct If {
    pub cond: Exp,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug)]
pub struct While {
    pub cond: Exp,
    pub block: Block,
}

#[derive(Debug)]
pub struct Return {
    pub exp: Exp,
}

