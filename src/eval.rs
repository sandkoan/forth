use std::collections::VecDeque;
use crate::lex::Token;

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