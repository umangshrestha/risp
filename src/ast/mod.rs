mod literal;
pub use literal::LiteralType;

mod expr;
pub use expr::Expr;

mod stmt;
pub use stmt::Stmt;

pub mod Visitor {
    pub use crate::ast::{expr::Visitor as Expr, stmt::Visitor as Stmt};
}


pub struct ExprInfo {
    expr: Expr,
    line: usize,
    start: usize,
    end: usize,
}

pub struct StmtInfo {
    stmt: Stmt,
    line: usize,
    start: usize,
    end: usize,
}