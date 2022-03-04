// credits:
//   thanks to paul for fixing my stupid


use std::collections::VecDeque;
use std::io::Read;
use crate::parser::Op;

mod parser;

fn main() {
    let mut stk = VecDeque::<usize>::new();

    let mut mem = vec![0u8; u16::MAX as usize];
    let mut ptr = 0usize;

    let program = "-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[->+++++<]>+.------------.---[->+++<]>.-[--->+<]>---.+++.------.--------.-[--->+<]>.";
    let parsed = parser::parse(program.to_owned());
    let mut parsed_index = 0usize;

    let mut stdin_iterator = std::io::stdin().bytes();

    loop {
        let op = parsed.get(parsed_index);
        parsed_index += 1;

        if let Some(op) = op {
            match op {
                Op::CellInc => mem[ptr] = mem[ptr].wrapping_add(1),
                Op::CellDec => mem[ptr] = mem[ptr].wrapping_sub(1),
                Op::PtrInc => ptr = ptr.wrapping_add(1),
                Op::PtrDec => ptr = ptr.wrapping_sub(1),
                Op::Print => print!("{}", mem[ptr] as char),
                Op::Read => mem[ptr] = stdin_iterator.next().unwrap().unwrap(),
                Op::BlkPsh => stk.push_back(parsed_index - 1),
                Op::BlkPop => parsed_index = if mem[ptr] != 0 { stk.pop_back().unwrap() } else { parsed_index },
            }
        } else { break; }
    }
}



