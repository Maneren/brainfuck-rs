#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
// credits:
//   thanks to paul for fixing my stupid

mod instructions;
mod memory;
mod parser;

use std::fs;
use std::path::PathBuf;
use std::{io::Read, time::Instant};

use clap::Parser;

use instructions::Instruction;
use memory::Memory;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
  /// The brainfuck program file
  file: PathBuf,

  /// Memory size in bytes. Accepts suffixes k, M, G. Leave empty for dynamically allocated, starting at 256B.
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
  if memory.dynamic() {
    println!("Max memory usage: {}", memory.size());
  }
}

fn run(memory: &mut Memory, parsed: &[Instruction]) -> u64 {
  let mut stdin = std::io::stdin().bytes();
  let mut parsed_index = 0usize;
  let mut counter = 0;

  while let Some(op) = parsed.get(parsed_index) {
    match op {
      Instruction::Increment(count) => memory.increment(*count),
      Instruction::Decrement(count) => memory.decrement(*count),
      Instruction::Right(count) => memory.right(*count),
      Instruction::Left(count) => memory.left(*count),
      Instruction::Print => print!("{}", memory.get() as char),
      Instruction::Read => memory.set(stdin.next().unwrap_or(Ok(0)).unwrap_or_default()),
      Instruction::JumpIfZero(target) => {
        if memory.get() == 0 {
          parsed_index = *target;
        }
      }
      Instruction::JumpIfNonZero(target) => {
        if memory.get() != 0 {
          parsed_index = *target;
        }
      }
    }

    counter += 1;
    parsed_index += 1;
  }

  counter
}

fn create_memory(memory_size: Option<String>) -> Memory {
  if let Some(mem_size_input) = memory_size {
    let number = match mem_size_input[..mem_size_input.len() - 1].parse::<u32>() {
      Ok(n) => n,
      _ => panic!("Invalid memory size!"),
    };

    let unit = match &mem_size_input[mem_size_input.len() - 1..] {
      "" => 1,
      "k" => 1024,
      "M" => 1024 * 1024,
      "G" => 1024 * 1024 * 1024,
      _ => panic!("Invalid memory unit!"),
    };

    let mem_size = (number * unit) as usize;

    Memory::new(mem_size, false)
  } else {
    Memory::new(256, true)
  }
}

fn link_jumps(input: &mut [Instruction]) {
  let mut left_indexes = Vec::new();

  for i in 0..input.len() {
    match input[i] {
      Instruction::JumpIfZero(..) => left_indexes.push(i),

      Instruction::JumpIfNonZero(..) => {
        let left = match left_indexes.pop() {
          Some(left) => left,
          None => panic!("Unmatched closing bracket!"),
        };

        let right = i;

        input[left] = Instruction::JumpIfZero(right);
        input[right] = Instruction::JumpIfNonZero(left);
      }
      _ => {}
    }
  }

  assert!(left_indexes.is_empty(), "Unmatched opening bracket!");
}
