// credits:
//   thanks to paul for fixing my stupid


use std::collections::VecDeque;
use std::io::Read;
use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

use crate::parser::Op;

mod parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {

    /// The brainfuck program file
    file: PathBuf,

    /// Memory size in kB (kilobytes). The default is set to 1kB.
    #[clap(short, long)]
    memory_size: Option<usize>,
}

fn main() {
    let args: Cli = Cli::parse();

    let program = fs::read_to_string(args.file)
        .expect("Couldn't read from file!");

    let mut stk = VecDeque::<usize>::new();

    let mut mem = vec![0u8; args.memory_size.unwrap_or(1) * 1024];
    let mut ptr = 0usize;

    let parsed = parser::parse(program);
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


