use std::{env, process};
use postfix_calculator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = postfix_calculator::run(&args[1]) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}