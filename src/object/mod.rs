use std::fmt;
pub mod utils;
// pub mod function;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
            Object::Nil => write!(f, "nil"),
        }
    }
}
