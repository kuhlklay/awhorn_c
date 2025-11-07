mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let filename = "main.awh";
    let source = std::fs::read_to_string(filename)
        .expect("Failed to read main.awh");

    let mut lexer = Lexer::new(&source);

    loop {
        let t = lexer.next_token();
        println!("{:?}", t);
        if t == token::Token::EOF {
            break;
        }
    }
}