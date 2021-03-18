use std::io;

use crate::eval::eval;

mod eval;
mod functions;

fn main() {
    println!("Welcome to the Forth REPL! Type help for help.");

    let mut input: String;

    loop {
        input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => if input.contains("exit") {
                println!("Goodbye...!");
                break;
            } else { eval(input.as_str()) },
            Err(error) => println!("Error: {}", error)
        }
    }
}
