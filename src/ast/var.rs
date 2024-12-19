use crate::ast::exp::ConstExp;
use crate::ast::BType;

#[derive(Debug)]
pub struct Decl {
    pub decl: ConstDecl,
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
pub struct LVal {
    pub id: String,
}
