use std::fmt;

// Tokens are the smallest, discreate and unique information that can be
// used to give information about code
#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown(char),        // Unknown symbol
    Identifier(String), // variable
    String(String),     /* data types */
    Int(i64),
    Float(f64),
    True,
    False,
    Comma, /* Delimiter */
    Semicolon,
    Function, /* Keyword */
    Let,
    Return,
    If,
    Else,
    For,
    While,
    LParen, /* brackets */
    RParen,
    LBrace,
    RBrace,
    LCurly,
    RCurly,
    Assign, /* operator */
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    LShift,
    RShift,
    And, /* logical operator */
    Or,
    Not,
    Xor,
    Lt,
    Gt,
    Eq,
    Ne,
    Le,
    Ge,
    Eof,
    PlusEq,
    DivEq,
    SubEq,
    MulEq,
    AndEq,
    OrEq,
    XorEq,
    ModEq,
    LAnd,
    LOr,
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Unknown(x)    => format!("unknown {:#?}",*x),
            Token::Identifier(x) => format!("${}", x),
            Token::String(x)     => format!("\"{}\"", x),
            Token::Int(x)        => x.to_string(),
            Token::Float(x)      => x.to_string(),
            Token::True          => "True".to_string(),
            Token::False         => "False".to_string(),
            Token::Comma         => ".to_string(),".to_string(),
            Token::Semicolon     => ";".to_string(),
            Token::LParen        => "(".to_string(),
            Token::RParen        => ")".to_string(),
            Token::LBrace        => "[".to_string(),
            Token::RBrace        => "]".to_string(),
            Token::LCurly        => "{{".to_string(),
            Token::RCurly        => "}}".to_string(),
            Token::Plus          => "+".to_string(),
            Token::Minus         => "-".to_string(),
            Token::Times         => "*".to_string(),
            Token::Divide        => "/".to_string(),
            Token::Assign        => "=".to_string(),
            Token::Mod           => "%".to_string(),
            Token::PlusEq        => "+=".to_string(),
            Token::SubEq         => "-=".to_string(),
            Token::MulEq         => "*=".to_string(),
            Token::DivEq         => "/=".to_string(),
            Token::ModEq         => "%=".to_string(),
            Token::Function      => "fn".to_string(),
            Token::Let           => "let".to_string(),
            Token::If            => "if".to_string(),
            Token::Else          => "else".to_string(),
            Token::Return        => "return".to_string(),
            Token::For           => "for".to_string(),
            Token::While         => "while".to_string(),
            Token::Not           => "!".to_string(),
            Token::And           => "&".to_string(),
            Token::Or            => "|".to_string(),
            Token::Xor           => "^".to_string(),
            Token::AndEq         => "&=".to_string(),
            Token::OrEq          => "|=".to_string(),
            Token::LAnd          => "&&".to_string(),
            Token::LOr           => "||".to_string(),
            Token::XorEq         => "^=".to_string(),
            Token::Gt            => ">".to_string(),
            Token::Lt            => "<".to_string(),
            Token::Eq            => "==".to_string(),
            Token::Ne            => "!=".to_string(),
            Token::Le            => "=<".to_string(),
            Token::Ge            => ">=".to_string(),
            Token::LShift        => "<<".to_string(),
            Token::RShift        => ">>".to_string(),
            Token::Eof           => "EOF".to_string(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
