use std::fmt;
mod function;
pub mod utils;
pub use function::Function;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Boolean(bool),
    Number(f64),
    String(String),
    Function(Function),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
            Object::Nil => write!(f, "nil"),
            Object::Function(_) => write!(f, "<function>"),
        }
    }
}
