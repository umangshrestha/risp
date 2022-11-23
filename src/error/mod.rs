mod error;
pub use error::Error;

use crate::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorInfo {
    pub error: Error,
    span: Span,
}

impl ErrorInfo {
    pub fn new(error: Error, line: usize, line_start: usize, start: usize, end: usize) -> Self {
        Self {
            error,
            span: Span::new(line, line_start, start, end),
        }
    }

    pub fn new_with_span(error: Error, span: Span) -> Self {
        Self {
            error,
            span,
        }
    }

    pub fn report(&self) {
        eprintln!("{}, line {}, pos {}", self.error, self.span.line, self.span.start - self.span.line_start);
    }
}
