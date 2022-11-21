use std::{fs::File, io::Read};
use std::{env, process};
use rlisp::{Interpretor, Lexer, Parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("[usuage] rlisp <file_name>");
        process::exit(0);
    }
    let file_name = &args[1];
    let mut data = String::new();
    let mut f = File::open(file_name).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    let lexer = Lexer::new(data);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    let mut interpretor = Interpretor::new();
    interpretor.interpret(program);
}
