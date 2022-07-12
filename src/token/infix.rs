use super::Token;

const INFIX_OPERARTORS: [Token; 14] = [
    Token::Plus,
    Token::Minus,
    Token::Divide,
    Token::Times,
    Token::Mod,
    Token::Or,
    Token::And,
    Token::Xor,
    Token::Eq,
    Token::Ne,
    Token::Gte,
    Token::Gt,
    Token::Lt,
    Token::Lte,
];

impl Token {
    pub fn is_infix(&self) -> bool {
        INFIX_OPERARTORS.contains(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn test_is_infix() {
         // precedence level should be equal for the following token
         let is_infix = vec![
            Token::Plus,
            Token::Minus,
            Token::Divide,
            Token::Times,
            Token::Eq,
            Token::Ne,
            Token::Gte,
            Token::Gt,
            Token::Lt,
            Token::Lte,
        ];
        is_infix
        .iter()
        .for_each(|x| assert_eq!(x.is_infix(), true));  
    }

    #[test]
    fn test_not_infix() {
         // precedence level should be equal for the following token
         let is_infix = vec![
            Token::Int(1),
            Token::Float(1.0),
        ];
        is_infix
        .iter()
        .for_each(|x| assert_ne!(x.is_infix(), true));  
    }
}
