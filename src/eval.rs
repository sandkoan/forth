use std::collections::VecDeque;

use crate::functions;

pub fn eval(s: &str) {
    let iter = s.split_ascii_whitespace();

    let mut data_stack: VecDeque<i32> = VecDeque::new();

    for it in iter {
        match it {
            "add" | "+" => functions::fadd(&mut data_stack),
            "sub" | "-" => functions::fsub(&mut data_stack),
            "mul" | "*" => functions::fmul(&mut data_stack),
            "div" | "/" => functions::fdiv(&mut data_stack),
            "mod" | "%" => functions::fmod(&mut data_stack),
            "pow" | "**" => functions::fpow(&mut data_stack),
            "dot" | "." => functions::fdot(&mut data_stack),

            "abs" => functions::fabs(&mut data_stack),
            "neg" => functions::fneg(&mut data_stack),

            "eq" | "=" => functions::feq(&mut data_stack),
            "lt" | "<" => functions::flt(&mut data_stack),
            "lte" | "<=" => functions::flte(&mut data_stack),
            "gt" | ">" => functions::fgt(&mut data_stack),
            "gte" | ">=" => functions::fgte(&mut data_stack),

            "drop" => functions::fdrop(&mut data_stack),
            "dup" => functions::fdup(&mut data_stack),
            "swap" => functions::fswap(&mut data_stack),
            "over" => functions::fover(&mut data_stack),
            "rot" => functions::frot(&mut data_stack),
            "dump" => functions::fdump(&mut data_stack),

            "clear" => functions::fclear(&mut data_stack),

            // ":" => functions::fnew_word(&mut data_stack, iter.next()),

            _ => {
                if let Ok(n) = it.parse::<i32>() {
                    data_stack.push_back(n);
                }
            }
        }
    }
}
