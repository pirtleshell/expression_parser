mod parser;
mod tree;
mod tokenizer;
use parser::evaluate;

fn main() {
    println!("Evaluate some expressions!");
    while true {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read");
        println!("{}", evaluate(&input));
    }
}
