use std::fmt;

pub enum Object {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Object {
    pub fn to_boolean(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            Object::Number(n) => *n != 0.0,
            Object::String(s) => !s.is_empty(),
        }
    }
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
