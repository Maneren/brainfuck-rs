#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
// credits:
//   fade - base idea and code

mod instructions;
mod interpret;
mod memory;
mod optimizations;
mod parser;

use std::{
  fs,
  io::{stdin, stdout},
  path::PathBuf,
  time::Instant,
};

use clap::Parser;
use instructions::Instruction;
use optimizations::optimize;

use crate::interpret::interpret;

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
    $b;
    let elapsed = start.elapsed();

    elapsed
  }};
}

fn main() {
  let args = Cli::parse();

  let program = fs::read_to_string(args.file).expect("Couldn't read from file!");

  let (instructions, compiled) = measure_time!({ generate_instructions(&program) });

  let memory_size = parse_memory_size(args.memory_size);

  let executed = measure_time!({
    let instructions = generate_instructions(&program);

    interpret(&instructions, stdin(), stdout(), memory_size);
  });

  println!();
  println!("Executed in {executed:?}");
}

fn generate_instructions(source: &str) -> Vec<Instruction> {
  optimize(&parser::parse(source))
}

fn parse_memory_size(memory_size: Option<String>) -> usize {
  memory_size.map_or(256, |input| {
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
  })
}
