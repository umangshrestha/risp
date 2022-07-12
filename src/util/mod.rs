// the job of this macro is to implement the formatter
// for a given struct/ enum
// the macro works on the assumption that to_string() function is implemented
#[macro_export]
macro_rules! add_fmt_print {
    ($ident: ident) => {
        use std::fmt;
1usize
        impl fmt::Display for $ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }
    };
}
