use crate::{visitor, ErrorInfo, Expr, Interpretor, LiteralType, Object, Span, TokenType, TokenInfo, Error};

impl visitor::Expr for Interpretor {
    fn visit_literal_expr(&mut self, value: &LiteralType) -> Result<Object, ErrorInfo> {
        Ok(match value {
            LiteralType::Nil => Object::Nil,
            LiteralType::Boolean(b) => Object::Boolean(*b),
            LiteralType::Number(n) => Object::Number(*n),
            LiteralType::String(s) => Object::String(s.clone()),
        })
    }

    fn visit_unary_expr(
        &mut self,
        op: &TokenInfo,
        right: &Box<Expr>,
    ) -> Result<Object, ErrorInfo> {
        self.eval(right)?
            .to_unary(&op.token)
            .map_err(|e| ErrorInfo::new_with_span(e, op.span.to_owned()))
    }

    fn visit_binary_expr(
        &mut self,
        left: &Box<Expr>,
        op: &TokenInfo,
        right: &Box<Expr>,
    ) -> Result<Object, ErrorInfo> {
        let left = self.eval(left)?;
        let right = self.eval(right)?;
        Object::binary(left, &op.token, right).map_err(|e| ErrorInfo::new_with_span(e, op.span.to_owned()))
    }


    fn visit_grouping_expr(&mut self, expr: &Box<Expr>, span: &Span) -> Result<Object, ErrorInfo> {
        self.eval(expr)
    }

    fn visit_assign_expr(
        &mut self,
        name: &String,
        value: &Box<Expr>,
        span: &Span,
    ) -> Result<Object, ErrorInfo> {
        let value = self.eval(value)?;
        self.environment
            .borrow_mut()
            .assign(name, value)
            .map_err(|e| ErrorInfo::new_with_span(e, span.to_owned()))
    }
    fn visit_call_expr(
        &mut self,
        callee: &Box<Expr>,
        args: &Vec<Expr>,
        span: &Span,
    ) -> Result<Object, ErrorInfo> {
        let callee = self.eval(callee)?;
        let mut arguments = Vec::new();
        for arg in args {
            arguments.push(self.eval(arg)?);
        }
        match callee {
            Object::Function(f) => f.call(self, &arguments),
            x => Err(ErrorInfo::new_with_span(
                Error::Type(format!("{x} is not callable")),
                span.to_owned(),
            )),
        }
    }

    fn visit_get_expr(
        &mut self,
        object: &Box<Expr>,
        name: &String,
        span: &Span,
    ) -> Result<Object, ErrorInfo> {
        let object = self.eval(object)?;
        let output = self
            .environment
            .borrow_mut()
            .get(name)
            .map_err(|e| ErrorInfo::new_with_span(e, span.to_owned()))?;
        Ok(output)
    }

    fn visit_set_expr(
        &mut self,
        object: &Box<Expr>,
        name: &String,
        value: &Box<Expr>,
        span: &Span,
    ) -> Result<Object, ErrorInfo> {
        todo!();
    }
    fn visit_super_expr(&mut self, name: &String, span: &Span) -> Result<Object, ErrorInfo> {
        todo!();
    }

    fn visit_variable_expr(&mut self, name: &String, span: &Span) -> Result<Object, ErrorInfo> {
        self.environment
            .borrow_mut()
            .get(name)
            .map_err(|e| ErrorInfo::new_with_span(e, span.to_owned()))
    }
}

#[cfg(test)]
mod test {
    use crate::interpretor::Interpretor;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::Object;

    #[test]
    fn test_literal() {
        let input = "nil; true; false; 123; \"Hello, world!\";";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let mut interpretor = Interpretor::new();
        interpretor.interpret(program);
    }

    #[test]
    fn test_constant() {
        let input = "
        const a = 1 + 2 * 3;  # declaring a variable
        const a = 100;        # redeclaring the const variable
        ";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let mut interpretor = Interpretor::new();
        interpretor.interpret(program);
        let output = interpretor
            .environment
            .borrow_mut()
            .get(&"a".to_string())
            .unwrap();
        assert_eq!(output, Object::Number(7.0));
    }
}
