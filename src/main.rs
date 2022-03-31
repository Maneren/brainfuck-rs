// credits:
//   thanks to paul for fixing my stupid

mod parser;
mod mem_pointer;

use std::io::Read;
use std::fs;
use std::path::PathBuf;

use clap::Parser;

use parser::Op;
use mem_pointer::MemPointer;

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
    let args = Cli::parse();

    let program = fs::read_to_string(args.file)
        .expect("Couldn't read from file!");

    let mem_size = args.memory_size.unwrap_or(1) * 1024;

    let parsed = parser::parse(&program);

    let mut parsed_index = 0usize;
    let mut stdin = std::io::stdin().bytes();
    let mut ptr = MemPointer::new(mem_size);
    let mut stk = Vec::new();
    let mut mem = vec![0u8; mem_size];

    while let Some(op) = parsed.get(parsed_index) {
        match op {
            Op::CellInc => mem[*ptr] = mem[*ptr].wrapping_add(1),
            Op::CellDec => mem[*ptr] = mem[*ptr].wrapping_sub(1),
            Op::PtrInc => ptr += 1,
            Op::PtrDec => ptr -= 1,
            Op::Print => print!("{}", mem[*ptr] as char),
            Op::Read => mem[*ptr] = stdin.next().unwrap().unwrap(),
            Op::BlkPsh => stk.push(parsed_index - 1),
            Op::BlkPop => {
                if mem[*ptr] != 0 {
                    parsed_index = stk.pop().unwrap()
                }
            }
            Op::Invalid => unreachable!()
        }

        parsed_index += 1;
    }
}

