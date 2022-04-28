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

macro_rules! measure_time {
  ($b:block) => {{
    let start = Instant::now();
    let value = $b;
    let elapsed = start.elapsed();

    (value, elapsed)
  }};
}

fn main() {
  let args = Cli::parse();

  let program = fs::read_to_string(args.file).expect("Couldn't read from file!");

  let (instructions, compiled) = measure_time!({ generate_instructions(&program) });

  dbg!(&instructions);

  let mut memory = create_memory(args.memory_size);

  let (ops, executed) = measure_time!({ run(&mut memory, &instructions) });

  let ops_per_second = ops as f64 / executed.as_secs_f64() / 1_000_000_f64;

  println!();
  println!("Compiled in {compiled:?}");
  println!("Executed in {executed:?} ({ops_per_second:.2}M ops/s)");
  println!("Peak memory usage: {}", memory.size());
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
      Instruction::ModifyRun(data) => {
        memory.modify_run(data);
      }
      Instruction::LinearLoop(data) => {
        memory.apply_linear_loop(data);
      }
      Instruction::Print => {
        print!("{}", memory.get() as char);
      }
      Instruction::Read => {
        // if stdin empty, use NULL char
        let input = stdin
          .next()
          .unwrap_or(Ok(0))
          .expect("Error when reading from stdin");
        memory.set(input);
      }
      Instruction::Clear => memory.set(0),
      Instruction::Shift(amount) => {
        memory.shift(*amount);
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
      _ => unreachable!(),
    }

    parsed_index += 1;
    counter += 1;
  }

  counter
}

fn create_memory(memory_size: Option<String>) -> Memory {
  let size = memory_size.map_or(256, |input| {
    let number = input
      .chars()
      .take_while(|c| c.is_digit(10))
      .collect::<String>()
      .parse::<u32>()
      .expect("Invalid memory size!");

    let unit = match input.chars().find(|c| !c.is_digit(10)) {
      None | Some('B') => 1,
      Some('k') => 1024,
      Some('M') => 1024 * 1024,
      Some('G') => 1024 * 1024 * 1024,
      _ => panic!("Invalid memory unit!"),
    };

    (number * unit) as usize
  });

  Memory::new(size)
}
