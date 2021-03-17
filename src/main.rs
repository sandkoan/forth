mod eval;

use std::io;
use crate::eval::eval;

fn main() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => eval(input.as_str()),
            Err(error) => println!("error: {}", error),
        }
    }
}