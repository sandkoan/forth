use std::collections::VecDeque;

pub fn fdump(data_stack: &mut VecDeque<i32>) {
    println!("data_stack = {:?}", data_stack);
}

pub fn frot(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            if let Some(c) = data_stack.pop_back() {
                data_stack.push_back(b);
                data_stack.push_back(a);
                data_stack.push_back(c);
            }
        }
    }
}

pub fn fover(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(b);
            data_stack.push_back(a);
            data_stack.push_back(b);
        }
    }
}

pub fn fswap(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(a);
            data_stack.push_back(b);
        }
    }
}

pub fn fdup(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        data_stack.push_back(a);
        data_stack.push_back(a);
    }
}

pub fn fdrop(data_stack: &mut VecDeque<i32>) {
    data_stack.pop_back();
}

pub fn fdot(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        println!("{}", a);
    }
}

pub fn fneg(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        data_stack.push_back(-a);
    }
}

pub fn fabs(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        data_stack.push_back(a.abs());
    }
}

pub fn fpow(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if a < 0 {
            println!("Error: Exponent must be positive!")
        } else if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(b.pow(a as u32))
        }
    }
}

pub fn fmod(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(b % a);
        }
    }
}

pub fn fdiv(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(b / a);
        }
    }
}

pub fn fmul(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(b * a);
        }
    }
}

pub fn fsub(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(b - a);
        }
    }
}

pub fn fadd(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            data_stack.push_back(b + a);
        }
    }
}

pub fn fclear(data_stack: &mut VecDeque<i32>) {
    data_stack.clear();
}

pub fn feq(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            if b == a { data_stack.push_back(1) } else { data_stack.push_back(0) }
        }
    }
}

pub fn flt(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            if b < a { data_stack.push_back(1) } else { data_stack.push_back(0) }
        }
    }
}

pub fn flte(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            if b <= a { data_stack.push_back(1) } else { data_stack.push_back(0) }
        }
    }
}

pub fn fgt(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            if b > a { data_stack.push_back(1) } else { data_stack.push_back(0) }
        }
    }
}

pub fn fgte(data_stack: &mut VecDeque<i32>) {
    if let Some(a) = data_stack.pop_back() {
        if let Some(b) = data_stack.pop_back() {
            if b >= a { data_stack.push_back(1) } else { data_stack.push_back(0) }
        }
    }
}

