use std::collections::VecDeque;
use std::io;
use std::io::Write;

pub fn fdump(data_stack: &mut VecDeque<i32>) {
    println!("data_stack = {:?}", data_stack);
}

pub fn frot(data_stack: &mut VecDeque<i32>) {
    if let (Some(c), Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(b);
        data_stack.push_back(c);
        data_stack.push_back(a);
    } else {
        println!("Wrong number of arguments to rot!")
    }
}

pub fn fover(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a);
        data_stack.push_back(b);
        data_stack.push_back(a);
    } else {
        println!("Wrong number of arguments to over!")
    }
}

pub fn fswap(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(b);
        data_stack.push_back(a);
    } else {
        println!("Wrong number of arguments to swap!")
    }
}

pub fn fdup(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        data_stack.push_back(a);
        data_stack.push_back(a);
    } else {
        println!("Wrong number of arguments to dup!")
    }
}

pub fn fdrop(data_stack: &mut VecDeque<i32>) {
    data_stack.pop_back();
}

pub fn fdot(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        println!("{}", a);
    } else {
        println!("Wrong number of arguments to dot!")
    }
}

pub fn fneg(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        data_stack.push_back(-a);
    } else {
        println!("Wrong number of arguments to neg!")
    }
}

pub fn fabs(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        data_stack.push_back(a.abs());
    } else {
        println!("Wrong number of arguments to abs!")
    }
}

pub fn fpow(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if a < 0 {
            println!("Error: Exponent must be positive!")
        } else if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(b.pow(a as u32))
        }
    } else {
        println!("Wrong number of arguments to pow!")
    }
}

pub fn fmod(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a % b);
    } else {
        println!("Wrong number of arguments to mod!")
    }
}

pub fn fdiv(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a / b);
    } else {
        println!("Wrong number of arguments to div!")
    }
}

pub fn fmul(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a * b);
    } else {
        println!("Wrong number of arguments to mul!")
    }
}

pub fn fsub(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a - b);
    } else {
        println!("Wrong number of arguments to sub!")
    }
}

pub fn fadd(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a + b);
    } else {
        println!("Wrong number of arguments to add!")
    }
}

pub fn fclear(data_stack: &mut VecDeque<i32>) {
    data_stack.clear();
}

pub fn feq(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a == b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    } else {
        println!("Wrong number of arguments to eq!")
    }
}

pub fn flt(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a < b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    } else {
        println!("Wrong number of arguments to lt!")
    }
}

pub fn flte(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a <= b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    } else {
        println!("Wrong number of arguments to lte!")
    }
}

pub fn fgt(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a > b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    } else {
        println!("Wrong number of arguments to gt!")
    }
}

pub fn fgte(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a >= b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    } else {
        println!("Wrong number of arguments to gte!")
    }
}

pub fn fnew_word(data_stack: &mut VecDeque<i32>, index: &usize, code: &[&str]) {
    todo!()
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

