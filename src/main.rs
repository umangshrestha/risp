fn main() {
    let input = "
    \"Hello, world!\";";
    let lexer = rlisp::Lexer::new(input.to_string());
    let mut parser = rlisp::Parser::new(lexer);
    let program = parser.parse_program();
    if program.is_ok() {
        println!("{}", program.unwrap().stmts[0]);
    } else {
        program.err().unwrap().report()
    }
}
