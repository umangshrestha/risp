use crate::{Error, Expr, Interpretor, LiteralType, Object, TokenType, Visitor, ErrorInfo};

impl Visitor::Expr for Interpretor {
    fn visit_literal_expr(&mut self, value: &LiteralType) -> Result<Object, ErrorInfo> {
        match value {
            LiteralType::Nil => Ok(Object::Nil),
            LiteralType::Boolean(b) => Ok(Object::Boolean(*b)),
            LiteralType::Number(n) => Ok(Object::Number(*n)),
            LiteralType::String(s) => Ok(Object::String(s.clone())),
        }
    }

    fn visit_unary_expr(&mut self, op: &TokenType, right: &Box<Expr>) -> Result<Object, ErrorInfo> {
        let right = self.eval(right)?;
        match op {
            TokenType::Minus => match right {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => Err(Error::Runtime("Operand must be a number.".to_string())),
            },
            TokenType::Plus => match right {
                Object::Number(n) => Ok(Object::Number(n)),
                _ => Err(Error::Runtime("Operand must be a number.".to_string())),
            },
            TokenType::Not => Ok(Object::Boolean(!&right.to_boolean())),
            _ => Err(Error::Runtime("Invalid unary operator.".to_string())),
        }
    }

    fn visit_binary_expr(
        &mut self,
        left: &Box<Expr>,
        op: &TokenType,
        right: &Box<Expr>,
    ) -> Result<Object, ErrorInfo> {
        let left = self.eval(left)?;
        let right = self.eval(right)?;

        match op {
            TokenType::Plus => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                (Object::String(l), Object::String(r)) => Ok(Object::String(l + &r)),
                _ => Err(Error::Runtime(
                    "Operands must be two numbers or two strings.".to_string(),
                )),
            },
            TokenType::Minus => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
                _ => Err(Error::Runtime("Operands must be two numbers.".to_string())),
            },
            TokenType::Times => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                (Object::String(l), Object::Number(r)) | (Object::Number(r), Object::String(l)) => {
                    Ok(Object::String(l.repeat(r as usize)))
                }
                (Object::Number(l), Object::Boolean(r))
                | (Object::Boolean(r), Object::Number(l)) => {
                    Ok(Object::Number(if r { l } else { 0.0 }))
                }
                _ => Err(Error::Runtime(
                    "Operands must be two numbers or a string and a number.".to_string(),
                )),
            },
            TokenType::Divide => match (left, right) {
                (Object::Number(l), Object::Number(r)) => {
                    if r == 0.0 {
                        Err(Error::ZeroDivision)
                    } else {
                        Ok(Object::Number(l / r))
                    }
                }
                (Object::Number(l), Object::Boolean(r))
                | (Object::Boolean(r), Object::Number(l)) => {
                    Ok(Object::Number(if r { l } else { 0.0 }))
                }
                _ => Err(Error::Runtime("Operands must be two numbers.".to_string())),
            },
            TokenType::And => match (left, right) {
                (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l && r)),
                _ => Err(Error::Runtime("Operands must be two booleans.".to_string())),
            },
            
            _ => Err(Error::Runtime("Invalid binary operator.".to_string())),
        }
    }

    fn visit_logical_expr(
        &mut self,
        left: &Box<Expr>,
        op: &TokenType,
        right: &Box<Expr>,
    ) -> Result<Object, ErrorInfo> {
        let left = self.eval(left)?;
        let right = self.eval(right)?;
        match op {
            TokenType::Gt => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l > r)),
                _ => Err(Error::Runtime("Operands must be two numbers.".to_string())),
            },
            TokenType::Gte => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l >= r)),
                _ => Err(Error::Runtime("Operands must be two numbers.".to_string())),
            },
            TokenType::Lt => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l < r)),
                _ => Err(Error::Runtime("Operands must be two numbers.".to_string())),
            },
            TokenType::Lte => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l <= r)),
                _ => Err(Error::Runtime("Operands must be two numbers.".to_string())),
            },
            TokenType::Eq => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l == r)),
                (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l == r)),
                (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l == r)),
                (Object::Nil, Object::Nil) => Ok(Object::Boolean(true)),
                _ => Ok(Object::Boolean(false)),
            },
            TokenType::Ne => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l != r)),
                (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l != r)),
                (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l != r)),
                (Object::Nil, Object::Nil) => Ok(Object::Boolean(false)),
                _ => Ok(Object::Boolean(true)),
            },
            _ => Err(Error::Runtime("Invalid Logical operator.".to_string())),
        }
    }

    
    fn visit_grouping_expr(&mut self, expr: &Box<Expr>) -> Result<Object, ErrorInfo> {
        self.eval(expr)
    }

    fn visit_assign_expr(&mut self, name: &String, value: &Box<Expr>) -> Result<Object, ErrorInfo> {

    }
    fn visit_call_expr(
        &mut self,
        callee: &Box<Expr>,
        paren: &TokenType,
        args: &Vec<Expr>,
    ) -> Result<Object, ErrorInfo>;
    fn visit_get_expr(&mut self, object: &Box<Expr>, name: &String) -> Result<Object, ErrorInfo>;

   
    fn visit_set_expr(
        &mut self,
        object: &Box<Expr>,
        name: &String,
        value: &Box<Expr>,
    ) -> Result<Object, ErrorInfo>;
    fn visit_super_expr(&mut self, name: &String) -> Result<Object, ErrorInfo>;

    fn visit_variable_expr(&mut self, name: &String) -> Result<Object, ErrorInfo>;
}
