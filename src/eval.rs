use std::collections::VecDeque;

pub fn eval(s: &str) {
    let iter = s.split_ascii_whitespace();

    let mut stack: VecDeque<i32> = VecDeque::new();

    for it in iter {
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
            "**" | "pow" => {
                if let Some(a) = stack.pop_back() {
                    if let Some(b) = stack.pop_back() {
                        stack.push_back(b.pow(a as u32));
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
                if let Some(a) = stack.pop_back() {
                    stack.push_back(a);
                    stack.push_back(a);
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
                if let Some(a) = stack.pop_back() {
                    if let Some(b) = stack.pop_back() {
                        stack.push_back(b);
                        stack.push_back(a);
                        stack.push_back(b);
                    }
                }
            }
            "rot" => {
                if let Some(a) = stack.pop_back() {
                    if let Some(b) = stack.pop_back() {
                        if let Some(c) = stack.pop_back() {
                            stack.push_back(b);
                            stack.push_back(a);
                            stack.push_back(c);
                        }
                    }
                }
            }

            "clear" => {
                stack.clear();
            }

            _ => {
                if let Ok(n) = it.parse::<i32>() {
                    stack.push_back(n);
                }
            }
        }
    }

    println!("{:?}", stack);
}