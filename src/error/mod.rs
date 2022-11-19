mod error;
pub use error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorInfo {
    error: Error,
    line: usize,
    start: usize,
    end: usize,
}

impl ErrorInfo {
    pub fn new(error: Error, line: usize, start: usize, end: usize) -> Self {
        Self {
            error,
            line,
            start,
            end,
        }
    }

    pub fn report(&self) {
        eprintln!("{:?}", self);
    }
}