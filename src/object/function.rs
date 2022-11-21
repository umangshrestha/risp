use std::{cell::RefCell, rc::Rc};

use crate::{Environment, Interpretor, Object, Span, Stmt};

pub enum Function {
    Inbuilt {
        arity: usize,
        func: Box<fn(Vec<Object>) -> Result<Object, ErrorInfo>>,
    },

    User {
        name: String,
        span: Span,
        params: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
}

impl Function {
    pub fn arity(&self) -> usize {
        match self {
            Function::Inbuilt { arity, .. } => *arity,
            Function::User { params, .. } => params.len(),
        }
    }

    fn call(interpreter: &mut Interpretor, arguments: Vec<Object>) -> Object {
        match self {
            Function::Inbuilt { func, .. } => func(arguments),
            Function::User { params, closure, .. } => {
                let mut environment = Environment::new_with_enclosing(closure.clone());
                for (param, argument) in params.iter().zip(arguments) {
                    environment.define(param.name.clone(), argument);
                }
                interpreter.execute_block(&self.body, environment);
            }
        }
    }

}
