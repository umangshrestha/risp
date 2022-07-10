#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Times,
    Eq,
    Ne,
    Gte,
    Gt,
    Lt,
    Lte,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prefix {
    Plus,
    Minus,
    Bang,
}
