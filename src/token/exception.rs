use crate::add_fmt_print;

#[derive(Debug, PartialEq)]
pub enum Exception {
    FileNotFound(String),
    ValueError(String),
    SyntaxError(String),
}

add_fmt_print!(Exception);

impl Exception {
    fn to_string(&self) -> String {
        match self {
            Exception::FileNotFound(msg) => format!("FileNotFoundException:{:?}", msg),
            Exception::ValueError(msg) => format!("ValueError:{:?}", msg),
            Exception::SyntaxError(msg) => format!("SyntaxError:{:?}", msg),
        }
    }
}
