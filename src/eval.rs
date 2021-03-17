use std::collections::VecDeque;
use crate::eval::Val::{Int, Float};


pub fn eval(s: &str) {
    let iter = s.split_ascii_whitespace().into_iter();

    let mut stack: VecDeque<Val> = VecDeque::new();
    for t in iter {
        if stack.len() >= 1 {
            match t {
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
                    match parse_to_num(t) {
                        Some(Int(i)) => { stack.push_back(Int(i)); }
                        Some(Float(f)) => { stack.push_back(Float(f)); }
                        _ => None,
                    }
                }
            }
        }
    }

    println!("{:?}", stack);
}

#[derive(Debug, Clone, Copy)]
pub enum Val {
    Int(i32),
    Float(f32),
}

fn parse_to_num(s: &str) -> Option<Val> {
    if let Ok(i) = s.parse() {
        Some(Int(i))
    } else if let Ok(f) = s.parse() {
        Some(Float(f))
    } else {
        None
    }
}