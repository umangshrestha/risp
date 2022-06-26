use crate::{Exception, Lexer, Token};

mod ast;
use ast::*;

pub struct Parser {
    lexer: Lexer,
    token: Token,
    program: Vec<Expression>,
    errors: Vec<Exception>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer,
            token: Token::Eof,
            program: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) {
        while !self.is_token(Token::Eof) {
            match self.parse_statment() {
                Ok(x) => self.program.push(x),
                Err(x) => self.errors.push(x),
            }
        }
    }
}

impl Parser {
    fn parse_statment(&mut self) -> Result<Expression, Exception> {
        match &self.token {
            Token::True => Ok(Expression::BoolLiteral(ast::BoolLiteral { val: true })),
            Token::False => Ok(Expression::BoolLiteral(ast::BoolLiteral { val: false })),
            Token::Int(x) => Ok(Expression::IntLiteral(ast::IntLiteral { val: *x })),
            Token::Float(x) => Ok(Expression::FloatLiteral(ast::FloatLiteral { val: *x })),
            Token::String(x) => Ok(Expression::StringLiteral(ast::StringLiteral { val: x.to_string() })),
            Token::Identifier(x) => Ok(Expression::IdentifierLiteral(ast::IdentifierLiteral { val: x.to_string() })),
            x => Err(Exception::SyntaxError(format!("Unexpected Token: {}", self.token.to_string()))),
        }
    }
}
impl Parser {
    fn _update_token(&mut self) {
        match self.lexer.next_token() {
            Ok(token) => self.token = token,
            Err(err) => self.errors.push(err),
        }
    }
}

impl Parser {
    fn is_token(&self, token: Token) -> bool {
        self.token == token
    }
}
