use std::io;
use std::collections::VecDeque;

use crate::Token::{Int, Float};


fn main() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{}", input);
                println!("{:?}", eval(lex(input.as_str())))
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

pub fn eval(mut stack: VecDeque<Token>) -> VecDeque<Token> {
    let top = stack.pop().unwrap();
    match top {
        Token::Add => {
            stack.push_back(stack.pop() + stack.pop());
        }
        Token::Eof | _ => "Error!"
    }
    stack
}

pub fn lex(s: &str) -> VecDeque<Token> {
    let iter = s.split_ascii_whitespace().into_iter();

    let mut stack: VecDeque<Token> = VecDeque::new();
    for t in iter {
        let tok = match t {
            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            "%" => Token::Mod,

            "abs" => Token::Abs,
            "neg" => Token::Neg,

            "dup" => Token::Dup,
            "drop" => Token::Drop,
            "swap" => Token::Swap,
            "over" => Token::Over,
            "rot" => Token::Rot,

            _ => {
                match parse_to_num(t) {
                    Some(Int(i)) => Int(i),
                    Some(Float(f)) => Float(f),
                    None | _ => Token::Eof,
                }
            }
        };
        stack.push(tok);
    }
    stack
}

fn parse_to_num(s: &str) -> Option<Token> {
    if let Ok(i) = s.parse() {
        Some(Int(i))
    } else if let Ok(f) = s.parse() {
        Some(Float(f))
    } else {
        None
    }
}

#[derive(Debug)]
pub enum Token {
    Int(i32),
    Float(f32),

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Abs,
    Neg,

    Dup,
    Drop,
    Swap,
    Over,
    Rot,
    Eof,
}