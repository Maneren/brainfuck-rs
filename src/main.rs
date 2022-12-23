#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

// credits:
//   FADEOffical - idea and base code
//   bff4 - inspiration for optimizations

mod instructions;
mod interpret;
mod memory;
mod optimizations;
mod parser;

use std::{
  fs,
  io::{stdin, stdout, Read},
  time::Instant,
};

use clap::Parser;
use optimizations::optimize;

use crate::interpret::interpret;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
  /// Brainfuck program file to interpret
  ///
  /// Leave empty to read from stdin.
  file: Option<String>,

  /// Starting memory size in bytes
  ///
  /// Accepts suffixes B, k, M, G. Default is 256B.
  #[clap(short, long)]
  memory_size: Option<String>,
}

fn main() {
  let args = Cli::parse();

  let program = if let Some(file) = args.file {
    fs::read_to_string(file).expect("Couldn't read from file!")
  } else {
    stdin()
      .bytes()
      .take_while(|b| match b {
        Ok(b'b') => false,
        Ok(..) => true,
        _ => false,
      })
      .map(|ch| ch.expect("Error reading from stdin"))
      .map(char::from)
      .collect()
  };

  let memory_size = parse_memory_size(args.memory_size);

  let elapsed = {
    let start = Instant::now();

    let instructions = parser::parse(&program);
    let instructions = optimize(&instructions);

    interpret(&instructions, stdin(), stdout(), memory_size);

    start.elapsed()
  };

  println!();
  println!("Executed in {elapsed:?}");
}

fn parse_memory_size(memory_size: Option<String>) -> usize {
  memory_size.map_or(256, |input| {
    let number = input
      .chars()
      .take_while(char::is_ascii_digit)
      .collect::<String>()
      .parse::<usize>()
      .expect("Invalid memory size!");

    let unit = match input.chars().find(|c| !c.is_ascii_digit()) {
      None | Some('B') => 1,
      Some('k') => 1024,
      Some('M') => 1024 * 1024,
      Some('G') => 1024 * 1024 * 1024,
      _ => panic!("Invalid memory unit!"),
    };

    number * unit
  })
}
