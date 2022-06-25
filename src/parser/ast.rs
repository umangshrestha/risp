#[derive[Debug, PartialEq]]
pub enum Literal {
    Int(i64),
    Bool(bool),
    String(String),
    Array(Box<ArrayLiteral>),
    Hash(Box<HashLiteral>),
}

#[derive[Debug, PartialEq]]
pub enum Expression {
    Identifier(String),
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
    If(Box<IfExpression>),
    Function(Box<FunctionLiteral>),
    Index(Box<IndexExpression>)
}