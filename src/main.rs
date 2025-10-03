use crate::lexer::Lexer;

mod lexer;
mod parser;

fn main() {
    let test = include_str!("../test.sh");
    let mut lexer = Lexer::new(test);
    match lexer.tokens() {
        Ok(v) => {
            for t in v {
                println!("{:?}", t);
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
