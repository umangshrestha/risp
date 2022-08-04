use super::{Exception, Parser, Precedence, Token};
use crate::ast::{Expression, Identifier};

// expression
impl Parser {
    pub fn parse_expression_list(
        &mut self,
        end_token: Token,
    ) -> Result<Vec<Expression>, Exception> {
        let mut vec = Vec::new();
        if self.is_next_token(end_token.clone()) {
            self._update_token();
            return Ok(vec);
        }
        self._update_token();
        vec.push(self.parse_expression(Precedence::Lowest)?);
        while self.is_next_token(Token::Comma) {
            self._update_token();
            self._update_token();
            vec.push(self.parse_expression(Precedence::Lowest)?);
        }
        self.should_be(end_token)?;
        Ok(vec)
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, Exception> {
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
        let mut false_block = None;
        if self.is_next_token(Token::Else) {
            self._update_token();
            false_block = Some(self.parse_block_statement()?);
        }
        self.should_be(Token::RParen)?;
        let expression = Expression::If {
            condition: Box::new(condition),
            truth_block: true_block,
            false_block: false_block,
        };
        Ok(expression)
    }

    fn parse_infix_expression(&mut self, lhs: Expression) -> Result<Expression, Exception> {
        match self.current_token {
            Token::LParen => self.parse_call_expression(lhs),
            Token::LBrace => self.parse_index_expression(lhs),
            _ => self.parse_infix_operation(lhs),
        }
    }

    fn parse_infix_operation(&mut self, lhs: Expression) -> Result<Expression, Exception> {
        let op = self.current_token.clone();
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
        match &self.current_token.clone() {
            Token::True | Token::False => self.parse_boolean_literal(),
            Token::Int(x) => self.parse_integer_literal(x.clone()),
            Token::Float(x) => self.parse_float_literal(x.clone()),
            Token::String(x) => self.parse_string_literal(x.to_string()),
            Token::Identifier(x) => self.parse_identifier_literal(x.to_string()),
            Token::Not | Token::Minus => self.parse_prefix_expression(),
            Token::LParen => self.parse_group_expression(),
            Token::If => self.parse_if_expression(),
            Token::LBrace => self.parse_array_literal(),
            Token::LCurly => self.parse_hash_literal(),
            Token::Function => self.parse_function_literal(),

            x => Err(Exception::SyntaxError(format!("Unknown Syntax:\"{}\"", x))),
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, Exception> {
        let op = self.current_token.clone();
        self._update_token();
        let expression = self.parse_expression(Precedence::Prefix)?;
        Ok(Expression::Prefix {
            op,
            lhs: Box::new(expression),
        })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, Exception> {
        Ok(Expression::Call {
            function: Box::new(function),
            arguments: self.parse_expression_list(Token::RParen)?,
        })
    }

    fn parse_index_expression(&mut self, array: Expression) -> Result<Expression,Exception> {
        self._update_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        let expression = Expression::Index{
            array: Box::new(array),
            index: Box::new(index),
        };
        self.should_be(Token::RBrace)?;
        Ok(expression)
    }
}
