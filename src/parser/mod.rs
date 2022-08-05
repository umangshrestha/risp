use crate::ast::Statement;
use crate::{Exception, Lexer, Precedence, Token, Traceback};

mod expression;
mod literal;
mod statement;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
    //output
    pub program: Vec<Statement>,
    pub error: Vec<Traceback>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer: lexer,
            current_token: Token::Eof,
            next_token: Token::Eof,
            program: Vec::new(),
            error: Vec::new(),
        };
        p._update_token();
        p._update_token();
        return p;
    }

    pub fn parse_program(&mut self) {
        while !self.is_current_token(Token::Eof) {
            match self.parse_statement() {
                Ok(x) => self.program.push(x),
                Err(x) => self.error.push(self.lexer.get_traceback(x)),
            }
            self._update_token();
        }
    }

    fn _update_token(&mut self) {
        match self.lexer.next_token() {
            Ok(token) => {
                self.current_token = self.next_token.clone();
                self.next_token = token;
            }
            Err(err) => self.error.push(self.lexer.get_traceback(err)),
        }
    }
}

// predicate
impl Parser {
    fn is_current_token(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn is_next_token(&self, token: Token) -> bool {
        self.next_token == token
    }

    fn should_be(&mut self, token: Token) -> Result<(), Exception> {
        if self.is_next_token(token.clone()) {
            self._update_token();
            Ok(())
        } else {
            Err(Exception::SyntaxError(format!(
                "Expected:\"{}\" Observed:\"{}\"",
                token, self.next_token
            )))
        }
    }
}
