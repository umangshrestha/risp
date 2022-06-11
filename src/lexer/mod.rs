use crate::{Token, Exception};
use std::fs;

pub struct Lexer {
    data        : Vec<u8>, // all the data in the file
    data_pos    : usize,   // index of the next postition to read for data
    ch          : u8,
    pub filename: String,
    pub line_no : usize,   // current line number
    pub line_pos: usize,   //  current position in line
    pub line    : Vec<u8>, // current line data  
}

// exported function    
impl Lexer {

    pub fn from_file(filename: &str) -> Result<Self, Exception> {
        match  fs::read(filename) {
            Err(_) => Err(Exception::FileNotFoundException(filename.to_string())),
            Ok(data) =>    Ok(Lexer{
                filename: filename.to_string(),
                data    : data,
                data_pos: 0,
                line_no : 1,
                line_pos: 0,
                ch      : 0,
                line    : Vec::new(),
            })
        }
        
    }

    pub fn next_token(&mut self) -> Token {
        match self.ch {
            b'\x00' => Token::Eof,
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
            b';' => self.remove_comment(),
            b' '| b'\t'| b'\n'| b'\r' => self.remove_whitespace(),

            x => Token::Unknown(x)
        }
    }

    pub fn read_line(&mut self) -> String {
        while self.data_pos < self.data.len() && self.data[self.data_pos] != b'\n' {
            self.line.push(self.data[self.data_pos]);
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
        }
        let val = self.data[self.data_pos];
        if val == b'\n' {
            self.line_no += 1;
            self.line_pos = 0;
            self.line = Vec::new();
        }
        self.data_pos += 1;
        self.line.push(val);
        self.ch = val;
    }

    // check if the next value for self.ch is what is expected
    // if true then modify the value of self.ch and self.pos 
    fn _is_next_ch(&mut self, ch: u8) -> bool {
        if self.data_pos < self.data.len() && self.data[self.data_pos] == ch { 
            self._update_ch();      
            return true;
        } 
        return  false;
    }
}


// tokenizer functions
impl Lexer {
    fn remove_whitespace(&mut self) -> Token {
        while self.is_whitespace() {
            self._update_ch();      
        }
        return self.next_token();
    }

    fn remove_comment(&mut self) -> Token {
        while self.ch != b'\n' {
            self._update_ch();      
        }
        return self.next_token();
    }
}
// predicate function
impl Lexer {
    fn is_whitespace(&self) -> bool {
        self.ch  == b' ' || self.ch == b'\t' ||
        self.ch == b'\r' || self.ch == b'\n'
    }
    
    fn is_number(&self) -> bool {
        b'0' <= self.ch   || self.ch <= b'9' 
    }
    
    fn is_hex(&self) -> bool {
        b'0' <= self.ch   || self.ch <= b'9' ||
        b'a' <= self.ch   || self.ch <= b'f' ||
        b'A' <= self.ch   || self.ch <= b'X'

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