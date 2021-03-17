use std::collections::VecDeque;
use crate::eval::Token::{Int, Float};
use crate::eval::Val::{Int, Float};


pub fn eval(s: &str) -> VecDeque<Val> {
    let iter = s.split_ascii_whitespace().into_iter();

    let mut stack: VecDeque<Val> = VecDeque::new();
    for t in iter {
        if !stack.is_empty() {
            match t {
                "+" => {
                    stack.push_back(stack.pop_back().unwrap() + stack.pop_back().unwrap());
                }
                "-" => {
                    stack.push_back(stack.pop_back().unwrap() - stack.pop_back().unwrap());
                }
                "*" => {
                    stack.push_back(stack.pop_back().unwrap() * stack.pop_back().unwrap());
                }
                "/" => {
                    stack.push_back(stack.pop_back().unwrap() / stack.pop_back().unwrap());
                }
                "%" => {
                    stack.push_back(stack.pop_back().unwrap() % stack.pop_back().unwrap());
                }
                "abs" => {
                    stack.push_back(stack.pop_back().unwrap().abs());
                }
                "neg" => {
                    stack.push_back(-stack.pop_back().unwrap());
                }
                "dup" => {
                    if let Some(peek) = stack.pop_back() {
                        stack.push_back(peek);
                        stack.push_back(peek);
                    }
                }
                "drop" => Token::Drop,
                "swap" => Token::Swap,
                "over" => Token::Over,
                "rot" => Token::Rot,

                _ => {
                    match parse_to_num(t) {
                        Some(Int(i)) => Int(i),
                        Some(Float(f)) => Float(f),
                        _ => None,
                    }
                }
            }
        }
    }

    stack
}

#[derive(Debug)]
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