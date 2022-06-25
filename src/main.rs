use rlisp::{Token, Lexer, Exception, Parser};


fn main() {
    let file_name = "example/hello_world.lisp";
    let data = "println(\"Hello, World\")";

    let mut lex = Lexer::new(data).unwrap();
    let mut parse = Parser::new(lex);
    parse.parse()

}
