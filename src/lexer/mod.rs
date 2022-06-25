use crate::{Exception, Token};
use std::fs;

pub struct Lexer {
    /* information required for lexing */
    data: Vec<char>,   // all the data in the file
    data_pos: usize, // postition of next data
    ch: char,          // current charector
    /* information required for debugging */
    filename: String,  // name of the file
    line_no: usize,    // current line number
}

// exported function
impl Lexer {
    pub fn from_file(filename: &str) -> Result<Self, Exception> {
        if let Ok(data) = fs::read_to_string(filename) {
            let mut lex = Lexer {
                filename: filename.to_string(),
                data: data.chars().collect(),
                data_pos: 0,
                line_no: 1,
                ch: '\x00',
            };
            lex._update_ch();
            Ok(lex)            
        } else {
            Err(Exception::FileNotFound(filename.to_string()))
        }
    }

    pub fn new(data: String) -> Result<Self, Exception> {
        let mut lex = Lexer {
            filename: "__main__".to_string(),
            data: data.chars().collect(),
            data_pos: 0,
            line_no: 1,
            ch: '\x00',
        };
        lex._update_ch();
        Ok(lex)       
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
}

// helper function
impl Lexer {
    // reads the next postion in self.line and stores it in self.ch
    // stores 0 if data is not found
    fn _update_ch(&mut self) {
        // read the char while updating the position
        if self.data_pos < self.data.len() {
            self.ch = self.data[self.data_pos];
            if self.ch == '\n' {
                self.line_no += 1;
            }
            self.data_pos+=1;        
        } else {

            self.ch = '\x00'
        };
    }

    // reduce next char pointer
    fn _undo_pos(&mut self) {
        self.data_pos -= 1;
    }

    // check if the next value for self.ch is what is expected
    // if true then modify the value of self.ch and seld.data_pos
    fn _is_next_ch(&mut self, ch: char) -> bool {
        if self.data_pos < self.data.len() && self.data[self.data_pos] == ch {
            self._update_ch();
            return true;
        }
        return false;
    }
}

// converstion
impl Lexer {
    #[inline(always)]
    fn data_to_string(&self, from: usize, to: usize) -> String {
        self.data[from..to].into_iter().collect()
    }

    fn data_to_int(&self, from: usize, to: usize) -> Result<i64, Exception> { 
        let val =  self.data_to_string(from, to);
        match val.parse::<i64>() {
            Ok(x) => Ok(x),
            Err(x) => Err(Exception::ValueError(format!(
                "invalid int literal \"{}\"",
                x
            ))),
        }
    }

    fn data_to_float(&self, from: usize, to: usize) -> Result<f64, Exception> {
        let val =  self.data_to_string(from, to);
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
    #[rustfmt::skip]
    fn tokenize_char(&mut self) -> Result<Token, Exception> {
        Ok(match self.ch {
            '\x00' => Token::Eof,
            ',' => Token::Comma,
            '[' => Token::LBrace,
            ']' => Token::RBrace,
            '{' => Token::LCurly,
            '}' => Token::RCurly,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ';' => Token::Semicolon,
            '+' => if self._is_next_ch('=') { Token::PlusEq } else { Token::Plus   },
            '-' => if self._is_next_ch('=') { Token::SubEq  } else { Token::Minus  },
            '*' => if self._is_next_ch('=') { Token::MulEq  } else { Token::Times  },
            '/' => if self._is_next_ch('=') { Token::DivEq  } else { Token::Divide },
            '%' => if self._is_next_ch('=') { Token::ModEq  } else { Token::Mod    },
            '=' => if self._is_next_ch('=') { Token::Eq     } else { Token::Assign },
            '!' => if self._is_next_ch('=') { Token::Ne     } else { Token::Not    },
            '^' => if self._is_next_ch('=') { Token::XorEq  } else { Token::Xor    },
            '&' => if self._is_next_ch('=') { Token::AndEq  } else if self._is_next_ch('&') { Token::LAnd   } else { Token::And },
            '|' => if self._is_next_ch('=') { Token::OrEq   } else if self._is_next_ch('|') { Token::LOr    } else { Token::Or  },
            '<' => if self._is_next_ch('=') { Token::Le     } else if self._is_next_ch('<') { Token::LShift } else { Token::Lt  },
            '>' => if self._is_next_ch('=') { Token::Ge     } else if self._is_next_ch('>') { Token::RShift } else { Token::Gt  },
            x => Token::Unknown(x),
        })
    }
    fn tokenize_number(&mut self) -> Result<Token, Exception> {
        let start_pos = self.data_pos - 1;
        let mut is_float = false;
        while self.is_number() {
            if self.ch == '.' {
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
        let val = self.data_to_string(start_pos, self.data_pos - 1);
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
        let val = self.data_to_string(start_pos + 1, self.data_pos - 1);
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
        while self.ch != '\n' && self.ch != '\r' && !self.is_eof() {
            self._update_ch();
        }
    }
}

// predicate function
impl Lexer {
    fn is_whitespace(&self) -> bool {
        self.ch == ' ' || self.ch == '\t' || self.ch == '\r' || self.ch == '\n'
    }

    fn is_number(&self) -> bool {
        '0' <= self.ch && self.ch <= '9' || self.ch == '.'
    }

    #[rustfmt::skip]
    fn is_hex(&self) -> bool {
        '0' <= self.ch || self.ch <= '9'
        || 'a' <= self.ch && self.ch <= 'f'
        || 'A' <= self.ch && self.ch <= 'X'
    }

    #[rustfmt::skip]
    fn is_identifier(&self) -> bool {
        'a' <= self.ch && self.ch <= 'z' || 
        'A' <= self.ch && self.ch <= 'Z' || 
        self.ch == '_'
    }

    fn is_string(&self) -> bool {
        self.ch == '"' || self.ch == '\''
    }
    fn is_comment(&self) -> bool {
        self.ch == '#'
    }
    fn is_eof(&self) -> bool {
        self.ch == '0'
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

        let mut lex = Lexer::new(data.to_string()).unwrap();

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

        let mut lex = Lexer::new(data.to_string()).unwrap();

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

        let mut lex = Lexer::new(data.to_string()).unwrap();

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
