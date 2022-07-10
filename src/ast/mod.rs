// mod statement;
// pub use statement::{Statement, BoxStatement};

mod expression;
pub use expression::Expression;

mod literal;
pub use literal::Literal;

mod notation;
use notation::{Infix, Prefix};



pub type Program = Vec<Statement>;

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(String);


#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
}