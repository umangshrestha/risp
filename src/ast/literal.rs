#[derive(Debug, PartialEq, Clone)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl LiteralType {
    pub fn to_string(&self) -> String {
        match self {
            LiteralType::String(s) => s.clone(),
            LiteralType::Number(n) => n.to_string(),
            LiteralType::Boolean(b) => b.to_string(),
            LiteralType::Nil => "nil".to_string(),
        }
    }
}
