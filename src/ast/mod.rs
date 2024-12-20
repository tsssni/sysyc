pub mod exp;
pub mod var;
pub mod stmt;
pub mod func;

pub use exp::*;
pub use var::*;
pub use stmt::*;
pub use func::*;

#[derive(Debug)]
pub struct CompUnit {
    pub func_defs: Vec<FuncDef>,
}
