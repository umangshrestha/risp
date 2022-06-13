use std::fmt;


#[derive(Debug)]
pub enum Exception {
    FileNotFound(String),
    ValueError(String)
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exception::FileNotFound(msg)   => write!(f, "FileNotFoundException: {:?}", msg),
            Exception::ValueError(msg)   => write!(f, "ValueError:\"{:?}\"", msg)  
        }
    }
}


