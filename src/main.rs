use std::env;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();


    println!("args {:?}", args);
}
