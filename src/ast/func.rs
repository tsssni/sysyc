use crate::ast::exp::Exp;
use crate::ast::var::BType;
use crate::ast::stmt::Block;

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub params: Option<FuncFParams>,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Int
}

#[derive(Debug)]
pub struct FuncFParams {
    pub params: Vec<FuncFParam>,
}

#[derive(Debug)]
pub struct FuncFParam {
    pub param_type: BType,
    pub id: String,
}

#[derive(Debug)]
pub struct FuncRParams {
    pub params: Vec<Exp>,
}
