use super::{Exception, Parser, Precedence, Token};
use crate::ast::{Identifier, Statement};

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement, Exception> {
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
        let name = self.get_identifier()?;
        self._update_token();
        self.should_be(Token::Assign)?;
        let statement = Statement::Let(name, self.parse_expression(Precedence::Lowest)?);
        self.should_be(Token::Semicolon)?;
        Ok(statement)
    }
    fn parse_return_statment(&mut self) -> Result<Statement, Exception> {
        self._update_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.should_be(Token::Semicolon)?;
        Ok(Statement::Return(expression))
    }

    pub fn parse_block_statement(&mut self) -> Result<Vec<Statement>, Exception> {
        self._update_token();
        self.should_be(Token::LBrace)?;
        let mut block = Vec::new();
        while !self.is_current_token(Token::RBrace) && !self.is_current_token(Token::Eof) {
            block.push(self.parse_statement()?);
            self._update_token();
        }
        Ok(block)
    }
}
