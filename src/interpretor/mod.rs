use std::{cell::RefCell, rc::Rc};

use crate::{LiteralType, Object, Expr, Error, Stmt, TokenType, ErrorInfo, Environment};
mod expr;
mod stmt;

pub struct Interpretor {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    pub locals: std::collections::HashMap<Expr, usize>,
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

