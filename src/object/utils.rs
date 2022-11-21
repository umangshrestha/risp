use crate::{Error, Object, TokenType};

impl Object {
    pub fn is_nil(&self) -> bool {
        match self {
            Object::Nil => true,
            _ => false,
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            Object::Number(n) => *n != 0.0,
            Object::String(s) => !s.is_empty(),
        }
    }

    pub fn to_unary(&mut self, op: &TokenType) -> Result<Object, Error> {
        match op {
            TokenType::Minus | TokenType::Plus => {
                if let Object::Number(n) = self {
                    let val = *n;
                    Ok(Object::Number(if *op == TokenType::Plus {
                        val
                    } else {
                        -val
                    }))
                } else {
                    Err(Error::Runtime("Operand must be a number.".to_string()))
                }
            }
            TokenType::Not => Ok(Object::Boolean(!self.to_boolean())),
            _ => Err(Error::Runtime("Invalid unary operator.".to_string())),
        }
    }

    pub fn binary(left: Object, op: &TokenType, right: Object) -> Result<Object, Error> {
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
}
