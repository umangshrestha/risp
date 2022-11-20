fn main() {
    let lexer = rlisp::Lexer::new("-(1 / (2 * 32));".to_string());
    let mut parser = rlisp::Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    println!("{}", program);
}
