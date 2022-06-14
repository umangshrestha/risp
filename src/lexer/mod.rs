use crate::{Exception, Token};
use std::fs;

pub struct Lexer {
    /* information required for lexing */
    data: Vec<u8>,   // all the data in the file
    data_pos: usize, // postition of next data
    ch: u8,          // current charector
    /* information required for debugging */
    line_start: usize, // start position of current line in data
    filename: String,  // name of the file
    line_no: usize,    // current line number
    line_pos: usize,   // current position in line
}

// exported function
impl Lexer {
    pub fn from_file(filename: &str) -> Result<Self, Exception> {
        if let Ok(data) = fs::read(filename) {
            Ok(Lexer {
                filename: filename.to_string(),
                data: data,
                data_pos: 0,
                line_no: 1,
                line_pos: 0,
                ch: b' ',
                line_start: 0,
            })
        } else {
            Err(Exception::FileNotFound(filename.to_string()))
        }
    }

    pub fn new(data: &str) -> Result<Self, Exception> {
        Ok(Lexer {
            filename: "__main__".to_string(),
            data: data.as_bytes().to_vec(),
            data_pos: 0,
            line_no: 1,
            line_pos: 0,
            ch: b' ',
            line_start: 0,
        })
    }

    pub fn next_token(&mut self) -> Result<Token, Exception> {
        self.remove_whitespace();
        return if self.is_comment() {
            self.remove_comment();
            self.next_token()
        } else if self.is_number() {
            self.tokenize_number()
        } else if self.is_identifier() {
            self.tokenize_identifier()
        } else if self.is_string() {
            self.tokenize_string()
        } else {
            let data = self.tokenize_char();
            self._update_ch();
            data
        };
    }

    pub fn read_line(&mut self) -> String {
        while self.data_pos < self.data.len() && self.data[self.data_pos] != b'\n' {
            self.data_pos += 1;
        }
        return format!("{:?}", self.data);
    }
}

// helper function
impl Lexer {
    // reads the next postion in self.data and stores it in self.ch
    // stores 0 if data is not found
    fn _update_ch(&mut self) {
        // read the char while updating the position
        if self.data_pos >= self.data.len() {
            self.ch = b'\x00';
        } else {
            self.ch = self.data[self.data_pos];
            if self.ch == b'\n' {
                self.line_no += 1;
                self.line_pos = 0;
                self.line_start = self.data_pos;
            }
            self.data_pos += 1;
        }
    }

    // reduce next char pointer
    fn _undo_pos(&mut self) {
        self.data_pos -= 1;
    }

    // check if the next value for self.ch is what is expected
    // if true then modify the value of self.ch and seld.data_pos
    fn _is_next_ch(&mut self, ch: u8) -> bool {
        if self.data_pos < self.data.len() && self.data[self.data_pos] == ch {
            self._update_ch();
            return true;
        }
        return false;
    }
}

// converstion
impl Lexer {
    fn data_to_string(&self, from: usize, to: usize) -> Result<String, Exception> {
        match std::str::from_utf8(&self.data[from..to]) {
            Ok(val) => Ok(val.to_string()),
            Err(x) => Err(Exception::ValueError(x.to_string())),
        }
    }

    fn data_to_int(&self, from: usize, to: usize) -> Result<i64, Exception> {
        println!("{:?}", std::str::from_utf8(&self.data[from..to].to_vec()));

        let val = self.data_to_string(from, to)?;
        match val.parse::<i64>() {
            Ok(x) => Ok(x),
            Err(_) => Err(Exception::ValueError(format!(
                "invalid int literal \"{}\"",
                val
            ))),
        }
    }

    fn data_to_float(&self, from: usize, to: usize) -> Result<f64, Exception> {
        let val = self.data_to_string(from, to)?;
        match val.parse::<f64>() {
            Ok(x) => Ok(x),
            Err(_) => Err(Exception::ValueError(format!(
                "invalid float literal \"{}\"",
                val
            ))),
        }
    }
}

impl Lexer {
    fn tokenize_char(&mut self) -> Result<Token, Exception> {
        Ok(match self.ch {
            b'\x00' => Token::Eof,
            b',' => Token::Comma,
            b'[' => Token::LBrace,
            b']' => Token::RBrace,
            b'{' => Token::LCurly,
            b'}' => Token::RCurly,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b';' => Token::Semicolon,
            b'+' => {
                if self._is_next_ch(b'=') {
                    Token::PlusEq
                } else {
                    Token::Plus
                }
            }
            b'-' => {
                if self._is_next_ch(b'=') {
                    Token::SubEq
                } else {
                    Token::Minus
                }
            }
            b'*' => {
                if self._is_next_ch(b'=') {
                    Token::MulEq
                } else {
                    Token::Times
                }
            }
            b'/' => {
                if self._is_next_ch(b'=') {
                    Token::DivEq
                } else {
                    Token::Divide
                }
            }
            b'%' => {
                if self._is_next_ch(b'=') {
                    Token::ModEq
                } else {
                    Token::Mod
                }
            }
            b'=' => {
                if self._is_next_ch(b'=') {
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            b'!' => {
                if self._is_next_ch(b'=') {
                    Token::Ne
                } else {
                    Token::Not
                }
            }
            b'^' => {
                if self._is_next_ch(b'=') {
                    Token::XorEq
                } else {
                    Token::Xor
                }
            }
            b'&' => {
                if self._is_next_ch(b'=') {
                    Token::AndEq
                } else if self._is_next_ch(b'&') {
                    Token::LAnd
                } else {
                    Token::And
                }
            }
            b'|' => {
                if self._is_next_ch(b'=') {
                    Token::OrEq
                } else if self._is_next_ch(b'|') {
                    Token::LOr
                } else {
                    Token::Or
                }
            }
            b'<' => {
                if self._is_next_ch(b'=') {
                    Token::Le
                } else if self._is_next_ch(b'<') {
                    Token::LShift
                } else {
                    Token::Lt
                }
            }
            b'>' => {
                if self._is_next_ch(b'=') {
                    Token::Ge
                } else if self._is_next_ch(b'>') {
                    Token::RShift
                } else {
                    Token::Gt
                }
            }
            x => Token::Unknown(x),
        })
    }
    fn tokenize_number(&mut self) -> Result<Token, Exception> {
        let start_pos = self.data_pos - 1;
        let mut is_float = false;
        while self.is_number() {
            if self.ch == b'.' {
                is_float = true;
            }
            self._update_ch();
        }
        Ok(if is_float {
            Token::Float(self.data_to_float(start_pos, self.data_pos)?)
        } else {
            Token::Int(self.data_to_int(start_pos, self.data_pos - 1)?)
        })
    }

    fn tokenize_identifier(&mut self) -> Result<Token, Exception> {
        let start_pos = self.data_pos - 1;
        while self.is_identifier() {
            self._update_ch();
        }
        let val = self.data_to_string(start_pos, self.data_pos - 1)?;
        Ok(Token::lookup_identifiertype(val))
    }

    fn tokenize_string(&mut self) -> Result<Token, Exception> {
        let start_pos = self.data_pos - 1;
        self._update_ch();
        while self.ch != self.data[start_pos] {
            self._update_ch();
            if self.data_pos >= self.data.len() {
                return Err(Exception::SyntaxError(
                    "EOL while scanning string literal".to_string(),
                ));
            }
        }
        let val = self.data_to_string(start_pos + 1, self.data_pos - 1)?;
        self._update_ch();
        Ok(Token::String(val))
    }
}
// remover functions
impl Lexer {
    fn remove_whitespace(&mut self) {
        while self.is_whitespace() {
            self._update_ch();
        }
    }

    fn remove_comment(&mut self) {
        while self.ch != b'\n' && self.ch != b'\r' && !self.is_eof() {
            self._update_ch();
        }
    }
}

// predicate function
impl Lexer {
    fn is_whitespace(&self) -> bool {
        self.ch == b' ' || self.ch == b'\t' || self.ch == b'\r' || self.ch == b'\n'
    }

    fn is_number(&self) -> bool {
        b'0' <= self.ch && self.ch <= b'9' || self.ch == b'.'
    }

    fn is_hex(&self) -> bool {
        b'0' <= self.ch
            || self.ch <= b'9'
            || b'a' <= self.ch && self.ch <= b'f'
            || b'A' <= self.ch && self.ch <= b'X'
    }

    fn is_identifier(&self) -> bool {
        b'a' <= self.ch && self.ch <= b'z' || b'A' <= self.ch && self.ch <= b'Z' || self.ch == b'_'
    }

    fn is_string(&self) -> bool {
        self.ch == b'"' || self.ch == b'\''
    }
    fn is_comment(&self) -> bool {
        self.ch == b'#'
    }
    fn is_eof(&self) -> bool {
        self.ch == b'0'
    }
}

#[cfg(test)]
mod tests {
    // The syntax used in unittest may not represent the actual langauge system
    // these are just to test the basic functionality of lexer
    use super::Exception;
    use super::Lexer;
    use super::Token;

    #[test]
    fn test_hello_world() {
        let data = "println(\"Hello, World\");";

        let mut lex = Lexer::new(data).unwrap();

        let expected_tokens = vec![
            Token::Identifier("println".to_string()),
            Token::LParen,
            Token::String("Hello, World".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Eof,
        ];

        expected_tokens.iter().for_each(|expected| {
            let output = lex.next_token().unwrap();
            if &output != expected {
                panic!("Expected:{:?} Observed:{:?}", expected, output);
            }
        })
    }

    #[test]
    fn test_lexer_function() {
        let data = "
        # This is a comment
        let square = fn(a) {
            return a << 2; # equivalent to power(a, 2)
        }";

        let mut lex = Lexer::new(data).unwrap();

        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("square".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Identifier("a".to_string()),
            Token::RParen,
            Token::LCurly,
            Token::Return,
            Token::Identifier("a".to_string()),
            Token::LShift,
            Token::Int(2),
            Token::Semicolon,
            Token::RCurly,
            Token::Eof,
        ];

        expected_tokens.iter().for_each(|expected| {
            let output = lex.next_token().expect("Unexpected exception");
            if &output != expected {
                panic!("Expected:{:?} Observed:{:?}", expected, output);
            }
        })
    }

    #[test]
    fn test_float_error() {
        let data = "2.0.0";

        let mut lex = Lexer::new(data).unwrap();

        let expected = Exception::ValueError("invalid float literal \"2.0.0\"".to_string());

        match lex.next_token() {
            Ok(output) => panic!("Expected:{:?} Observed:{:?}", expected, output),
            Err(output) => {
                if output != expected {
                    panic!("Expected:{:?} Observed:{:?}", expected, output);
                }
            }
        }
    }
}
