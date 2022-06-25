use crate::{Lexer, Token, Exception, token};

mod literal;
pub use literal::Literal;

#[derive(Debug)]
pub enum Node {
    Literal(Box<Literal>),
    Program(Box<Program>),
    Statement(Box<Statement>),
    Expression(Box<Expression>)
}




// pub struct Parser {
//     lexer: Lexer,
//     token: Token,
//     errors: Vec<Exception>,
// }


// impl Parser {
//     pub fn new(lexer: Lexer) -> Parser {
//         Parser {
//             lexer: lexer,
//             token: Token::Eof,
//             errors: Vec::new()
//         }
//     }


//     pub fn parse(&mut self) {
//          self.lexer.next_token();
//     }

// }

// impl Token {
//     fn _update_token(&self)  {
//         match self.lexer.next_token() {
//             Ok(token) => self.token = token,
//             Err(err) => self.errors.push(),
//         }      
//     }

// }