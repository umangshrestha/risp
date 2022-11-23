use std::{cell::RefCell, rc::Rc};

use crate::{Environment, Error, ErrorInfo, Interpretor, Object, Span, Stmt, environment};

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    Inbuilt {
        arity: usize,
        func: Box<fn(Vec<Object>) -> Result<Object, ErrorInfo>>,
    },

    User {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
        span: Span,
    },
}

impl Function {
    pub fn arity(&self) -> usize {
        match self {
            Function::Inbuilt { arity, .. } => *arity,
            Function::User { params, .. } => params.len(),
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpretor,
        args: &Vec<Object>,
    ) -> Result<Object, ErrorInfo> {
        match self {
            Function::Inbuilt { func, .. } => Ok(func(args.to_vec())?),
            Function::User {
                params,
                body,
                closure,
                ..
            } => {
                let mut environment = Environment::new_from_closure(closure);
                for (param, argument) in params.iter().zip(args) {
                    environment
                        .define(param
                            .clone(), argument.to_owned(), false);
                }
                let environment = Rc::new(RefCell::new(environment));
                match  interpreter.exec_block(body, environment) {
                Ok(()) => Ok(Object::Nil),
                Err(x) => {
                    if let Error::Return(value) = x.error {
                        Ok(value)
                    } else {
                        Err(x)
                    }                }
                }
               
            }
        }
    }
}
