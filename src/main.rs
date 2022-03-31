#![warn(clippy::pedantic)]
// credits:
//   thanks to paul for fixing my stupid

mod memory;
mod parser;

use std::{collections::HashMap, fs};
use std::{io::Read, time::Instant};
use std::{io::Write, path::PathBuf};

use clap::Parser;

use memory::Memory;
use parser::Op;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
  /// The brainfuck program file
  file: PathBuf,

  /// Memory size in bytes. Accepts suffixes k, M, G. Default is set to 1k.
  #[clap(short, long)]
  memory_size: Option<String>,
}

fn main() {
  let args = Cli::parse();

  let program = fs::read_to_string(args.file).expect("Couldn't read from file!");

  let parsed = parser::parse(&program);
  let jump_table = create_jump_table(&parsed);
  let mut memory = create_memory(args.memory_size);

  let start = Instant::now();

  run(&mut memory, &parsed, &jump_table);

  let elapsed = start.elapsed();
  println!("\nExecuted in {elapsed:?}");
}

fn run(memory: &mut Memory, parsed: &[Op], jump_table: &HashMap<usize, usize>) {
  let mut stdout = std::io::stdout();
  let mut stdin = std::io::stdin().bytes();
  let mut parsed_index = 0usize;
  let mut counter = 0;

  while let Some(op) = parsed.get(parsed_index) {
    counter += 1;
    match op {
      Op::CellInc => memory.increment(),
      Op::CellDec => memory.decrement(),
      Op::PtrInc => memory.right(),
      Op::PtrDec => memory.left(),
      Op::Print => stdout
        .write_all(&[memory.get()])
        .expect("Couldn't write to stdout!"),
      Op::Read => memory.set(stdin.next().unwrap_or(Ok(0)).unwrap_or_default()),
      Op::BlkPsh => {
        if memory.get() == 0 {
          parsed_index = jump_table[&parsed_index];
        }
      }
      Op::BlkPop => {
        if memory.get() != 0 {
          parsed_index = jump_table[&parsed_index];
        }
      }
      Op::Invalid => unreachable!(),
    }

    parsed_index += 1;
  }

  println!("\nExecuted {} operations", counter);
}

fn create_memory(memory_size: Option<String>) -> Memory {
  if let Some(mem_size_input) = memory_size {
    let number = match mem_size_input[..mem_size_input.len() - 1].parse::<u32>() {
      Ok(n) => n,
      _ => panic!("Invalid memory size!"),
    };

    let unit = match &mem_size_input[mem_size_input.len() - 1..] {
      "k" => 1024,
      "M" => 1024 * 1024,
      "G" => 1024 * 1024 * 1024,
      _ => panic!("Invalid memory unit!"),
    };

    let mem_size = (number * unit) as usize;

    Memory::new(mem_size, false)
  } else {
    Memory::new(512, true)
  }
}

fn create_jump_table(input: &[Op]) -> HashMap<usize, usize> {
  let mut jump_table = HashMap::new();
  let mut left_positions = vec![];

  for (position, operator) in input.iter().enumerate() {
    match operator {
      Op::BlkPsh => left_positions.push(position),

      Op::BlkPop => {
        let left = left_positions.pop().unwrap();
        let right = position;
        jump_table.insert(left, right);
        jump_table.insert(right, left);
      }
      _ => {}
    }
  }

  jump_table
}
