pub enum Token {
    Illegal(String), // Unknown symbol
    Identifier(String), // variable 
    /* data types */
    String(String), 
    Int(usize),
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
    And, /* logical operator */
    Or,
    Not,
    Xor,
    Lt,
    Gt,
    Eq,
    Ne,
    Lte,
    Gte,
    EOF,


    

}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Illegal(x)    => write!(f, "Illegal:\"{}\"", x),
            Token::Identifier(x) => write!(f, "${}", x),
            Token::String(x)     => write!(f, "\"{}\"", x),
            Token::Int(x)    => write!(f, "{}", x),
            Token::Float(x)      => write!(f, "{}", x),
            Token::True          => write!(f, "True"),
            Token::False         => write!(f, "False"),
            Token::Comma         => write!(f, ","),
            Token::Semicolon     => write!(f, ";"),
            Token::LParen        => write!(f, "("),
            Token::RParen        => write!(f, ")"),
            Token::LBrace        => write!(f, "["),
            Token::RBrace        => write!(f, "]"),
            Token::LCurly        => write!(f, "{{"),
            Token::RCurly        => write!(f, "}}"),
            Token::Plus          => write!(f, "+"),
            Token::Minus         => write!(f, "-"),
            Token::Times         => write!(f, "*"),
            Token::Divide        => write!(f, "/"),
            Token::Assign        => write!(f, "="),
            Token::Function      => write!(f, "fn"),
            Token::Let           => write!(f, "let"),
            Token::If            => write!(f, "if"),
            Token::Else          => write!(f, "else"),
            Token::Return        => write!(f, "return"),
            Token::Not           => write!(f, "!"),
            Token::And           => write!(f, "&"),
            Token::Or            => write!(f, "|"),
            Token::Xor           => write!(f, "^"),
            Token::Gt            => write!(f, ">"),
            Token::Lt            => write!(f, "<"),
            Token::Eq            => write!(f, "=="),
            Token::Ne            => write!(f, "!="),
            Token::Lte           => write!(f, "=<"),
            Token::Gte           => write!(f, ">="),
            Token::EOF           => write!(f, "EOF")
        }
    }
}