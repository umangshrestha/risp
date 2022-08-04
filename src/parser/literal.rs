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
    use super::{Lexer, Parser, Exception};
    use crate::ast::{Expression, Literal, Statement};

    struct TestCase {input: String, program: Vec<Statement>, error: Vec<Exception>}
    #[test]
    fn test_int_boolean() {
        let testcases = vec![
            TestCase{
                input: "false;".to_string(), 
                program: vec![Statement::Expression(Expression::Literal(Literal::Bool(false)))],
                error: Vec::new()
            }, 
            TestCase{
                input:"true==true".to_string(), 
                program: Vec::new(),
                error: vec![Exception::SyntaxError("Expected:\";\" Observed:\"EOF\"".to_string())]
            },
            // ("1", Literal::Int(1)),
            // ("1000.0", Literal::Float(1000.0)),
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
            
            (1..testcase.error.len()).for_each(
                |i| {
                if testcase.error[i] != parser.error[i] {
                    panic!(
                        "(Error) Input:{} Expected:{:?} Observed:{:?}",
                        testcase.input,
                        testcase.error[i],
                        parser.error[i],
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
            
            (1..testcase.program.len()).for_each(
                |i| {
                if testcase.program[i] != parser.program[i] {
                    panic!(
                        "(Program) Input:{} Expected:{:?} Observed:{:?}",
                        testcase.input,
                        testcase.program[i],
                        parser.program[i],
                    );
                } 
            });
            
            

        });
    }

    // #[test]
    // fn test_int_parser() {
    //     let data = "1";
    //     let lex = Lexer::new(data.to_string()).unwrap();
    //     let mut parser = Parser::new(lex);
    //     let output_statements = parser.parse_program();
    //     let expected_statement = vec![Statement::Expression(Expression::Literal(Literal::Int(1)))];

    //     if expected_statement.len() != output_statements.len() {
    //         panic!(
    //             "Expected:{:?} Observed:{:?}",
    //             expected_statement.len(),
    //             output_statements.len()
    //         );
    //     }

    //     (1..output_statements.len()).for_each(|i| {
    //         if expected_statement[i] != output_statements[i] {
    //             panic!(
    //                 "Expected:{:?} Observed:{:?}",
    //                 expected_statement[i], output_statements[i]
    //             );
    //         }
    //     })
    // }
}
