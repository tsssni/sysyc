use crate::ast::Exp;

#[derive(Debug)]
pub struct Decl {
    pub decl_type: BType,
    pub defs: Vec<Def>,
}

#[derive(Debug)]
pub struct Def {
    pub id: String,
    pub val: Option<InitVal>,
}

#[derive(Debug)]
pub struct InitVal {
    pub exp: Exp,
}

#[derive(Debug)]
pub struct LVal {
    pub id: String,
}

#[derive(Debug)]
pub enum BType {
    Int
}
