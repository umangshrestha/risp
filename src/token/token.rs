use std::fmt;

use super::Precedence;

// Tokens are the smallest, discreate and unique information that can be 
// used to give information about code
#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown(u8), // Unknown symbol
    Identifier(String), // variable 
    /* data types */
    String(String), 
    Int(i64),
    Float(f64),
    True,
    False,
    Comma,     /* Delimiter */
    Semicolon,
    Function,    /* Keyword */
    Let,
    Return,
    If,
    Else,
    LParen, /* brackets */
    RParen, 
    LBrace,
    RBrace,
    LCurly,
    RCurly,
    Assign,   /* operator */  
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Unknown(x)   => write!(f, "unknown {:?}", String::from_utf8_lossy(&[*x])),
            Token::Identifier(x)=> write!(f, "${{{}}}",x),
            Token::String(x)    => write!(f, "\"{}\"", x),
            Token::Int(x)       => write!(f, "\"{}\"", x),
            Token::Float(x)     => write!(f, "\"{}\"", x),
            Token::True         => write!(f, "True"),
            Token::False        => write!(f, "False"),
            Token::Comma        => write!(f, "),"),
            Token::Semicolon    => write!(f, ";"),
            Token::LParen       => write!(f, "("),
            Token::RParen       => write!(f, ")"),
            Token::LBrace       => write!(f, "["),
            Token::RBrace       => write!(f, "]"),
            Token::LCurly       => write!(f, "{{"),
            Token::RCurly       => write!(f, "}}"),
            Token::Plus         => write!(f, "+"),
            Token::Minus        => write!(f, "-"),
            Token::Times        => write!(f, "*"),
            Token::Divide       => write!(f, "/"),
            Token::Assign       => write!(f, "="),
            Token::Mod          => write!(f, "%"),
            Token::PlusEq       => write!(f, "+="),
            Token::SubEq        => write!(f, "-="),
            Token::MulEq        => write!(f, "*="),
            Token::DivEq        => write!(f, "/="),
            Token::ModEq        => write!(f, "%="),
            Token::Function     => write!(f, "fn"),
            Token::Let          => write!(f, "let"),
            Token::If           => write!(f, "if"),
            Token::Else         => write!(f, "else"),
            Token::Return       => write!(f, "return"),
            Token::Not          => write!(f, "!"),
            Token::And          => write!(f, "&"),
            Token::Or           => write!(f, "|"),
            Token::Xor          => write!(f, "^"),
            Token::AndEq        => write!(f, "&="),
            Token::OrEq         => write!(f, "|="),
            Token::LAnd         => write!(f, "&&"),
            Token::LOr          => write!(f, "||"),
            Token::XorEq        => write!(f, "^="),
            Token::Gt           => write!(f, ">"),
            Token::Lt           => write!(f, "<"),
            Token::Eq           => write!(f, "=="),
            Token::Ne           => write!(f, "!="),
            Token::Le           => write!(f, "=<"),
            Token::Ge           => write!(f, ">="),
            Token::LShift       => write!(f, "<<"),
            Token::RShift       => write!(f, ">>"),
            Token::Eof          => write!(f, "EOF"),
        }
    }
}



