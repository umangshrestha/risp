use std::fmt;

#[derive(Hash, Eq, Clone, Debug, PartialEq)]
pub enum Literal {
    Int(IntLiteral),
    Float(FloatLiteral),
    Bool(BoolLiteral),
    String(StringLiteral),
    List(Box<ListLiteral>),
    Dict(Box<DictLiteral>),
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::Int(x) => x.to_string(),
            Literal::Float(x) => x.to_string(),
            Literal::Bool(x) => x.to_string(),
            Literal::String(x) => x.to_string(),
            Literal::List(x) => x.to_string(),
            Literal::Dict(x) => x.to_string(),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct IntLiteral {
    val: i64,
}

impl IntLiteral {
    pub fn new(val: i64) -> Self {
        Self { val }
    }

    pub fn to_string(&self) -> String {
        self.val.to_string()
    }
}

impl fmt::Display for IntLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FloatLiteral {
    val: f64,
}

impl FloatLiteral {
    pub fn new(val: f64) -> Self {
        Self { val }
    }

    pub fn to_string(&self) -> String {
        self.val.to_string()
    }
}

impl fmt::Display for FloatLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct BoolLiteral {
    val: bool,
}

impl BoolLiteral {
    pub fn new(val: bool) -> Self {
        Self { val }
    }

    pub fn to_string(&self) -> String {
        self.val.to_string()
    }
}

impl fmt::Display for BoolLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct StringLiteral {
    val: String,
}

impl StringLiteral {
    pub fn new(val: String) -> Self {
        Self { val }
    }

    pub fn to_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct ArrayLiteral {
    val: Vec<Literal>,
}

impl ArrayLiteral {
    pub fn new(val:  Vec<Literal>) -> Self {
        Self { val }
    }

    pub fn to_string(&mut self) -> String {
        let elem: Vec<String> = (&self.val).into_iter()
            .map(|x| x.to_string())
            .collect();
        elem.join(",")
    }
}

impl fmt::Display for ArrayLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]",self.to_string())
    }
}
