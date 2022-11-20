use std::fmt;


mod literal;
pub use literal::LiteralType;

mod expr;
pub use expr::Expr;

mod stmt;
pub use stmt::Stmt;


pub mod visitor {
    pub use crate::ast::{expr::Visitor as Expr, stmt::Visitor as Stmt};
}


pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        for stmt in &self.stmts {
            write!(f, "{}", stmt)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
