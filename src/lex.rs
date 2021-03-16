pub enum Token {
    Int(i32),
    // Float(f32),
    // String(String),
    Oper(Operator)
}

pub enum Operator {
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
    Rot
}

