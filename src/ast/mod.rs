use crate::Token;

mod expression;
pub use expression::Expression;

mod literal;
pub use literal::Literal;


pub type Program = Vec<Statement>;

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(pub String);

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
}
