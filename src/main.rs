#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
// credits:
//   fade - base idea and code

mod instructions;
mod memory;
mod optimizations;
mod parser;

use std::{
  fs,
  io::{stdin, Read},
  path::PathBuf,
  time::Instant,
};

use clap::Parser;
use instructions::Instruction;
use memory::Memory;
use optimizations::{link_jumps, optimize};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
  /// The brainfuck program file
  file: PathBuf,

  /// Memory size in bytes. Accepts suffixes B, k, M, G. Leave empty for dynamically allocated, starting at 256B.
  #[clap(short, long)]
  memory_size: Option<String>,
}

fn main() {
  let args = Cli::parse();

  let program = fs::read_to_string(args.file).expect("Couldn't read from file!");

  let start = Instant::now();
  let instructions = generate_instructions(&program);
  println!("Compiled in {:?}\n", start.elapsed());

  let mut memory = create_memory(args.memory_size);

  // dbg!(&instructions);

  let ops = run(&mut memory, &instructions);

  let elapsed = start.elapsed();
  let ops_per_second = ops as f64 / elapsed.as_secs_f64() / 1_000_000_f64;
  println!("\nExecuted in {elapsed:?} ({ops_per_second:.2}M ops/s)");
  println!("Max memory usage: {}", memory.size());
}

fn generate_instructions(source: &str) -> Vec<Instruction> {
  link_jumps(&optimize(&parser::parse(source)))
}

fn run(memory: &mut Memory, instructions: &[Instruction]) -> u64 {
  let mut stdin = stdin().bytes();
  let mut parsed_index = 0usize;
  let mut counter = 0;

  while let Some(op) = instructions.get(parsed_index) {
    match op {
      Instruction::Print => print!("{}", memory.get() as char),
      Instruction::Read => {
        // if stdin empty, use NULL char
        let input = stdin.next().unwrap_or(Ok(0)).unwrap();
        memory.set(0, input);
      }
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
      Instruction::JumpIfZeroWithData(target, data) => {
        if memory.get() == 0 {
          parsed_index = *target;
        }
        memory.modify_run(data);
      }
      Instruction::JumpIfNonZeroWithData(target, data) => {
        if memory.get() != 0 {
          parsed_index = *target;
        }
        memory.modify_run(data);
      }
      Instruction::Clear => memory.set(0, 0),
      Instruction::ModifyRun(data) => {
        memory.modify_run(data);
      }
      Instruction::Shift(amount) => memory.shift(*amount),
      _ => unreachable!(),
    }

    counter += 1;
    parsed_index += 1;
  }

  counter
}

fn create_memory(memory_size: Option<String>) -> Memory {
  if let Some(mem_size_input) = memory_size {
    let number = mem_size_input[..mem_size_input.len() - 1]
      .parse::<u32>()
      .expect("Invalid memory size!");

    let unit = match &mem_size_input[mem_size_input.len() - 1..] {
      "B" => 1,
      "k" => 1024,
      "M" => 1024 * 1024,
      "G" => 1024 * 1024 * 1024,
      _ => panic!("Invalid memory unit!"),
    };

    let mem_size = (number * unit) as usize;

    Memory::new(mem_size)
  } else {
    Memory::new(512)
  }
}
