fn main() {
    let input = "
    if (a == 1) {
        print a;
    } else {
        print b;
    }";
    let lexer = rlisp::Lexer::new(input.to_string());
    let mut parser = rlisp::Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    println!("{}", program);
}
