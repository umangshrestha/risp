
fn main() {
    let lexer = rlisp::Lexer::new("const a = 1;".to_string());
    let mut parser = rlisp::Parser::new(lexer);
    let program =parser.parse_program().unwrap();
    for stmt in program {
        println!("{}", stmt);
    } 
   
}
