use super::{Identifier, Literal, Statement};
use crate::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix {
        op: Token,
        lhs: Box<Expression>,
    },
    Infix {
        op: Token,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        truth_block: Vec<Statement>,
        false_block: Option<Vec<Statement>>,
    },
    Function {
        name: Identifier,
        arguments: Vec<Identifier>,
        body: Vec<Statement>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Index {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    HashMap(Vec<(Expression, Expression)>),
    Array(Vec<Expression>),
}
