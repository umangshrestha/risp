use std::fmt;

#[derive(PartialEq, Clone)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl fmt::Debug for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{}", s),
            LiteralType::Number(n) => write!(f, "{}", n),
            LiteralType::Boolean(b) => write!(f, "{}", b),
            LiteralType::Nil => write!(f, "nil"),
        }
    }
}