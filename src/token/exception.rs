use std::fmt;

#[derive(Debug)]
pub enum Exception {
    FileNotFoundException(String)
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exception::FileNotFoundException(x)   => write!(f, "FileNotFoundException: \"{:?}\" does not exist", x),
        }
    }
}
