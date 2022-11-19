use crate::{TokenType, LiteralType, Error};
use std::fmt;

mod visitor;
pub use visitor::Visitor;


#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: TokenType,
        args: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: String,
    },
    Grouping(Box<Expr>),
    Literal(LiteralType),
    Logical {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: String,
        value: Box<Expr>,
    },
    Super {
        name: String,
    },
    Unary {
        op: TokenType,
        right: Box<Expr>,
    },
    Variable(String),
}



impl Expr {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<(), Error> {
        match self {
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Binary { left, op, right } => visitor.visit_binary_expr(left, op, right),
            Expr::Call { callee, paren, args } => visitor.visit_call_expr(callee, paren, args),
            Expr::Get { object, name } => visitor.visit_get_expr(object, name),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(literal) => visitor.visit_literal_expr(literal),
            Expr::Logical { left, op, right } => visitor.visit_logical_expr(left, op, right),
            Expr::Set { object, name, value } => visitor.visit_set_expr(object, name, value),
            Expr::Super { name } => visitor.visit_super_expr(name),
            Expr::Unary { op, right } => visitor.visit_unary_expr(op, right),
            Expr::Variable(name) => visitor.visit_variable_expr(name),
        }
    }
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Assign { name, value } => write!(f, "(= {} {})", name, value),
            Expr::Binary { left, op, right } => write!(f, "({} {} {})", op, left, right),
            Expr::Call { callee, paren, args } => write!(f, "({} {} {:?})", callee, paren, args),
            Expr::Get { object, name } => write!(f, "({}.{})", object, name),
            Expr::Grouping(expr) => write!(f, "({})", expr),
            Expr::Literal(literal) => write!(f, "{:?}", literal),
            Expr::Logical { left, op, right } => write!(f, "({} {} {})", op, left, right),
            Expr::Set { object, name, value } => write!(f, "({}.{} = {})", object, name, value),
            Expr::Super { name } => write!(f, "super.{}", name),
            Expr::Unary { op, right } => write!(f, "({}{})", op, right),
            Expr::Variable(name) => write!(f, "{}", name),
        }
    }
}

