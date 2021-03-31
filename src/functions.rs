use std::io;
use std::io::Write;
use std::str::SplitAsciiWhitespace;
use std::collections::HashMap;

pub fn fdump(data_stack: &mut Vec<i32>) {
    println!("data_stack = {:?}", data_stack);
}

pub fn frot(data_stack: &mut Vec<i32>) {
    if let (Some(c), Some(b), Some(a)) = (data_stack.pop(), data_stack.pop(), data_stack.pop()) {
        data_stack.push(b);
        data_stack.push(c);
        data_stack.push(a);
    } else {
        println!("Wrong number of arguments to rot!")
    }
}

pub fn fover(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        data_stack.push(a);
        data_stack.push(b);
        data_stack.push(a);
    } else {
        println!("Wrong number of arguments to over!")
    }
}

pub fn fswap(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        data_stack.push(b);
        data_stack.push(a);
    } else {
        println!("Wrong number of arguments to swap!")
    }
}

pub fn fdup(data_stack: &mut Vec<i32>) {
    if let Some(a) = data_stack.pop() {
        data_stack.push(a);
        data_stack.push(a);
    } else {
        println!("Wrong number of arguments to dup!")
    }
}

pub fn fdrop(data_stack: &mut Vec<i32>) {
    data_stack.pop();
}

pub fn fdot(data_stack: &mut Vec<i32>) {
    if let Some(a) = data_stack.pop() {
        println!("{}", a);
    } else {
        println!("Wrong number of arguments to dot!")
    }
}

pub fn fneg(data_stack: &mut Vec<i32>) {
    if let Some(a) = data_stack.pop() {
        data_stack.push(-a);
    } else {
        println!("Wrong number of arguments to neg!")
    }
}

pub fn fabs(data_stack: &mut Vec<i32>) {
    if let Some(a) = data_stack.pop() {
        data_stack.push(a.abs());
    } else {
        println!("Wrong number of arguments to abs!")
    }
}

pub fn fpow(data_stack: &mut Vec<i32>) {
    if let Some(a) = data_stack.pop() {
        if a < 0 {
            println!("Error: Exponent must be positive!")
        } else if let Some(b) = data_stack.pop() {
            data_stack.push(b.pow(a as u32))
        }
    } else {
        println!("Wrong number of arguments to pow!")
    }
}

pub fn fmod(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        data_stack.push(a % b);
    } else {
        println!("Wrong number of arguments to mod!")
    }
}

pub fn fdiv(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        data_stack.push(a / b);
    } else {
        println!("Wrong number of arguments to div!")
    }
}

pub fn fmul(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        data_stack.push(a * b);
    } else {
        println!("Wrong number of arguments to mul!")
    }
}

pub fn fsub(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        data_stack.push(a - b);
    } else {
        println!("Wrong number of arguments to sub!")
    }
}

pub fn fadd(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        data_stack.push(a + b);
    } else {
        println!("Wrong number of arguments to add!")
    }
}

pub fn fclear(data_stack: &mut Vec<i32>) {
    data_stack.clear();
}

pub fn feq(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        if a == b { data_stack.push(1); } else { data_stack.push(0); }
    } else {
        println!("Wrong number of arguments to eq!")
    }
}

pub fn flt(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        if a < b { data_stack.push(1); } else { data_stack.push(0); }
    } else {
        println!("Wrong number of arguments to lt!")
    }
}

pub fn flte(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        if a <= b { data_stack.push(1); } else { data_stack.push(0); }
    } else {
        println!("Wrong number of arguments to lte!")
    }
}

pub fn fgt(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        if a > b { data_stack.push(1); } else { data_stack.push(0); }
    } else {
        println!("Wrong number of arguments to gt!")
    }
}

pub fn fgte(data_stack: &mut Vec<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop(), data_stack.pop()) {
        if a >= b { data_stack.push(1); } else { data_stack.push(0); }
    } else {
        println!("Wrong number of arguments to gte!")
    }
}

pub fn fnew_word(tokens: &mut SplitAsciiWhitespace, word_dict: &mut HashMap<&str, &mut str>) {
    let def = tokens.next().unwrap();

    let mut opt = None;

    while let Some(body) = tokens.next() {
        if body != ";" {
            opt = word_dict.insert(def, if let Some(b) = word_dict.get(def) { b + body } else {body});
        }
    }

    match opt {
        Some(old_body) => println!("Word {} was defined as {}", def, old_body),
        _ => {}
    }
}

pub fn help() {
    let builtin = [
        "add (+)",
        "sub (-)",
        "mul (*)",
        "div (/)",
        "mod (%)",
        "pow (**)",
        "dot (.) - prints out top of stack",
        "dump - prints out whole stack",
        "abs",
        "neg",
        "dup",
        "drop",
        "swap",
        "over",
        "rot - rotates top three stack elements",
        "clear (the stack)",
        "exit"
    ];

    println!("Forth is a postfix, Stack-oriented programming language.");
    println!("Builtin functions:");
    builtin.iter().for_each(|x| println!("{}", x));
    io::stdout().flush();
}

