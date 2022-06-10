use rlisp::{Token, Lexer};


fn main(){
    let file_name = "example/hello_world.lisp";
    let mut lexer = Lexer::from_file(file_name).expect(&format!("FileNotFoundError:{}", file_name));
    
    loop {  
        let next_token = lexer.next_token();
        if next_token == Token::Eof || next_token == Token::Unknown {
            break;
        }
        println!("{}", next_token);
    }
}