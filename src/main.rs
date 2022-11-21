use rlisp::Interpretor;

fn main() {
    let input = "
    print(\"Hello, world!\"*3);";
    let lexer = rlisp::Lexer::new(input.to_string());
    let mut parser = rlisp::Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    let mut interpretor = Interpretor::new();
    interpretor.interpret(program);
}
