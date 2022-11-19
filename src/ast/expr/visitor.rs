use crate::{LiteralType, TokenType, Error, Expr};

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

