use super::{Identifier, Literal, Program};
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
        truth_block: Program,
        false_block: Option<Program>,
    },
    Function {
        name: Identifier,
        arguments: Vec<Identifier>,
        body: Program,
    },
    Call {
        function: Box<Expression>,
        argument: Vec<Expression>,
    },
    Index {
        array: Box<Expression>,
        index: Vec<Expression>,
    },
    HashMap(Vec<(Literal, Expression)>),
    Array(Vec<Expression>),
}
