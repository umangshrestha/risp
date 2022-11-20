use crate::{visitor, ErrorInfo, Expr, Interpretor, LiteralType, Object, Span, TokenType};

impl visitor::Expr for Interpretor {
    fn visit_literal_expr(&mut self, value: &LiteralType, _: &Span) -> Result<Object, ErrorInfo> {
        Ok(match value {
            LiteralType::Nil => Object::Nil,
            LiteralType::Boolean(b) => Object::Boolean(*b),
            LiteralType::Number(n) => Object::Number(*n),
            LiteralType::String(s) => Object::String(s.clone()),
        })
    }

    fn visit_unary_expr(
        &mut self,
        op: &TokenType,
        right: &Box<Expr>,
        span: &Span,
    ) -> Result<Object, ErrorInfo> {
        self.eval(right)?
            .to_unary(op)
            .map_err(|e| ErrorInfo::new_with_span(e, span.to_owned()))
    }

    fn visit_binary_expr(
        &mut self,
        left: &Box<Expr>,
        op: &TokenType,
        right: &Box<Expr>,
        span: &Span,
    ) -> Result<Object, ErrorInfo> {
        let left = self.eval(left)?;
        let right = self.eval(right)?;
        Object::binary(left, op, right).map_err(|e| ErrorInfo::new_with_span(e, span.to_owned()))
    }

    fn visit_logical_expr(
        &mut self,
        left: &Box<Expr>,
        op: &TokenType,
        right: &Box<Expr>,
        span: &Span,
    ) -> Result<Object, ErrorInfo> {
        let left = self.eval(left)?;
        let right = self.eval(right)?;
        Object::logical(left, op, right).map_err(|e| ErrorInfo::new_with_span(e, span.to_owned()))
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
        todo!();
    }
    fn visit_call_expr(
        &mut self,
        callee: &Box<Expr>,
        paren: &TokenType,
        args: &Vec<Expr>,
        span: &Span,
    ) -> Result<Object, ErrorInfo> {
        todo!();
    }

    fn visit_get_expr(
        &mut self,
        object: &Box<Expr>,
        name: &String,
        span: &Span,
    ) -> Result<Object, ErrorInfo> {
        todo!();
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
        todo!();
    }
}



#[cfg(test)]
mod test {
    use crate::interpretor::Interpretor;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::visitor::Expr;

    #[test]
    fn test_literal() {
        let input = "nil; true; false; 123; \"Hello, world!\";";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let mut interpretor = Interpretor::new();
        interpretor.interpret(program);
    }
}