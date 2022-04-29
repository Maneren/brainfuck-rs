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
  num::Wrapping,
  path::PathBuf,
  time::Instant,
};

use clap::Parser;
use instructions::{Instruction, Loop, Run};
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
  println!("Peak memory usage: {}", memory.data.len());
}

fn generate_instructions(source: &str) -> Vec<Instruction> {
  link_jumps(&optimize(&parser::parse(source)))
}

fn run(memory: &mut Memory, instructions: &[Instruction]) -> u64 {
  let mut stdin = stdin().bytes();
  let mut parsed_index = 0usize;
  let mut counter = 0;

  let mut stats = vec![0u64; 14];

  while let Some(op) = instructions.get(parsed_index) {
    stats[op.index()] += 1;

    match op {
      Instruction::ModifyRun(Run {
        shift,
        offset,
        data,
      }) => {
        let ptr = (memory.ptr + Wrapping(*offset as usize)).0;

        memory.check_length(ptr + data.len());

        data.iter().enumerate().for_each(|(i, value)| {
          memory.data[ptr + i] += value;
        });

        memory.shift(*shift);
      }
      Instruction::LinearLoop(Loop {
        data,
        linear_factor,
      }) => {
        let ptr = memory.ptr.0;

        memory.check_length(ptr + data.len());

        let factor = {
          let mut tmp = Wrapping(memory.get());
          let mut i = Wrapping(0);

          while tmp.0 != 0 {
            tmp -= linear_factor;
            i += 1;
          }

          i
        };

        data
          .iter()
          .map(|value| value * factor)
          .enumerate()
          .for_each(|(i, value)| memory.data[ptr + i] += value);
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

  println!("stats: {:?}", stats);

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
