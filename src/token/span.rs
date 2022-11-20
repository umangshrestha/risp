use std::cmp;

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(line: usize, start: usize, end: usize) -> Self {
        Self { line, start, end }
    }

    pub fn merge(&self, other: &Span) -> Self {
        Self {
            line: self.line,
            start: cmp::min(self.start, other.start),
            end: cmp::max(self.end, other.end),
        }
    }
}