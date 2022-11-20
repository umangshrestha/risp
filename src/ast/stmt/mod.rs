use crate::{ErrorInfo, Expr, Span};

mod visitor;
pub use visitor::Visitor;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expr {
        expr: Expr,
        span: Span,
    },
    Print {
        expr: Expr,
        span: Span,
    },
    Let {
        name: String,
        value: Option<Expr>,
        is_const: bool,
        span: Span,
    },
    Block {
        stmt: Vec<Stmt>,
        span: Span,
    },
    If {
        condition: Expr,
        truthy: Box<Stmt>,
        falsy: Option<Box<Stmt>>,
        span: Span,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
        span: Span,
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
    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<Stmt>,
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
            Stmt::Expr { expr, span } => visitor.visit_expr_stmt(expr, span),
            Stmt::Print { expr, span } => visitor.visit_print_stmt(expr, span),
            Stmt::Let {
                name,
                value,
                is_const,
                span,
            } => visitor.visit_let_stmt(name, value, is_const, span),
            Stmt::Block { stmt, span } => visitor.visit_block_stmt(stmt, span),
            Stmt::If {
                condition,
                truthy,
                falsy,
                span,
            } => visitor.visit_if_stmt(condition, truthy, falsy, span),
            Stmt::While {
                condition,
                body,
                span,
            } => visitor.visit_while_stmt(condition, body, span),
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
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
                span,
            } => visitor.visit_for_stmt(initializer, condition, increment, body, span),
            Stmt::Break { span } => visitor.visit_break_stmt(span),
            Stmt::Continue { span } => visitor.visit_continue_stmt(span),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr { expr, span: _ } => write!(f, "{}", expr),
            Stmt::Print { expr, span: _ } => write!(f, "(print {})", expr),
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
            Stmt::Block { stmt, span: _ } => {
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
                span: _,
            } => {
                write!(f,"(if ({condition}) {truthy}")?;
                if let Some(else_block) = falsy {
                    write!(f," else {else_block})")?;
                }
                Ok(())
            }
            Stmt::While {
                condition,
                body,
                span: _,
            } => write!(f, "(while ({}) {})", condition, body),
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
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
                span: _,
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
            Stmt::Break { span: _ } => write!(f, "break"),
            Stmt::Continue { span: _ } => write!(f, "continue"),
        }
    }
}
