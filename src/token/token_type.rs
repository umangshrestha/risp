use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Identifier(String), // variable
    String(String),     /* data types */
    Number(f64),
    True,
    False,
    Comma, /* Delimiter */
    Semicolon,
    Colon,
    Function, /* Keyword */
    Let,
    Const,
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
    Lte,
    Gte,
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
    Import,
    Class,
    Nil,
    This,
    Break,
    Continue,
    Print,
    Dot,
    Super
}

impl fmt::Display for TokenType {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;

        match self {
            Identifier(x) => write!(f, "${}", x),
            String(x)     => write!(f, "\"{}\"", x),
            Number(x)        => write!(f, "{}", x),
            True          => write!(f, "True"),
            False         => write!(f, "False"),
            Dot           => write!(f, "."),
            Comma         => write!(f, ","),
            Colon         => write!(f, ":"),
            Semicolon     => write!(f, ";"),
            LParen        => write!(f, "("),
            RParen        => write!(f, ")"),
            LBrace        => write!(f, "["),
            RBrace        => write!(f, "]"),
            LCurly        => write!(f, "{{"),
            RCurly        => write!(f, "}}"),
            Plus          => write!(f, "+"),
            Minus         => write!(f, "-"),
            Times         => write!(f, "*"),
            Divide        => write!(f, "/"),
            Assign        => write!(f, "="),
            Mod           => write!(f, "%"),
            PlusEq        => write!(f, "+="),
            SubEq         => write!(f, "-="),
            MulEq         => write!(f, "*="),
            DivEq         => write!(f, "/="),
            ModEq         => write!(f, "%="),
            Function      => write!(f, "fn"),
            Let           => write!(f, "let"),
            Const           => write!(f, "let"),
            If            => write!(f, "if"),
            Else          => write!(f, "else"),
            Return        => write!(f, "return"),
            For           => write!(f, "for"),
            While         => write!(f, "while"),
            Not           => write!(f, "!"),
            And           => write!(f, "&"),
            Or            => write!(f, "|"),
            Xor           => write!(f, "^"),
            AndEq         => write!(f, "&="),
            OrEq          => write!(f, "|="),
            LAnd          => write!(f, "&&"),
            LOr           => write!(f, "||"),
            XorEq         => write!(f, "^="),
            Gt            => write!(f, ">"),
            Lt            => write!(f, "<"),
            Eq            => write!(f, "=="),
            Ne            => write!(f, "!="),
            Lte           => write!(f, "=<"),
            Gte           => write!(f, ">="),
            LShift        => write!(f, "<<"),
            RShift        => write!(f, ">>"),
            Eof           => write!(f, "EOF"),
            Import        => write!(f, "import"),
            Class         => write!(f, "class"),
            Nil           => write!(f, "nil"),
            This          => write!(f, "this"),
            Break         => write!(f, "break"),
            Continue      => write!(f, "continue"),
            Print         => write!(f, "print"),
            Super         => write!(f, "super"),

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_literal() {
        let token = TokenType::Identifier("x".to_string());
        assert_eq!(token.to_string(), "$x");

        let token = TokenType::String("hello".to_string());
        assert_eq!(token.to_string(), "\"hello\"");

        let token = TokenType::Number(1.0);
        assert_eq!(token.to_string(), "1");

        let token = TokenType::True;
        assert_eq!(token.to_string(), "True");

        let token = TokenType::False;
        assert_eq!(token.to_string(), "False");
    }
}
