#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
// credits:
//   thanks to paul for fixing my stupid

mod memory;
mod parser;

use std::fs;
use std::path::PathBuf;
use std::{io::Read, time::Instant};

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

  let start = Instant::now();
  let mut parsed = parser::parse(&program);
  link_jumps(&mut parsed);
  let mut memory = create_memory(args.memory_size);

  let ops = run(&mut memory, &parsed);

  let elapsed = start.elapsed();
  let ops_per_second = ops as f64 / elapsed.as_secs_f64() / 1_000_000_f64;
  println!("\nExecuted in {elapsed:?} ({ops_per_second:.2}M ops/s)");
}

fn run(memory: &mut Memory, parsed: &[Op]) -> u64 {
  let mut stdin = std::io::stdin().bytes();
  let mut parsed_index = 0usize;
  let mut counter = 0;

  while let Some(op) = parsed.get(parsed_index) {
    match op {
      Op::Increment(count) => memory.increment(*count),
      Op::Decrement(count) => memory.decrement(*count),
      Op::Right(count) => memory.right(*count),
      Op::Left(count) => memory.left(*count),
      Op::Print => print!("{}", memory.get() as char),
      Op::Read => memory.set(stdin.next().unwrap_or(Ok(0)).unwrap_or_default()),
      Op::JumpIfZero(target) => {
        if memory.get() == 0 {
          parsed_index = *target;
        }
      }
      Op::JumpIfNonZero(target) => {
        if memory.get() != 0 {
          parsed_index = *target;
        }
      }
      Op::Invalid => unreachable!(),
    }

    counter += 1;
    parsed_index += 1;
  }

  println!("\nExecuted {} operations", counter);
  counter
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

fn link_jumps(input: &mut [Op]) {
  let mut left_indexes = Vec::new();

  for i in 0..input.len() {
    match input[i] {
      Op::JumpIfZero(..) => left_indexes.push(i),

      Op::JumpIfNonZero(..) => {
        let left = match left_indexes.pop() {
          Some(left) => left,
          None => panic!("Unmatched closing bracket!"),
        };

        let right = i;

        input[left] = Op::JumpIfZero(right);
        input[right] = Op::JumpIfNonZero(left);
      }
      _ => {}
    }
  }

  assert!(left_indexes.is_empty(), "Unmatched opening bracket!");
}
