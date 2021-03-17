use std::io;

use crate::eval::eval;

mod eval;
mod functions;

fn main() {
    // let mut input = String::new();
    let builtin = ["add (+)", "sub (-)", "mul (*)", "div (/)", "mod (%)", "pow (**)",
        "abs", "neg", "dup", "drop", "swap", "over", "rot (top three stack elements)", "clear (the stack)", "exit"];

    println!("Builtin Functions:");
    println!("{:?}", builtin);

    let mut input: String;

    loop {
        print!(">> ");

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
