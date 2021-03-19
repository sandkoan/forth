use std::io;

use crate::eval::eval;
use std::io::Write;

mod eval;
mod functions;

fn main() {
    println!("Welcome to the Forth REPL! Type help for help.");
    io::stdout().flush();

    let mut input: String;

    loop {
        print!(">> ");
        io::stdout().flush();

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
