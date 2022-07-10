use super::{Identifier, Infix, Literal, Prefix, Program};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    If {
        condition: Box<Expression>,
        truthy: Program,
        falsy: Option<Program>,
    },
    Function {
        argument: Vec<Identifier>,
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
    Array(Vec<(Literal, Expression)>),
}
