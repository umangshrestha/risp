use crate::Error;
use crate::Lexer;
use crate::{Expr, LiteralType, Stmt};
use crate::{TokenInfo, TokenType};

pub struct Parser {
    lexer: Lexer,
    prev: TokenInfo,
    curr: TokenInfo,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        Self {
            prev: TokenInfo::new(TokenType::Eof, 0, 0, 0),
            curr: lexer.next(),
            lexer,
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, Vec<Error>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();
        while !self.curr.is(TokenType::Eof) {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => errors.push(e),
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        match self.curr.token {
            TokenType::Let | TokenType::Const => self.let_declaration(),
            TokenType::Class => self.class_declaration(),
            TokenType::Function => {
                self.advance();
                self.function_declaration()
            }
            _ => self.statement(),
        }
    }

    fn let_declaration(&mut self) -> Result<Stmt, Error> {
        let is_const = self.curr.is(TokenType::Const);
        self.advance();
        let name = self.get_identifier()?;
        let mut value = None;
        if self.curr.is(TokenType::Assign) {
            self.advance();
            value = Some(self.expression()?);
        }
        self.should_be(TokenType::Semicolon)?;
        Ok(Stmt::Let {
            name,
            value,
            is_const,
        })
    }

    fn class_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.get_identifier()?;
        let super_class = if self.curr.is(TokenType::Lt) {
            self.advance();
            let super_class_name = self.get_identifier()?;
            if name == super_class_name {
                return Err(Error::Parse("Cannot inherit from itself".to_string()));
            }
            Some(super_class_name)
        } else {
            None
        };
        self.should_be(TokenType::LBrace)?;
        let mut methods = Vec::new();
        while !self.curr.is(TokenType::RBrace) && !self.curr.is(TokenType::Eof) {
            methods.push(self.function_declaration()?);
        }
        self.should_be(TokenType::RBrace)?;
        Ok(Stmt::Class {
            name,
            super_class,
            methods,
        })
    }

    fn function_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.get_identifier()?;
        self.should_be(TokenType::LParen)?;
        let mut params = Vec::new();
        if !self.curr.is(TokenType::RParen) {
            params.push(self.get_identifier()?);
            while self.curr.is(TokenType::Comma) {
                self.advance();
                params.push(self.get_identifier()?);
            }
        }
        self.should_be(TokenType::RParen)?;
        let body = self.block_statement()?;
        Ok(Stmt::Function {
            name,
            params: params,
            body: Box::new(body),
        })
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        match self.curr.token {
            TokenType::Print => self.print_statement(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::For => self.for_statement(),
            TokenType::Return => self.return_statement(),
            TokenType::LBrace => self.block_statement(),
            _ => self.expression_statement(),
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.should_be(TokenType::Semicolon)?;
        Ok(Stmt::Expr { expr })
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        self.advance();
        let expr = self.expression()?;
        self.should_be(TokenType::Semicolon)?;
        Ok(Stmt::Print { expr })
    }

    fn return_statement(&mut self) -> Result<Stmt, Error> {
        self.advance();
        let mut value = None;
        if !self.curr.is(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }
        self.should_be(TokenType::Semicolon)?;
        Ok(Stmt::Return { value })
    }

    fn for_statement(&mut self) -> Result<Stmt, Error> {
        self.advance();
        self.should_be(TokenType::LParen)?;
        let initializer = match self.curr.token {
            TokenType::Semicolon => None,
            TokenType::Let => Some(Box::new(self.let_declaration()?)),
            _ => Some(Box::new(self.expression_statement()?)),
        };

        let condition = match self.curr.token {
            TokenType::Semicolon => None,
            _ => Some(Box::new(self.expression()?)),
        };
        self.should_be(TokenType::Semicolon)?;

        let increment = match self.curr.token {
            TokenType::RParen => None,
            _ => Some(Box::new(self.expression()?)),
        };
        self.should_be(TokenType::RParen)?;

        let body = self.statement()?;

        Ok(Stmt::For {
            increment,
            condition,
            initializer: initializer,
            body: Box::new(body),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.advance();
        self.should_be(TokenType::LParen)?;
        let condition = self.expression()?;
        self.should_be(TokenType::RParen)?;
        let truthy = Box::new(self.statement()?);
        let mut falsy = None;
        if self.curr.is(TokenType::Else) {
            self.advance();
            falsy = Some(Box::new(self.statement()?));
        }
        Ok(Stmt::If {
            condition,
            truthy,
            falsy,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.advance();
        self.should_be(TokenType::LParen)?;
        let condition = self.expression()?;
        self.should_be(TokenType::RParen)?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn block_statement(&mut self) -> Result<Stmt, Error> {
        self.should_be(TokenType::LBrace)?;
        let mut stmt = Vec::new();
        while !self.curr.is(TokenType::RBrace) && !self.curr.is(TokenType::Eof) {
            stmt.push(self.declaration()?);
        }
        self.should_be(TokenType::RBrace)?;
        Ok(Stmt::Block { stmt })
    }
}

impl Parser {
    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let left = self.equality()?;
        if self.curr.is(TokenType::Assign) {
            self.advance();
            let right = self.assignment()?;
            return match left {
                Expr::Variable(name) => Ok(Expr::Assign {
                    name,
                    value: Box::new(right),
                }),
                Expr::Get { object, name } => Ok(Expr::Set {
                    object,
                    name,
                    value: Box::new(right),
                }),
                _ => return Err(Error::Parse("Invalid assignment target".to_string())),
            };
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut left = self.comparison()?;
        while let TokenType::Eq | TokenType::Ne = self.curr.token {
            let op = self.advance();
            let right = self.comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut left = self.term()?;
        while let TokenType::Gt | TokenType::Gte | TokenType::Lt | TokenType::Lte = self.curr.token
        {
            let op = self.advance();
            let right = self.term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut left = self.factor()?;
        while let TokenType::Plus | TokenType::Minus = self.curr.token {
            let op = self.advance();
            let right = self.factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut left = self.unary()?;
        while let TokenType::Times | TokenType::Divide = self.curr.token {
            let op = self.advance();
            let right = self.unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if let TokenType::Minus | TokenType::Not = self.curr.token {
            let op = self.advance();
            let right = self.unary()?;
            Ok(Expr::Unary {
                op,
                right: Box::new(right),
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;
        loop {
            match self.curr.token {
                TokenType::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if !self.curr.is(TokenType::RParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.curr.is(TokenType::Comma) {
                                break;
                            }
                            self.advance();
                        }
                    }
                }
                TokenType::Dot => {
                    self.advance();
                    let name = self.get_identifier()?;
                    expr = Expr::Get {
                        object: Box::new(expr),
                        name,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        match self.curr.token.clone() {
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal(LiteralType::Boolean(true)))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal(LiteralType::Boolean(false)))
            }
            TokenType::Number(x) => {
                self.advance();
                Ok(Expr::Literal(LiteralType::Number(x)))
            }
            TokenType::String(x) => {
                self.advance();
                Ok(Expr::Literal(LiteralType::String(x)))
            }
            TokenType::Identifier(x) => {
                self.advance();
                Ok(Expr::Variable(x))
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.should_be(TokenType::RParen)?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            TokenType::Super => {
                self.advance();
                self.should_be(TokenType::Dot)?;
                let name = self.get_identifier()?;
                Ok(Expr::Super { name })
            }
            TokenType::This => {
                self.advance();
                Ok(Expr::Variable("this".to_string()))
            }
            _ => {
                self.advance();
                Err(Error::Parse("Expect expression.".to_string()))
            }
        }
    }
}

impl Parser {
    fn should_be(&mut self, token_type: TokenType) -> Result<TokenType, Error> {
        if self.curr.is(token_type.clone()) {
            Ok(self.advance())
        } else {
            Err(Error::Syntax(format!(
                "Expected: \"{}\" Found: \"{}\"",
                token_type, self.curr.token
            )))
        }
    }

    fn get_identifier(&mut self) -> Result<String, Error> {
        if let TokenType::Identifier(x) = self.curr.token.clone() {
            self.advance();
            Ok(x)
        } else {
            Err(Error::Syntax(format!(
                "Expected: \"Identifier\" Found: \"{}\"",
                self.curr.token
            )))
        }
    }

    fn advance(&mut self) -> TokenType {
        self.prev = self.curr.clone();
        self.curr = self.lexer.next();
        self.prev.token.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_unary() {
        let input = "-(1 / (2 * 32));";
        let mut parser = Parser::new(Lexer::new(input.into()));
        let expr = parser.parse_program().unwrap();
        assert_eq!(
            expr,
            vec![Stmt::Expr {
                expr: Expr::Unary {
                    op: TokenType::Minus,
                    right: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                        left: Box::new(Expr::Literal(LiteralType::Number(1.0))),
                        op: TokenType::Divide,
                        right: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                            left: Box::new(Expr::Literal(LiteralType::Number(2.0))),
                            op: TokenType::Times,
                            right: Box::new(Expr::Literal(LiteralType::Number(32.0))),
                        }))),
                    }))),
                }
            }]
        );
    }

    #[test]
    fn test_assignment() {
        let input= "
        let a = 1;
        print a ;";
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let expr = parser.parse_program().unwrap();
        assert_eq!(
            expr,
            vec![Stmt::Let {
                name: "a".to_string(),
                value: Some(Expr::Literal(LiteralType::Number(1.0))),
                is_const: false,
            },
            Stmt::Print { expr: Expr::Variable("a".to_string()) }]
        );
    }
  
}
