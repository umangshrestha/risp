
use crate::TokenType;


pub fn lookup_identifier(ident: String) -> TokenType {
    // 1. the language is case sensitive
    // 2. all the keywords have full lowercase
    match ident.as_str() {
        "true" => TokenType::True,
        "false" => TokenType::False,
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "else" => TokenType::Else,
        "if" => TokenType::If,
        "for" => TokenType::For,
        "while" => TokenType::While,
        "return" => TokenType::Return,
        "import" => TokenType::Import,
        "nil" => TokenType::Nil,
        "class" => TokenType::Class,
        "this" => TokenType::This,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "super" => TokenType::Super,
        "print" => TokenType::Print,
        "const" => TokenType::Const,
        _ => TokenType::Identifier(ident),
    }
}

#[cfg(test)]
mod tests {
    use crate::{TokenType};
    use super::lookup_identifier;


    #[test]
    fn test_keywords() {
        // following string should be token
        let is_keyword = vec![
            // Lowest
            ("true", TokenType::True),
            ("false", TokenType::False),
            ("fn", TokenType::Function),
            ("let", TokenType::Let),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("for", TokenType::For),
            ("while", TokenType::While),
            ("return", TokenType::Return),
            ("class", TokenType::Class),
            ("this", TokenType::This),
            ("import", TokenType::Import),
            ("nil", TokenType::Nil),
            ("break", TokenType::Break),
            ("continue", TokenType::Continue),
            ("print", TokenType::Print),
            ("super", TokenType::Super),
            ("const", TokenType::Const),

        ];
        is_keyword.iter().for_each(|x| {
            let ident = x.0.to_string();
            assert_eq!(lookup_identifier(ident), x.1)
        });
    }

    #[test]
    fn test_identifier() {
        let is_keyword = vec![
            "True", "TRUE", 
            "False", "FALSE", 
            "Fn", "fN", "FN", 
            "If", "iF", "IF", 
            "Let", "lEt",
            "FOr", "FoR", "FOR",
             "wHILe", "wHILE", "WHile",
            "ReTURN", "RETURN", 
            "Class", "cLass", 
            "ThIS", "THIS", 
            "Import", "iMport",
             "Nil", "nIl", "niL", 
        ];
        is_keyword
            .iter()
            .map(|x| (x.to_string(), TokenType::Identifier(x.to_string())))
            .for_each(|x| assert_eq!(lookup_identifier(x.0), x.1));
    }
}


// 