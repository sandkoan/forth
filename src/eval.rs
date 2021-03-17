use std::collections::VecDeque;
use crate::lex::Token;

pub fn eval(mut stack: VecDeque<Token>) -> VecDeque<Token> {
    if let Some(top) = stack.pop() {
        match top {
            Token::Add => {
                stack.push_back(stack.pop() + stack.pop());
            }
            Token::Sub => {
                stack.push_back(stack.pop() - stack.pop());
            }
            _ => "Error!"
        }
    }
    stack
}