use std::cmp;

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub line: usize,
    pub line_start: usize,
    pub start: usize, // this is the postion wrt to data in form of string
    pub end: usize,
}

impl Span {
    pub fn new(line: usize, line_start: usize, start: usize, end: usize) -> Self {
        Self { line, line_start, start, end}
    }

    pub fn merge(&self, other: &Span) -> Self {
        Self {
            line: self.line,
            line_start: cmp::min(self.line_start, other.line_start),
            start: cmp::min(self.start, other.start),
            end: cmp::max(self.end, other.end),
        }
    }
}