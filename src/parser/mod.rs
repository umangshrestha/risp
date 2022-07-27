use crate::ast::Statement;
use crate::{Exception, Lexer, Precedence, Token};

mod expression;
mod literal;
mod statement;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
    errors: Vec<Exception>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer: lexer,
            current_token: Token::Eof,
            next_token: Token::Eof,
            errors: Vec::new(),
        };
        p._update_token();
        p._update_token();
        return p;
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut program = Vec::new();
        while !self.is_current_token(Token::Eof) {
            match self.parse_statement() {
                Ok(x) => program.push(x),
                Err(x) => self.errors.push(x),
            }
            self._update_token();
        }
        program
    }

    fn _update_token(&mut self) {
        match self.lexer.next_token() {
            Ok(token) => {
                self.current_token = self.next_token.clone();
                self.next_token = token;
            }
            Err(err) => self.errors.push(err),
        }
    }
}

// predicate
impl Parser {
    fn is_current_token(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn is_next_token(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn should_be(&mut self, token: Token) -> Result<(), Exception> {
        if self.is_next_token(token.clone()) {
            self._update_token();
            Ok(())
        } else {
            let error_msg = format!("Expected:{} Observed:{}", token, self.next_token);
            Err(Exception::SyntaxError(error_msg))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     // The syntax used in unittest may not represent the actual langauge system
//     // these are just to test the basic functionality of lexer
//     use super::Lexer;
//     use super::Parser;
//     use super::{ast, Expression};
//     #[test]
//     fn test_int_parser() {
//         let data = "1";

//         let mut lex = Lexer::new(data.to_string()).unwrap();
//         let mut parser = Parser::new(lex);
//         parser.parse_program();
//         let expected_ast = vec![Expression::Int(ast::IntLiteral { val: 1 })];

//         if expected_ast.len() != parser.program.len() {
//             panic!(
//                 "Expected:{:?} Observed:{:?}",
//                 expected_ast.len(),
//                 parser.program.len()
//             );
//         }

//         (1..expected_ast.len()).for_each(|i| {
//             let expected = &expected_ast[i];
//             let output = &parser.program[i];
//             if output != expected {
//                 panic!("Expected:{:?} Observed:{:?}", expected, output);
//             }
//         })
//     }
// }
