use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Exception {
    FileNotFound(String),
    ValueError(String),
    SyntaxError(String),
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
