use crate::token::{self, TokenInfo, TokenType};
use crate::{Error, ErrorInfo};

pub struct Lexer {
    start: usize,
    curr: usize,
    line: usize,
    data: Vec<char>,
}

//  methods
impl Lexer {
    pub fn new(data: String) -> Self {
        Self {
            start: 0,
            curr: 0,
            line: 1,
            data: data.chars().collect(),
        }
    }

    pub fn next(&mut self) -> TokenInfo {
        loop {
            let result = self.scan();
            if result.is_ok() {
                return TokenInfo::new(result.unwrap(), self.start, self.curr, self.line);
            } else {
                let err = ErrorInfo::new(result.unwrap_err(), self.line, self.start, self.curr);
                err.report();
            }
        }
    }

    pub fn scan(&mut self) -> Result<TokenType, Error> {
        self.start = self.curr;
        match self.next_char() {
            '\0' => Ok(TokenType::Eof),
            ',' => Ok(TokenType::Comma),
            '[' => Ok(TokenType::LBrace),
            ']' => Ok(TokenType::RBrace),
            '{' => Ok(TokenType::LCurly),
            '}' => Ok(TokenType::RCurly),
            '(' => Ok(TokenType::LParen),
            ')' => Ok(TokenType::RParen),
            ';' => Ok(TokenType::Semicolon),
            '+' => {
                if self.is_next_char('=') {
                    Ok(TokenType::PlusEq)
                } else {
                    Ok(TokenType::Plus)
                }
            }
            '-' => {
                if self.is_next_char('=') {
                    Ok(TokenType::SubEq)
                } else {
                    Ok(TokenType::Minus)
                }
            }
            '*' => {
                if self.is_next_char('=') {
                    Ok(TokenType::MulEq)
                } else {
                    Ok(TokenType::Times)
                }
            }
            '/' => {
                if self.is_next_char('=') {
                    Ok(TokenType::DivEq)
                } else {
                    Ok(TokenType::Divide)
                }
            }
            '%' => {
                if self.is_next_char('=') {
                    Ok(TokenType::ModEq)
                } else {
                    Ok(TokenType::Mod)
                }
            }
            '=' => {
                if self.is_next_char('=') {
                    Ok(TokenType::Eq)
                } else {
                    Ok(TokenType::Assign)
                }
            }
            '!' => {
                if self.is_next_char('=') {
                    Ok(TokenType::Ne)
                } else {
                    Ok(TokenType::Not)
                }
            }
            '^' => {
                if self.is_next_char('=') {
                    Ok(TokenType::XorEq)
                } else {
                    Ok(TokenType::Xor)
                }
            }
            '&' => {
                if self.is_next_char('=') {
                    Ok(TokenType::AndEq)
                } else if self.is_next_char('&') {
                    Ok(TokenType::LAnd)
                } else {
                    Ok(TokenType::And)
                }
            }
            '|' => {
                if self.is_next_char('=') {
                    Ok(TokenType::OrEq)
                } else if self.is_next_char('|') {
                    Ok(TokenType::LOr)
                } else {
                    Ok(TokenType::Or)
                }
            }
            '<' => {
                if self.is_next_char('=') {
                    Ok(TokenType::Lte)
                } else if self.is_next_char('<') {
                    Ok(TokenType::LShift)
                } else {
                    Ok(TokenType::Lt)
                }
            }
            '>' => {
                if self.is_next_char('=') {
                    Ok(TokenType::Gte)
                } else if self.is_next_char('>') {
                    Ok(TokenType::RShift)
                } else {
                    Ok(TokenType::Gt)
                }
            }
            '\"' => {
                while self.peek_char() != '\"' {
                    if self.is_eof() {
                        return Err(Error::Syntax(format!("unterminated string",)));
                    }
                    self.next_char();
                }
                let data = self.data[self.start..self.curr]
                    .into_iter()
                    .collect::<String>();
                Ok(TokenType::String(data))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                while (self.is_digit(self.peek_char()) || self.is_identifier(self.peek_char()))
                    && !self.is_eof()
                {
                    self.next_char();
                }

                let data = self.data[self.start..self.curr]
                    .into_iter()
                    .collect::<String>();
                Ok(token::lookup_identifier(data))
            }
            ' ' | '\r' | '\t' | '\n' => {
                while self.is_whitespace(self.peek_char()) {
                    self.next_char();
                }
                return self.scan();
            }
            '0'..='9' => {
                let start = self.curr;
                while self.is_digit(self.peek_char()) || self.peek_char() == '.' {
                    self.next_char();
                }
                let data = self.data[start - 1..self.curr]
                    .into_iter()
                    .collect::<String>();
                match data.parse::<f64>() {
                    Ok(x) => Ok(TokenType::Number(x)),
                    Err(_) => Err(Error::Value(format!("invalid number:'{}'", data,))),
                }
            }
            ch => Err(Error::Syntax(format!("unknown character:'{ch}'"))),
        }
    }
}

impl Lexer {
    fn is_eof(&self) -> bool {
        self.curr >= self.data.len()
    }

    fn next_char(&mut self) -> char {
        if self.is_eof() {
            return '\x00';
        }
        let ch = self.data[self.curr];
        if ch == '\n' {
            self.line += 1;
        }
        self.curr += 1;
        ch
    }

    fn peek_char(&self) -> char {
        if self.is_eof() {
            return '\x00';
        }
        self.data[self.curr]
    }

    fn is_digit(&self, ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }

    fn is_identifier(&self, ch: char) -> bool {
        return ('a' <= ch && ch <= 'z') || ('a' <= ch && ch <= 'z') || '_' == ch;
    }

    fn is_next_char(&mut self, ch: char) -> bool {
        if self.peek_char() == ch {
            self.next_char();
            return true;
        }
        return false;
    }

    fn is_whitespace(&self, ch: char) -> bool {
        return ch == ' ' || ch == '\r' || ch == '\t' || ch == '\n';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexers(input: &str, expected: Vec<TokenType>) {
        let mut lexer = Lexer::new(input.to_string());
        let mut result = Vec::new();
        loop {
            let token = lexer.scan();
            match token {
                Ok(TokenType::Eof) => break,
                Ok(x) => result.push(x),
                Err(x) => panic!("Err:{x}"),
            }
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn test_symbols() {
        let input = "(){}[],;+-*/% =&|!^<>
        == != <= >= && || += -= *= /= %= ^= << >>";
        let expected = vec![
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LCurly,
            TokenType::RCurly,
            TokenType::LBrace,
            TokenType::RBrace,
            TokenType::Comma,
            TokenType::Semicolon,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Times,
            TokenType::Divide,
            TokenType::Mod,
            TokenType::Assign,
            TokenType::And,
            TokenType::Or,
            TokenType::Not,
            TokenType::Xor,
            TokenType::Lt,
            TokenType::Gt,
            TokenType::Eq,
            TokenType::Ne,
            TokenType::Lte,
            TokenType::Gte,
            TokenType::LAnd,
            TokenType::LOr,
            TokenType::PlusEq,
            TokenType::SubEq,
            TokenType::MulEq,
            TokenType::DivEq,
            TokenType::ModEq,
            TokenType::XorEq,
            TokenType::LShift,
            TokenType::RShift,
        ];
        test_lexers(input, expected);
    }

    #[test]
    fn test_identifier() {
        let input = "if else while for return abc def_ _ghi";
        let expected = vec![
            TokenType::If,
            TokenType::Else,
            TokenType::While,
            TokenType::For,
            TokenType::Return,
            TokenType::Identifier("abc".to_string()),
            TokenType::Identifier("def_".to_string()),
            TokenType::Identifier("_ghi".to_string()),
        ];
        test_lexers(input, expected);
    }

    #[test]
    fn test_assignment() {
        let input = "";
        let expected = vec![];
        test_lexers(input, expected);
    }

    #[test]
    fn test_function() {
        let input = "fn add(a, b) {
            return a + b; 
        }
        let a = 1;
        add(a, 2);";
        let expected = vec![
            TokenType::Function,
            TokenType::Identifier("add".to_string()),
            TokenType::LParen,
            TokenType::Identifier("a".to_string()),
            TokenType::Comma,
            TokenType::Identifier("b".to_string()),
            TokenType::RParen,
            TokenType::LCurly,
            TokenType::Return,
            TokenType::Identifier("a".to_string()),
            TokenType::Plus,
            TokenType::Identifier("b".to_string()),
            TokenType::Semicolon,
            TokenType::RCurly,
            TokenType::Let,
            TokenType::Identifier("a".to_string()),
            TokenType::Assign,
            TokenType::Number(1.0),
            TokenType::Semicolon,
            TokenType::Identifier("add".to_string()),
            TokenType::LParen,
            TokenType::Identifier("a".to_string()),
            TokenType::Comma,
            TokenType::Number(2.0),
            TokenType::RParen,
            TokenType::Semicolon,
        ];
        test_lexers(input, expected);
    }

    #[test]
    fn test_unknown_character() {
        let input = "@ 1.2.3 \"this is untermintated string";
        let expected = vec![
            Error::Syntax(format!("unknown character:'@'")),
            Error::Value(format!("invalid number:'1.2.3'")),
            Error::Syntax(format!("unterminated string")),
        ];
        let mut lexer = Lexer::new(input.to_string());
        let mut result = Vec::new();
        loop {
            let token = lexer.scan();
            match token {
                Ok(TokenType::Eof) => break,
                Ok(x) => panic!("Err:{x}"),
                Err(ref x) => result.push(x.clone()),
            }
        }
        assert_eq!(result, expected);
    }
}
