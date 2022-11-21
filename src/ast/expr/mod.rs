use crate::{ErrorInfo, LiteralType, Object, Span, TokenType, TokenInfo};
use std::fmt;

mod visitor;
pub use visitor::Visitor;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Assign {
        name: String,
        value: Box<Expr>,
        span: Span,
    },
    Binary {
        left: Box<Expr>,
        op: TokenInfo,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: TokenType,
        args: Vec<Expr>,
        span: Span,
    },
    Get {
        object: Box<Expr>,
        name: String,
        span: Span,
    },
    Grouping {
        expr: Box<Expr>,
        span: Span,
    },
    Literal {
        value: LiteralType,
    },
    Set {
        object: Box<Expr>,
        name: String,
        value: Box<Expr>,
        span: Span,
    },
    Super {
        name: String,
        span: Span,
    },
    Unary {
        op: TokenInfo,
        right: Box<Expr>,
    },
    Variable {
        name: String,
        span: Span,
    },
}

impl Expr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<Object, ErrorInfo> {
        match self {
            Expr::Assign { name, value, span } => visitor.visit_assign_expr(name, value, span),
            Expr::Binary {
                left,
                op,
                right,
            } => visitor.visit_binary_expr(left, op, right),
            Expr::Call {
                callee,
                paren,
                args,
                span,
            } => visitor.visit_call_expr(callee, paren, args, span),
            Expr::Get { object, name, span } => visitor.visit_get_expr(object, name, span),
            Expr::Grouping { expr, span } => visitor.visit_grouping_expr(expr, span),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Set {
                object,
                name,
                value,
                span,
            } => visitor.visit_set_expr(object, name, value, span),
            Expr::Super { name, span } => visitor.visit_super_expr(name, span),
            Expr::Unary { op, right} => visitor.visit_unary_expr(op, right),
            Expr::Variable { name, span } => visitor.visit_variable_expr(name, span),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Assign {
                name,
                value,
                span: _,
            } => write!(f, "(= {name} {value})"),
            Expr::Binary {
                left,
                op,
                right,
            } => write!(f, "({} {left} {right})", op.token),
            Expr::Call {
                callee,
                paren: _,
                args,
                span: _,
            } => write!(f, "(call {callee} {:?})", args),
            Expr::Get {
                object,
                name,
                span: _,
            } => write!(f, "(get {object} {name})"),
            Expr::Grouping { expr, span: _ } => write!(f, "{expr}"),
            Expr::Literal { value } => write!(f, "{:?}", value),
            Expr::Set {
                object,
                name,
                value,
                span: _,
            } => write!(f, "(set {object} {name} {value})"),
            Expr::Super { name, span: _ } => write!(f, "(super {name})"),
            Expr::Unary { op, right} => write!(f, "({} {right})", op.token),
            Expr::Variable { name, span: _ } => write!(f, "{name}"),
        }
    }
}
