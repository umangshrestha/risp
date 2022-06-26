#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    IntLiteral(IntLiteral),
    FloatLiteral(FloatLiteral),
    BoolLiteral(BoolLiteral),
    StringLiteral(StringLiteral),
    IdentifierLiteral(IdentifierLiteral),
    // ListLiteral(Box<ListLiteral>),
    // DictLiteral(Box<DictLiteral>),
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::IntLiteral(x) => x.to_string(),
            Expression::FloatLiteral(x) => x.to_string(),
            Expression::BoolLiteral(x) => x.to_string(),
            Expression::StringLiteral(x) => x.to_string(),
            Expression::IdentifierLiteral(x) => x.to_string(),
            // ListLiterals(x) => x.to_string(),
            // DictLiteral(x) => x.to_string(),
        }
    }
}

// the function creates struct with generic format
// example:
// literal_generator!(IntLiteral, i64)
// will generate:
//
// #[derive(Hash, Eq, PartialEq, Clone, Debug)]
// pub struct IntLiteral {
//     val: i64
// }
//
// impl IntLiteral {
//    pub fn to_string(&self) -> String {
//       self.val.to_string()
//    }
// }
//
//  impl fmt::Display for $ident {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }
#[macro_export]
macro_rules! literal_generator {
    ($ident: ident, $ty: ty) => {
        #[derive(PartialEq, Clone, Debug)]
        pub struct $ident {
            pub val: $ty,
        }

        impl $ident {
            fn to_string(&self) -> String {
                self.val.to_string()
            }
        }

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }
    };
}

literal_generator!(IntLiteral, i64);
literal_generator!(FloatLiteral,f64);
literal_generator!(BoolLiteral, bool);
literal_generator!(StringLiteral, String);
literal_generator!(IdentifierLiteral, String);

#[derive(PartialEq, Clone, Debug)]
pub struct ListLiteral {
    val: Vec<Expression>,
}

impl ListLiteral {
    pub fn to_string(&mut self) -> String {
        let elem: Vec<String> = (&self.val).into_iter().map(|x| x.to_string()).collect();
        format!("[{}]", elem.join(","))
    }
}
