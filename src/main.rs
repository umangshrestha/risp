use std::{fs::File, io::Read};

use rlisp::{Interpretor, Lexer, Parser};

fn main() {
    let mut data = String::new();
    let mut f = File::open("./example/01_hello_world.example").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    let lexer = Lexer::new(data);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    let mut interpretor = Interpretor::new();
    interpretor.interpret(program);
}
