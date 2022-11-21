use crate::{ErrorInfo, Expr, Object, Span};

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
        span: Span,
    },
    Block {
        stmts: Vec<Stmt>,
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
        span: Span,
    },
    Return {
        value: Option<Expr>,
        span: Span,
    },
    Class {
        name: String,
        super_class: Option<String>,
        methods: Vec<Stmt>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
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
                span,
            } => visitor.visit_let_stmt(name, value, *is_const, span),
            Stmt::Block { stmts } => visitor.visit_block_stmt(stmts),
            Stmt::If {
                condition,
                truthy,
                falsy,
            } => visitor.visit_if_stmt(condition, truthy, falsy),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Function {
                name,
                params,
                body,
                span,
            } => visitor.visit_function_stmt(name, params, body, span),
            Stmt::Return { value, span } => visitor.visit_return_stmt(value, span),
            Stmt::Class {
                name,
                super_class,
                methods,
                span,
            } => visitor.visit_class_stmt(name, super_class, methods, span),
            Stmt::Break { span } => visitor.visit_break_stmt(span),
            Stmt::Continue { span } => visitor.visit_continue_stmt(span),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr { expr } => write!(f, "{}", expr),
            Stmt::Print { expr } => write!(f, "(print {})", expr),
            Stmt::Let {
                name,
                value,
                is_const,
                span: _,
            } => {
                if *is_const {
                    write!(f, "(const {} {})", name, value.as_ref().unwrap())
                } else {
                    write!(f, "(let {} {})", name, value.as_ref().unwrap())
                }
            }
            Stmt::Block { stmts } => {
                let mut s = String::new();
                s.push_str("(");
                for stmt in stmts {
                    s.push_str(&format!("{}", stmt));
                }
                write!(f, "{})", s)
            }
            Stmt::If {
                condition,
                truthy,
                falsy,
            } => {
                write!(f, "(if {condition} then {truthy}")?;
                if let Some(else_block) = falsy {
                    write!(f, " else {else_block})")?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => write!(f, "(while ({}) {})", condition, body),
            Stmt::Function {
                name,
                params,
                body,
                span: _,
            } => {
                let mut s = String::new();
                s.push_str(&format!("fun {} (", name));
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
            Stmt::Return { value, span: _ } => {
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
                span: _,
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
            Stmt::Break { span: _ } => write!(f, "break"),
            Stmt::Continue { span: _ } => write!(f, "continue"),
        }
    }
}
