// lexer takes the source code as bytearray and returns tokens as output
// this stage doesn't check if there is error or not
use crate::Token;
use std::fs;
use std::io::Error;

pub struct Lexer {
    data    : Vec<u8>, // In rust we cannot get the index of string without makeing it iterable 
                // hence using list of bytes for for lexing stage
    pub pos : usize, // next position that will be read
    pub ch  : u8,
    pub line:Vec<u8>, // stores the whole line for debug position
}

impl Lexer {
    pub fn from_file(file_name: &str) -> Result<Self, Error> {
        let data = fs::read(file_name)?;
        let lexer  = Lexer{
            data : data, 
            pos  : 0, 
            ch   : 0, 
            line : Vec::new(),
        };
        Ok(lexer)
    }


    pub fn next_token(&mut self) -> Token {
        self._update_ch();
        match self.ch {
            b'\x00' => Token::Eof,
            b';' => Token::Semicolon,
            b',' => Token::Comma,
            b'[' => Token::LBrace,
            b']' => Token::RBrace,
            b'{' => Token::LCurly,
            b'}' => Token::RCurly,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'+' => if self._is_next_ch(b'=') { Token::PlusEq } else { Token::Plus   },
            b'-' => if self._is_next_ch(b'=') { Token::SubEq  } else { Token::Minus  },
            b'*' => if self._is_next_ch(b'=') { Token::MulEq  } else { Token::Times  },
            b'/' => if self._is_next_ch(b'=') { Token::DivEq  } else { Token::Divide },
            b'%' => if self._is_next_ch(b'=') { Token::ModEq  } else { Token::Mod    }, 
            b'=' => if self._is_next_ch(b'=') { Token::Eq     } else { Token::Assign },
            b'!' => if self._is_next_ch(b'=') { Token::Ne     } else { Token::Not    },
            b'^' => if self._is_next_ch(b'=') { Token::XorEq  } else { Token::Xor    },
            b'&' => if self._is_next_ch(b'=') { Token::AndEq  } else if self._is_next_ch(b'&') { Token::LAnd   } else { Token::And },
            b'|' => if self._is_next_ch(b'=') { Token::OrEq   } else if self._is_next_ch(b'|') { Token::LOr    } else { Token::Or  },
            b'<' => if self._is_next_ch(b'=') { Token::Le     } else if self._is_next_ch(b'<') { Token::LShift } else { Token::Lt  },
            b'>' => if self._is_next_ch(b'=') { Token::Ge     } else if self._is_next_ch(b'>') { Token::RShift } else { Token::Gt  },
            _ => Token::Unknown,
        }
    }
}



impl Lexer {
    // reads the next postion in self.data and stores it in self.ch
    // stores 0 if data is not found
    fn _update_ch(&mut self) { 
         self.ch = if self.pos >= self.data.len() {
            b'\x00'
        } else {
            let val = self.data[self.pos];
            self.pos += 1;
            val
        }
    }

    // check if the next value for self.ch is what is expected
    // if true then modify the value of self.ch and self.pos 
    fn _is_next_ch(&mut self, ch: u8) -> bool {
        if self.pos < self.data.len() && self.data[self.pos] == ch { 
            self._update_ch();      
            return true;
        } 

        return  false;
    }
}


impl Lexer {
    fn is_whitespace(&self) -> bool {
        self.ch  == b' ' || self.ch == b'\t' ||
        self.ch == b'\r' || self.ch == b'\n'
    }
    
    fn is_number(&self) -> bool {
        b'0' <= self.ch   || self.ch <= b'9' 
    }
    
    fn is_alphabet(&self) -> bool { 
        b'a' <= self.ch   || self.ch <= b'z' ||
        b'A' <= self.ch   || self.ch <= b'Z' ||
        self.ch == b'_' 
    }
    
    fn is_comment(&self) -> bool {
        self.ch == b';'
    }
    
}