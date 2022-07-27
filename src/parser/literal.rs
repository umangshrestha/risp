use super::{Exception, Parser, Precedence, Token};
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
                "expected: Identifier, found: {}",
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
        while self.is_next_token((Token::Comma)) {
            self._update_token();
            self._update_token();
            vec.push(self.get_identifier()?);
        }
        self.should_be(Token::RParen)?;
        Ok(vec)
    }
}
