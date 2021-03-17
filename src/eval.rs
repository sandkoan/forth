use std::collections::VecDeque;

pub fn eval(s: &str) {
    let iter = s.split_ascii_whitespace().into_iter();

    let mut stack: VecDeque<i32> = VecDeque::new();
    for it in iter {
        if stack.len() >= 1 {
            match it {
                "+" | "add" => {
                    if let Some(a) = stack.pop_back() {
                        if let Some(b) = stack.pop_back() {
                            stack.push_back(b + a);
                        }
                    }
                }
                "-" | "sub" => {
                    if let Some(a) = stack.pop_back() {
                        if let Some(b) = stack.pop_back() {
                            stack.push_back(b - a);
                        }
                    }
                }
                "*" | "mul" => {
                    if let Some(a) = stack.pop_back() {
                        if let Some(b) = stack.pop_back() {
                            stack.push_back(b * a);
                        }
                    }
                }
                "/" | "div" => {
                    if let Some(a) = stack.pop_back() {
                        if let Some(b) = stack.pop_back() {
                            stack.push_back(b / a);
                        }
                    }
                }
                "%" | "mod" => {
                    if let Some(a) = stack.pop_back() {
                        if let Some(b) = stack.pop_back() {
                            stack.push_back(b % a);
                        }
                    }
                }
                "abs" => {
                    if let Some(a) = stack.pop_back() {
                        stack.push_back(a.abs());
                    }
                }
                "neg" => {
                    if let Some(a) = stack.pop_back() {
                        stack.push_back(-a);
                    }
                }
                "dup" => {
                    if let Some(peek) = stack.pop_back() {
                        stack.push_back(peek);
                        stack.push_back(peek);
                    }
                }
                "drop" => {
                    stack.pop_back();
                }
                "swap" => {
                    if let Some(a) = stack.pop_back() {
                        if let Some(b) = stack.pop_back() {
                            stack.push_back(a);
                            stack.push_back(b);
                        }
                    }
                }
                "over" => {
                    if let Some(a) = stack.get(stack.len() - 2) {
                        stack.push_back(*a);
                    }
                }
                // "rot" => {}

                _ => {
                    match parse_to_num(it) {
                        Some(i) => { stack.push_back(i); }
                        _ => {}
                    }
                }
            }
        }
    }

    println!("{:?}", stack);
}


fn parse_to_num(s: &str) -> Option<i32> {
    if let Ok(i) = s.parse() {
        Some(i)
    } else {
        None
    }
}