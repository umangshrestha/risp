
fn main() {
    let lexer = rlisp::Lexer::new("a = 1;".to_string());
    let mut parser = rlisp::Parser::new(lexer);
    print!("{:?}", parser.parse_program().unwrap());
}
