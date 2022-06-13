#[repr(u8)]
pub enum Precedence {
    Lowest,
    Lower,
    Low,
    Mid,
    High,
    Higher,
    Highest,
}
 
use crate::Token;
pub fn get_precedence(token: &Token) -> u8 {
    let p = match token {
        Token::Eq     => Precedence::Lower, // ==
        Token::Ne     => Precedence::Lower, // !=
        Token::Lt     => Precedence::Low,   // <=
        Token::Gt     => Precedence::Low,   // >=
        Token::Plus   => Precedence::Mid,   // +
        Token::Minus  => Precedence::Mid,   // -
        Token::Not    => Precedence::Mid,   // !
        Token::Times  => Precedence::High,  // *
        Token::Divide => Precedence::High,  // /
        Token::LParen => Precedence::Higher,// function ()
        Token::LBrace => Precedence::Highest, // index []
        _ => Precedence::Lowest,
    };
    return p as u8;
}

#[cfg(test)]
mod tests {
    use super::Token;
    use super::get_precedence;

    #[test]
    fn test_precedence_equal() {
        // precedence level should be equal for the following token
        let is_equal =  vec![
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
            (Token::Minus, Token::Minus),
            (Token::Plus, Token::Minus),
            // High
            (Token::Times, Token::Times),
            (Token::Divide, Token::Divide),
            (Token::Times, Token::Divide),
            // Higher
            (Token::LParen, Token::LParen),
            // Highest
            (Token::LBrace, Token::LBrace),
        ];
        is_equal.iter().for_each(|x| 
            assert_eq!(get_precedence(&x.0), get_precedence(&x.1)));
    }

    #[test]
    fn test_precedence_not_equal() {
        // precedence level should be equal for the following token
        let is_equal =  vec![
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
            (Token::Minus, Token::Divide),
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
        is_equal.iter().for_each(|x|  {
            let (a, b) = x;
            let a = get_precedence(a);
            let b =  get_precedence(b);
            assert_ne!(a, b);
            assert!(a < b)
        });

    }
}