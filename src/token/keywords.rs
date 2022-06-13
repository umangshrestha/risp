use super::Token;

pub fn lookup_identifiertype(ident: String) -> Token {
    // 1. the language is case sensitive
    // 2. all the keywords have full lowercase 
    match ident.as_str() {
        "true" => Token::True,
        "false" => Token::False,
        "fn" => Token::Function,
        "let" => Token::Let,
        "else" => Token::Else,
        "if" => Token::If,
        "for" => Token::For,
        "while" => Token::While,
        "return" => Token::Return,
        _ => Token::Identifier(ident),
    }
}

#[cfg(test)]
mod tests {
    use super::lookup_identifiertype;
    use super::Token;

    #[test]
    fn test_keywords() {
        // following string should be token
        let is_keyword = vec![
            // Lowest
            ("true", Token::True),
            ("false", Token::False),
            ("fn", Token::Function),
            ("let", Token::Let),
            ("if", Token::If),
            ("else", Token::Else),
            ("for", Token::For),
            ("while", Token::While),
            ("return", Token::Return),
        ];
        is_keyword.iter().for_each(|x| {
            let ident = x.0.to_string();
            assert_eq!(lookup_identifiertype(ident), x.1)
        });
    }

    fn test_identifier() {
        // following string should be identifiers
        let is_keyword = vec![
            "True", "TRUE", //
            "False", "FALSE", //
            "Fn", "fN", "FN", //
            "If", "iF", "IF", //
            "Let", "lEt", "leT", "lET", "LEt", "LeT", "LET", //
            "For", "fOr", "foR", "fOR", "FOr", "FoR", "FOR", //
        ];
        is_keyword
            .iter()
            .map(|x| (x.to_string(), Token::Identifier(x.to_string())))
            .for_each(|x| assert_eq!(lookup_identifiertype(x.0), x.1));
    }
}
