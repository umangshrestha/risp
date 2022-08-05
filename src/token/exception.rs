use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Traceback {
    pub line_no: usize,
    pub pos: usize,
    pub line: String,
    pub filename: String,
    pub exception: Exception,
}

impl fmt::Display for Traceback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File:\"{}\", line:{}, pos:{}\n{}\n{}",
            self.filename, self.line_no, self.pos, self.line, self.exception
        )
    }
}

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
