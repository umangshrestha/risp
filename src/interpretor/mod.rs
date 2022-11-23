use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::Program, object::Function, Environment, Error, ErrorInfo, Expr, LiteralType, Object, Stmt,
    TokenType,
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
        let time = Object::Function(Function::Inbuilt {
            arity: 0,
            func: Box::new(|_args| {
                let time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as f64;
                Ok(Object::Number(time as f64))
            }),
        });
        globals.borrow_mut().define("time".to_string(), time, true).unwrap();
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

    pub fn exec_block(
        &mut self,
        stmts: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), ErrorInfo> {
        let parent = self.environment.clone();
        self.environment = environment;

        let result = (|| -> Result<(), ErrorInfo> {
            for stmt in stmts {
                self.exec(stmt)?;
            }
            Ok(())
        })();

        self.environment = parent;
        result
    }
}

impl Default for Interpretor {
    fn default() -> Self {
        Self::new()
    }
}
