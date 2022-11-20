mod error;
pub use error::Error;

use crate::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorInfo {
    error: Error,
    span: Span,
}

impl ErrorInfo {
    pub fn new(error: Error, line: usize, start: usize, end: usize) -> Self {
        Self {
            error,
            span: Span::new(line, start, end),
        }
    }

    pub fn new_with_span(error: Error, span: Span) -> Self {
        Self {
            error,
            span,
        }
    }

    pub fn report(&self) {
        eprintln!("{:?}", self);
    }
}
