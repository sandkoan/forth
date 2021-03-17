mod eval;

use std::io;
use crate::eval::eval;

fn main() {
    let mut input = String::new();
    let builtin = vec!["add (+)", "sub (-)", "mul (*)", "div (/)", "mod (%)", "pow (**)",
                       "abs", "neg", "dup", "drop", "swap", "over", "rot (top three stack elements)", "clear (the stack)"];

    println!("Builtin Functions:");
    println!("{:?}", builtin);

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => eval(input.as_str()),
            Err(error) => println!("error: {}", error),
        }
    }
}