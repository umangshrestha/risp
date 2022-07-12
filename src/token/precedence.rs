use super::Token;

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Eq,
    Lte,
    Plus,
    Product,
    Prefix,
    Call,
    Index,
}

impl Token {
    pub fn get_precedence(&self) -> Precedence {
        let p = match self {
            Token::Eq => Precedence::Eq,          // ==
            Token::Ne => Precedence::Eq,          // !=
            Token::Lt => Precedence::Lte,         // <=
            Token::Gt => Precedence::Lte,         // >=
            Token::Plus => Precedence::Plus,      // +
            Token::Times => Precedence::Product,  // *
            Token::Divide => Precedence::Product, // /
            Token::Not => Precedence::Prefix,     // !
            Token::Minus => Precedence::Prefix,   // -
            Token::LParen => Precedence::Call,    // function ()
            Token::LBrace => Precedence::Index,   // index []
            _ => Precedence::Lowest,
        };
        return p;
    }
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn test_precedence_equal() {
        // precedence level should be equal for the following token
        let is_equal = vec![
            // Lowest
            (Token::Mod, Token::RShift),
            (Token::LShift, Token::RShift),
            // Lower
            (Token::Eq, Token::Eq),
            (Token::Ne, Token::Ne),
            (Token::Eq, Token::Ne),
            // Low
            (Token::Lt, Token::Lt),
            (Token::Gt, Token::Gt),
            (Token::Gt, Token::Lt),
            // Mid
            (Token::Plus, Token::Plus),
            // High
            (Token::Times, Token::Times),
            (Token::Divide, Token::Divide),
            (Token::Times, Token::Divide),
            // Higher
            (Token::LParen, Token::LParen),
            // Highest
            (Token::LBrace, Token::LBrace),
        ];
        is_equal
            .iter()
            .for_each(|x| assert_eq!(x.0.get_precedence(), x.1.get_precedence()));
    }

    #[test]
    fn test_precedence_not_equal() {
        // precedence level on left should be less than right for the following token
        let is_greter = vec![
            // Lowest to Lower
            (Token::Mod, Token::Eq),
            (Token::LShift, Token::Ne),
            // Lower to Low
            (Token::Eq, Token::Lt),
            (Token::Ne, Token::Gt),
            // lower to mid
            (Token::Eq, Token::Plus),
            (Token::Ne, Token::Minus),
            // Low to mid
            (Token::Lt, Token::Plus),
            (Token::Gt, Token::Minus),
            (Token::Gt, Token::Not),
            // mid to higher
            (Token::Lt, Token::LParen),
            // mid to highest
            (Token::Gt, Token::LBrace),
            // Mid to high
            (Token::Plus, Token::Times),
            // mid to higher
            (Token::Plus, Token::LParen),
            // mid to highest
            (Token::Not, Token::LBrace),
            // High to Higher
            (Token::Times, Token::LParen),
            (Token::Divide, Token::LParen),
            (Token::Times, Token::LParen),
            // Higher to highest
            (Token::LParen, Token::LBrace),
        ];
        is_greter.iter().for_each(|x| {
            let (a, b) = x;
            let a = a.get_precedence();
            let b = b.get_precedence();
            assert_ne!(a, b);
            assert!(a < b)
        });
    }
}
