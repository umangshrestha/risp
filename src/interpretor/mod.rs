use crate::{LiteralType, Object, Expr, Error, Stmt, TokenType, ErrorInfo};
mod expr;
mod stmt;
mod environment;

pub struct Interpretor {

}

impl Interpretor {

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            let res = self.exec(&stmt);
            if res.is_err() {
                res.err().unwrap().report();
            }
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<Object, ErrorInfo> {
        expr.accept(self)
    }

    pub fn exec(&mut self, stmt: &Stmt) -> Result<(), ErrorInfo> {
        stmt.accept(self)
    }
}

