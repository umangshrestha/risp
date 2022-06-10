use std::fmt;

// Tokens are the smallest, discreate and unique information that can be 
// used to give information about code
#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown, // Unknown symbol
    Identifier, // variable 
    /* data types */
    String, 
    Int,
    Float,
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
        write!(f, "{}", match self {
            Token::Unknown   =>  "unknown:",
            Token::Identifier=>  "$",
            Token::String    =>  "str:",
            Token::Int   =>  "int:",
            Token::Float    =>  "{}",
            Token::True         =>  "True",
            Token::False        =>  "False",
            Token::Comma        =>  ",",
            Token::Semicolon    =>  ";",
            Token::LParen       =>  "(",
            Token::RParen       =>  ")",
            Token::LBrace       =>  "[",
            Token::RBrace       =>  "]",
            Token::LCurly       =>  "{{",
            Token::RCurly       =>  "}}",
            Token::Plus         =>  "+",
            Token::Minus        =>  "-",
            Token::Times        =>  "*",
            Token::Divide       =>  "/",
            Token::Assign       =>  "=",
            Token::Mod          =>  "%",
            Token::PlusEq       =>  "+=",
            Token::SubEq        =>  "-=",
            Token::MulEq        =>  "*=",
            Token::DivEq        =>  "/=",
            Token::ModEq        =>  "%=",
            Token::Function     =>  "fn",
            Token::Let          =>  "let",
            Token::If           =>  "if",
            Token::Else         =>  "else",
            Token::Return       =>  "return",
            Token::Not          =>  "!",
            Token::And          =>  "&",
            Token::Or           =>  "|",
            Token::Xor          =>  "^",
            Token::AndEq        =>  "&=",
            Token::OrEq         =>  "|=",
            Token::LAnd         =>  "&&",
            Token::LOr          =>  "||",
            Token::XorEq        =>  "^=",
            Token::Gt           =>  ">",
            Token::Lt           =>  "<",
            Token::Eq           =>  "==",
            Token::Ne           =>  "!=",
            Token::Le           =>  "=<",
            Token::Ge           =>  ">=",
            Token::LShift       =>  "<<",
            Token::RShift       =>  ">>",
            Token::Eof          =>  "EOF",
        })
    }
}