use crate::Error;
pub use crate::LiteralType;
pub use crate::TokenType;

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

pub trait  Visitor {
    fn visit_assign_expr(&mut self, name: &String, value: &Box<Expr>) -> Result<(), Error>;
    fn visit_binary_expr(&mut self, left: &Box<Expr>, op: &TokenType, right: &Box<Expr>) -> Result<(), Error>;
    fn visit_call_expr(&mut self, callee: &Box<Expr>, paren: &TokenType, args: &Vec<Expr>) -> Result<(), Error>;
    fn visit_get_expr(&mut self, object: &Box<Expr>, name: &String) -> Result<(), Error>;
    fn visit_grouping_expr(&mut self, expr: &Box<Expr>) -> Result<(), Error>;
    fn visit_literal_expr(&mut self, value: &LiteralType) -> Result<(), Error>;
    fn visit_logical_expr(&mut self, left: &Box<Expr>, op: &TokenType, right: &Box<Expr>) -> Result<(), Error>;
    fn visit_set_expr(&mut self, object: &Box<Expr>, name: &String, value: &Box<Expr>) -> Result<(), Error>;
    fn visit_super_expr(&mut self, name: &String) -> Result<(), Error>;
    fn visit_unary_expr(&mut self, op: &TokenType, right: &Box<Expr>) -> Result<(), Error>;
    fn visit_variable_expr(&mut self, name: &String) -> Result<(), Error>;
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