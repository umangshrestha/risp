use crate::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub struct TokenInfo {
    pub token: TokenType,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl TokenInfo {
    pub fn new(token: TokenType, start: usize, end: usize, line: usize) -> Self {
        Self {
            token,
            start,
            end,
            line,
        }
    }
    pub fn is(&self, token: TokenType) -> bool {
        self.token == token
    }
}