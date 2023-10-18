use std::env;

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("args {:?}", args);
}
