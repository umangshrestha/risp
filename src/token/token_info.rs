use crate::{TokenType, Span};

#[derive(Debug, PartialEq, Clone)]
pub struct TokenInfo {
    pub token: TokenType,
    pub span: Span,
}

impl TokenInfo {
    pub fn new(token: TokenType, start: usize, end: usize, line: usize, line_start: usize) -> Self {
       let span = Span {start, end, line, line_start };
        Self {
            token,
            span
        }
    }
    pub fn is(&self, token: TokenType) -> bool {
        self.token == token
    }
}