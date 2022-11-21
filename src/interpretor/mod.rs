use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::Program, Environment, Error, ErrorInfo, Expr, LiteralType, Object, Stmt, TokenType,
};
mod expr;
mod stmt;

pub struct Interpretor {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
    pub locals: std::collections::HashMap<Expr, usize>,
}

impl Interpretor {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        let environment = globals.clone();
        Self {
            globals,
            environment,
            locals: std::collections::HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) {
        for stmt in program.stmts {
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
