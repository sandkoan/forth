use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io;
use std::collections::VecDeque;
use crate::Token::{Int, Float};
use std::io::Read;

fn main() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{}", input);
                println!("{:?}", lex(input.as_str()))
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

pub fn eval(mut stack: Vec<Token>) {
    let top = stack.pop().unwrap();
}

pub fn lex(s: &str) -> Vec<Token> {
    let iter = s.split_ascii_whitespace().into_iter();

    let mut stack: Vec<Token> = vec![];
    for t in iter {
        let tok = match t {
            "+" => Token::Bin(BinaryOperator::Add),
            "-" => Token::Bin(BinaryOperator::Sub),
            "*" => Token::Bin(BinaryOperator::Mul),
            "/" => Token::Bin(BinaryOperator::Div),
            "%" => Token::Bin(BinaryOperator::Mod),

            "abs" => Token::Un(UnaryOperator::Abs),
            "neg" => Token::Un(UnaryOperator::Neg),

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
    Bin(BinaryOperator),
    Un(UnaryOperator),
    Dup,
    Drop,
    Swap,
    Over,
    Rot,
    Eof,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Abs,
    Neg,
}