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
    Return(Return),
}

#[derive(Debug)]
pub struct Assign {
    pub lval: LVal,
    pub exp: Exp,
}

#[derive(Debug)]
pub struct Sexp {
    pub exp: Option<Exp>,
}

#[derive(Debug)]
pub struct Sblock {
    pub block: Block,
}

#[derive(Debug)]
pub struct Return {
    pub exp: Exp,
}

