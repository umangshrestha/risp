use rlisp::{Token, Lexer, Exception};


fn main() {
    let file_name = "example/hello_world.lisp";
    let mut lexer = Lexer::from_file(file_name).unwrap();
    
    loop {  
        let next_token = lexer.next_token();
        if next_token == Token::Eof {
            break;
        }
        println!("{}", next_token);
    }
}
