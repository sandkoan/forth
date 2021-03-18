use std::collections::VecDeque;

pub fn fdump(data_stack: &mut VecDeque<i32>) {
    println!("data_stack = {:?}", data_stack);
}

pub fn frot(data_stack: &mut VecDeque<i32>) {
    if let (Some(c), Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(b);
        data_stack.push_back(c);
        data_stack.push_back(a);
    }
}

pub fn fover(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a);
        data_stack.push_back(b);
        data_stack.push_back(a);
    }
}

pub fn fswap(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(b);
        data_stack.push_back(a);
    }
}

pub fn fdup(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        data_stack.push_back(a);
        data_stack.push_back(a);
    }
}

pub fn fdrop(data_stack: &mut VecDeque<i32>) {
    data_stack.pop_back();
}

pub fn fdot(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        println!("{}", a);
    }
}

pub fn fneg(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        data_stack.push_back(-a);
    }
}

pub fn fabs(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        data_stack.push_back(a.abs());
    }
}

pub fn fpow(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if a < 0 {
            println!("Error: Exponent must be positive!")
        } else if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(b.pow(a as u32))
        }
    }
}

pub fn fmod(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a % b);
    }
}

pub fn fdiv(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a / b);
    }
}

pub fn fmul(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a * b);
    }
}

pub fn fsub(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a - b);
    }
}

pub fn fadd(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        data_stack.push_back(a + b);
    }
}

pub fn fclear(data_stack: &mut VecDeque<i32>) {
    data_stack.clear();
}

pub fn feq(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a == b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    }
}

pub fn flt(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a < b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    }
}

pub fn flte(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a <= b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    }
}

pub fn fgt(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a > b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    }
}

pub fn fgte(data_stack: &mut VecDeque<i32>) {
    if let (Some(b), Some(a)) = (data_stack.pop_back(), data_stack.pop_back()) {
        if a >= b { data_stack.push_back(1); } else { data_stack.push_back(0); }
    }
}

pub fn fnew_word(data_stack: &mut VecDeque<i32>, index: &usize, code: &Vec<&str>) {
    unimplemented!()
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
    for i in builtin { println!(i); }
}
