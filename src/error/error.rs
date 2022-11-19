use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    Syntax(String),
    Value(String),
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Syntax(x) => write!(f, "SyntaxError: {x}"),
            Error::Value(x) => write!(f, "ValueError: {x}"),
            Error::Parse(x) => write!(f, "ParseError: {x}"),
        }
    }
}
