mod eval;

use std::io;
use std::collections::VecDeque;


fn main() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                println!("{}", input);
                println!("{:?}", eval(input.as_str()))
            }
            Err(error) => println!("error: {}", error),
        }
    }
}