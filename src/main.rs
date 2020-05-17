mod tokenizer;
use tokenizer::{Token, Tokenizer};

fn main() {
    println!("Hello, world!");

    let mut test = Tokenizer::new("10 + 6".chars());

    while test.current_token != Token::EOF {
        test.next_token();
        print!("{:?}", test.current_token);
        if test.current_token == Token::Number {
            print!(": {}", test.number);
        }
        println!();
    }
}
