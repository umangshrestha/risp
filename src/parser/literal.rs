use super::{Exception, Lexer, Parser, Precedence, Token};
use crate::ast::{Expression, Identifier, Literal};

//  literal
impl Parser {
    fn parse_index_literal(&mut self) -> Result<Expression, Exception> {
        self._update_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        Ok(expression)
    }

    pub fn parse_hash_literal(&mut self) -> Result<Expression, Exception> {
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
                    "Expected:\"}}\" or \",\" found:\"{}\"",
                    self.next_token
                )));
            }
        }
        if !self.is_next_token(Token::RCurly) {
            Err(Exception::SyntaxError(format!(
                "Expected:\"}}\"  found:\"{}\"",
                self.next_token
            )))
        } else {
            Ok(Expression::HashMap(val))
        }
    }

    pub fn parse_function_literal(&mut self) -> Result<Expression, Exception> {
        let name = self.get_identifier()?;
        let arguments = self.parse_function_paraments()?;
        let body = self.parse_block_statement()?;
        Ok(Expression::Function {
            name: name,
            arguments: arguments,
            body: body,
        })
    }

    pub fn parse_array_literal(&mut self) -> Result<Expression, Exception> {
        let expression = self.parse_expression_list(Token::RBrace)?;
        Ok(Expression::Array(expression))
    }

    pub fn parse_boolean_literal(&mut self) -> Result<Expression, Exception> {
        Ok(Expression::Literal(Literal::Bool(
            self.is_current_token(Token::True),
        )))
    }

    pub fn parse_integer_literal(&mut self, x: i64) -> Result<Expression, Exception> {
        Ok(Expression::Literal(Literal::Int(x)))
    }

    pub fn parse_float_literal(&mut self, x: f64) -> Result<Expression, Exception> {
        Ok(Expression::Literal(Literal::Float(x)))
    }

    pub fn parse_string_literal(&mut self, x: String) -> Result<Expression, Exception> {
        Ok(Expression::Literal(Literal::String(x)))
    }
    pub fn parse_identifier_literal(&mut self, x: String) -> Result<Expression, Exception> {
        Ok(Expression::Identifier(Identifier(x.to_string())))
    }

    pub fn get_identifier(&mut self) -> Result<Identifier, Exception> {
        if let Token::String(x) = self.current_token.clone() {
            Ok(Identifier(x))
        } else {
            Err(Exception::SyntaxError(format!(
                "expected:\"Identifier\", found:\"{}\"",
                self.current_token,
            )))
        }
    }

    fn parse_function_paraments(&mut self) -> Result<Vec<Identifier>, Exception> {
        self.should_be(Token::LParen)?;
        let mut vec = Vec::new();
        if self.is_next_token(Token::RParen) {
            self._update_token();
            return Ok(vec);
        }
        self._update_token();
        vec.push(self.get_identifier()?);
        while self.is_next_token(Token::Comma) {
            self._update_token();
            self._update_token();
            vec.push(self.get_identifier()?);
        }
        self.should_be(Token::RParen)?;
        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    // The syntax used in unittest may not represent the actual langauge system
    // these are just to test the basic functionality of lexer
    use super::{Exception, Lexer, Parser, Token};
    use crate::ast::{Expression, Literal, Statement, Identifier};

    struct TestCase {
        input: String,
        program: Vec<Statement>,
        error: Vec<Exception>,
    }
    #[test]
    fn test_int_boolean() {
        let testcases = vec![
            TestCase {
                input: "(false!=true);".to_string(),
                program: vec![Statement::Expression(Expression::Infix {
                    op: Token::Ne,
                    lhs: Box::new(Expression::Literal(Literal::Bool(false))),
                    rhs: Box::new(Expression::Literal(Literal::Bool(true))),
                })],
                error: Vec::new(),
            },
            TestCase {
                input: "true==true".to_string(),
                program: Vec::new(),
                error: vec![Exception::SyntaxError(
                    "Expected:\";\" Observed:\"EOF\"".to_string(),
                )],
            },
            TestCase {
                input: "-(1-10)*100+2/(2+3);".to_string(),
                program: vec![
                    Statement::Expression(
                        Expression::Infix { // -(1-10)*100 + 100 + 2 /(2+3)
                            op: Token::Plus, 
                            lhs: Box::new(Expression::Infix { // -(1-10)*100
                                op: Token::Times, 
                                lhs: Box::new(Expression::Prefix { // -(1-10)
                                    op: Token::Minus, 
                                    lhs: Box::new(Expression::Infix { // 1-10
                                        op: Token::Minus, 
                                        lhs: Box::new(Expression::Literal(Literal::Int(1))), 
                                        rhs: Box::new(Expression::Literal(Literal::Int(10))), 
                                    }) 
                                }), 
                                rhs: Box::new(Expression::Literal(Literal::Int(100))) // 100 
                            }), 
                            rhs: Box::new(Expression::Infix { 
                                op: Token::Divide, 
                                lhs: Box::new(Expression::Literal(Literal::Int(2))), // 2/(2+3 )
                                rhs: Box::new(Expression::Infix { // 2 + 3
                                    op: Token::Plus, 
                                    lhs: Box::new(Expression::Literal(Literal::Int(2))), 
                                    rhs: Box::new(Expression::Literal(Literal::Int(3))), 
                                }) 
                            }) 
                        })],
                        error: Vec::new(),
            },
            TestCase{
                input: "a(1);".to_string(),
                program: vec![
                    Statement::Expression(
                        Expression::Call {
                            function: Box::new(Expression::Identifier(Identifier("a".to_string()))),
                            arguments: vec![Expression::Literal(Literal::Int(1))],
                        }
                )],
                error: Vec::new(),
            },
            TestCase{
                input: "0.1000;".to_string(),
                program: vec![Statement::Expression(Expression::Literal(Literal::Float(0.1)))],
                error: Vec::new(),
            },
        ];
        testcases.iter().for_each(|testcase| {
            let lex = Lexer::new(testcase.input.to_string()).unwrap();
            let mut parser = Parser::new(lex);
            parser.parse_program();
            // testing for error
            if testcase.error.len() != parser.error.len() {
                panic!(
                    "(Error) Input:{} Expected:{:?} Observed:{:?}",
                    testcase.input,
                    testcase.error.len(),
                    parser.error.len(),
                );
            }

            parser.error.iter().enumerate().for_each(|(i, error)| {
                if testcase.error[i] != *error {
                    panic!(
                        "(Error) Input:{} Expected:{:?} Observed:{:?}",
                        testcase.input, testcase.error[i], error,
                    );
                }
            });

            if testcase.program.len() != parser.program.len() {
                panic!(
                    "(Program) Input:{} Expected:{:?} Observed:{:?}",
                    testcase.input,
                    testcase.program.len(),
                    parser.program.len()
                );
            }

            parser.program.iter().enumerate().for_each(|(i, program)| {
                if testcase.program[i] != *program {
                    panic!(
                        "(Program) Input:{} Expected:{:?} Observed:{:?}",
                        testcase.input, testcase.program[i], program,
                    );
                }
            });
        });
    }
}
