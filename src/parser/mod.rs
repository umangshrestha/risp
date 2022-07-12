use crate::ast::{Expression, Identifier, Literal, Program, Statement};
use crate::{Exception, Lexer, Precedence, Token};

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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();
        while !self.is_current_token(Token::Eof) {
            match self.parse_statement() {
                Ok(x) => program.push(x),
                Err(x) => self.errors.push(x),
            }
            self._update_token();
        }
        return program;
    }
}

// statement
impl Parser {
    fn parse_statement(&self) -> Result<Statement, Exception> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statment(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Exception> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.should_be(Token::Semicolon)?;
        Ok(Statement::Expression(expression))
    }

    fn parse_let_statement(&mut self) -> Result<Statement, Exception> {
        match self.next_token {
            Token::Identifier(x) => {
                self._update_token();
                self.should_be(Token::Assign)?;
                let statement =
                    Statement::Let(Identifier(x), self.parse_expression(Precedence::Lowest)?);
                self.should_be(Token::Semicolon)?;
                Ok(statement)
            }
            token => {
                let error_msg = format!("Expected:Identifier Observed:{}", token);
                Err(Exception::SyntaxError(error_msg))
            }
        }
    }
    fn parse_return_statment(&mut self) -> Result<Statement, Exception> {
        let token = self.current_token;
        self._update_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.should_be(Token::Semicolon)?;
        Ok(Statement::Return(expression))
    }
}

// literal
impl Parser {
    fn parse_index_literal(&mut self) -> Result<Expression, Exception> {
        self._update_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        Ok(expression)
    }

    fn parse_hash_literal(&mut self) -> Result<Expression, Exception> {
        let mut val = Vec::new();
        while !self.is_next_token(Token::RCurly) {
            self._update_token();
            let key = self.parse_expression(Precedence::Lowest)?;
            self.should_be(Token::Colon)?;
            self._update_token();
            let value = self.parse_expression(Precedence::Lowest)?;
            val.push((key, value));
            if !self.is_next_token(Token::RCurly) && !self.is_next_token(Token::Comma) {
                return Err(Exception::SyntaxError(format!(
                    "Expected: '}}' or ',' found: '{}'",
                    self.next_token
                )));
            }
        }
        if !self.is_next_token(Token::RCurly) {
            Err(Exception::SyntaxError(format!(
                "Expected: '}}'  found: '{}'",
                self.next_token
            )))
        } else {
            Ok(Expression::HashMap(val))
        }
    }

    fn parse_function_literal(&mut self) -> Result<Expression, Exception> {
        if let Token::Identifier(name) = self.current_token {
            self.should_be(Token::LParen)?;
            let arguments = self.parse_function_paraments()?;
            self.should_be(Token::RParen)?;
            let body = self.parse_block_statement()?;
            Ok(Expression::Function {
                name: Identifier(name),
                arguments: arguments,
                body: body,
            })
        } else {
            Err(Exception::SyntaxError(format!(
                "expected: Identifier, found: {}",
                self.current_token
            )))
        }
    }

    fn parse_array_literal(&mut self) -> Result<Expression, Exception> {
        let expression = self.parse_expression_list(Token::RBrace)?;
        Ok(expression)
    }

    fn parse_block_statement(&mut self) -> Result<Program, Exception> {
        self._update_token();
        self.should_be(Token::LBrace)?;
        let mut block: Program = Vec::new();
        while !self.is_current_token(Token::RBrace) && !self.is_current_token(Token::Eof) {
            block.push(self.parse_statement()?);
            self._update_token();
        }
        Ok(block)
    }
}

// expression
impl Parser {
    fn parse_group_expression(&mut self) -> Result<Expression, Exception> {
        self._update_token();
        let exp = self.parse_expression(Precedence::Lowest)?;
        self.should_be(Token::RParen)?;
        Ok(exp)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, Exception> {
        self.should_be(Token::LParen)?;
        self._update_token();

        let condition = self.parse_expression(Precedence::Lowest)?;
        self.should_be(Token::RParen)?;
        self._update_token();

        let true_block = self.parse_block_statement()?;
        let false_block = None;
        if self.is_next_token(Token::Else) {
            self._update_token();
            let false_block = Some(self.parse_block_statement()?);
        }
        self.should_be(Token::RParen)?;
        let expression = Expression::If {
            condition: Box::new(condition),
            truth_block: true_block,
            false_block: false_block,
        };
        Ok(expression)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, Exception> {
        let mut lhs = self.parse_prefix_function()?;
        while !self.is_next_token(Token::Semicolon)
            && precedence < self.next_token.get_precedence()
            && self.next_token.is_infix()
        {
            self._update_token();
            lhs = self.parse_infix_expression(lhs)?;
        }
        Ok(lhs)
    }

    fn parse_infix_expression(&mut self, lhs: Expression) -> Result<Expression, Exception> {
        match self.current_token {
            Token::LParen => self.parse_call_expression(lhs),
            Token::LBrace => self.parse_index_expression(lhs),
            _ => self.parse_infix_operation(lhs),
        }
    }

    fn parse_infix_operation(&mut self, lhs: Expression) -> Result<Expression, Exception> {
        let op = self.current_token;
        let precedence = self.current_token.get_precedence();
        self._update_token();
        let rhs = self.parse_expression(precedence)?;
        Ok(Expression::Infix {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    fn parse_prefix_function(&mut self) -> Result<Expression, Exception> {
        match &self.current_token {
            Token::True | Token::False => self.parse_boolean_literal(),
            Token::Int(x) => self.parse_integer_literal(*x),
            Token::Float(x) => self.parse_float_literal(*x),
            Token::String(x) => self.parse_string_literal(x.to_string()),
            Token::Identifier(x) => self.parse_identifier_literal(x.to_string()),
            Token::Not | Token::Minus => self.parse_prefix_expression(),
            Token::LParen => self.parse_group_expression(),
            Token::If => self.parse_if_expression(),
            Token::LBrace => self.parse_array_literal(),
            Token::LCurly => self.parse_hash_literal(),
            Token::Function => self.parse_function_literal(),

            x => Err(Exception::SyntaxError(format!("Unknown Syntax:{}", x))),
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, Exception> {
        let op = self.current_token;
        self._update_token();
        let expression = self.parse_expression(Precedence::Prefix)?;
        Ok(Expression::Prefix {
            op,
            lhs: Box::new(expression),
        })
    }

    fn parse_boolean_literal(&mut self) -> Result<Expression, Exception> {
        Ok(Expression::Literal(Literal::Bool(
            self.is_current_token(Token::True),
        )))
    }

    fn parse_integer_literal(&mut self, x: i64) -> Result<Expression, Exception> {
        Ok(Expression::Literal(Literal::Int(x)))
    }

    fn parse_float_literal(&mut self, x: f64) -> Result<Expression, Exception> {
        Ok(Expression::Literal(Literal::Float(x)))
    }

    fn parse_string_literal(&mut self, x: String) -> Result<Expression, Exception> {
        Ok(Expression::Literal(Literal::String(x)))
    }
    fn parse_identifier_literal(&mut self, x: String) -> Result<Expression, Exception> {
        Ok(Expression::Identifier(Identifier(x.to_string())))
    }
}
impl Parser {
    fn _update_token(&mut self) {
        match self.lexer.next_token() {
            Ok(token) => {
                self.current_token = self.next_token;
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
        if self.is_next_token(token) {
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
