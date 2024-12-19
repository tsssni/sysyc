use crate::ast::Exp;
use crate::ast::exp::ConstExp;
use crate::ast::BType;

#[derive(Debug)]
pub struct LVal {
    pub id: String,
}

#[derive(Debug)]
pub enum Decl {
    Const(ConstDecl),
    Var(VarDecl),
}

#[derive(Debug)]
pub struct ConstDecl {
    pub decl_type: BType,
    pub defs: Vec<ConstDef>,
}

#[derive(Debug)]
pub struct ConstDef {
    pub id: String,
    pub val: ConstInitVal,
}

#[derive(Debug)]
pub struct ConstInitVal {
    pub exp: ConstExp,
}

#[derive(Debug)]
pub struct VarDecl {
    pub decl_type: BType,
    pub defs: Vec<VarDef>,
}

#[derive(Debug)]
pub struct VarDef {
    pub id: String,
    pub val: Option<InitVal>,
}

#[derive(Debug)]
pub struct InitVal {
    pub exp: Exp,
}
