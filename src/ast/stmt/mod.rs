use crate::{ErrorInfo, Expr};

mod visitor;
pub use visitor::Visitor;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expr {
        expr: Expr,
    },
    Print {
        expr: Expr,
    },
    Let {
        name: String,
        value: Option<Expr>,
        is_const: bool,
    },
    Block {
        stmt: Vec<Stmt>,
    },
    If {
        condition: Expr,
        truthy: Box<Stmt>,
        falsy: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
    },
    Return {
        value: Option<Expr>,
    },
    Class {
        name: String,
        super_class: Option<String>,
        methods: Vec<Stmt>,
    },
    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<Box<Expr>>,
        increment: Option<Box<Expr>>,
        body: Box<Stmt>,
    },
    Break,
    Continue,
}

impl Stmt {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), ErrorInfo> {
        match self {
            Stmt::Expr { expr } => visitor.visit_expr_stmt(expr),
            Stmt::Print { expr } => visitor.visit_print_stmt(expr),
            Stmt::Let {
                name,
                value,
                is_const,
            } => visitor.visit_let_stmt(name, value, *is_const),
            Stmt::Block { stmt } => visitor.visit_block_stmt(stmt),
            Stmt::If {
                condition,
                truthy,
                falsy,
            } => visitor.visit_if_stmt(condition, truthy, falsy),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Function { name, params, body } => {
                visitor.visit_function_stmt(name, params, body)
            }
            Stmt::Return { value } => visitor.visit_return_stmt(value),
            Stmt::Class {
                name,
                super_class,
                methods,
            } => visitor.visit_class_stmt(name, super_class, methods),
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
            } => visitor.visit_for_stmt(initializer, condition, increment, body),
            Stmt::Break => visitor.visit_break_stmt(),
            Stmt::Continue => visitor.visit_continue_stmt(),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr { expr } => write!(f, "{}", expr),
            Stmt::Print { expr } => write!(f, "print {}", expr),
            Stmt::Let {
                name,
                value,
                is_const,
            } => {
                if *is_const {
                    write!(f, "(const {} {})", name, value.as_ref().unwrap())
                } else {
                    write!(f, "(let {} {})", name, value.as_ref().unwrap())
                }
            }
            Stmt::Block { stmt } => {
                let mut s = String::new();
                s.push_str("(");
                for stmt in stmt {
                    s.push_str(&format!("{}\n", stmt));
                }
                write!(f, "{}\n)", s)
            }
            Stmt::If {
                condition,
                truthy,
                falsy,
            } => {
                let mut s = String::new();
                s.push_str(&format!("if ({}) {}", condition, truthy));
                if let Some(falsy) = falsy {
                    s.push_str(&format!(" else {}", falsy));
                }
                write!(f, "{}", s)
            }
            Stmt::While { condition, body } => write!(f, "while ({}) {}", condition, body),
            Stmt::Function { name, params, body } => {
                let mut s = String::new();
                s.push_str(&format!("fun {}(", name));
                for (i, param) in params.iter().enumerate() {
                    if i != 0 {
                        s.push_str(", ");
                    }
                    s.push_str(param);
                }
                s.push_str(") ");
                s.push_str(&format!("{}", body));
                write!(f, "{}", s)
            }
            Stmt::Return { value } => {
                if let Some(value) = value {
                    write!(f, "return {}", value)
                } else {
                    write!(f, "return")
                }
            }
            Stmt::Class {
                name,
                super_class,
                methods,
            } => {
                let mut s = String::new();
                s.push_str(&format!("class {} ", name));
                if let Some(super_class) = super_class {
                    s.push_str(&format!("extends {} ", super_class));
                }
                s.push_str("{\n");
                for method in methods {
                    s.push_str(&format!("{}\n", method));
                }
                s.push_str("}");
                write!(f, "{}", s)
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                let mut s = String::new();
                s.push_str("for (");
                if let Some(initializer) = initializer {
                    s.push_str(&format!("{}", initializer));
                }
                s.push_str("; ");
                if let Some(condition) = condition {
                    s.push_str(&format!("{}", condition));
                }
                s.push_str("; ");
                if let Some(increment) = increment {
                    s.push_str(&format!("{}", increment));
                }
                s.push_str(") ");
                s.push_str(&format!("{}", body));
                write!(f, "{}", s)
            }
            Stmt::Break => write!(f, "break"),
            Stmt::Continue => write!(f, "continue"),
        }
    }
}
