use std::{
  io::{Read, Write},
  num::Wrapping,
};

use crate::{instructions::Instruction, memory::Memory};

pub fn interpret(
  instructions: &[Instruction],
  input: impl Read,
  output: impl Write,
  memory_size: usize,
) -> u64 {
  let mut memory = Memory::new(memory_size);

  let mut input = input.bytes();
  let mut output = output;

  let mut parsed_index = 0usize;
  let mut counter = 0;

  let mut stats = vec![0u64; 15];

  while let Some(op) = instructions.get(parsed_index) {
    //  stats[op.index()] += 1;
    counter += 1;

    match op {
      Instruction::LinearLoop {
        offset,
        linearity_factor,
        data,
      } => {
        if memory.get() == 0 {
          parsed_index += 1;
          continue;
        }

        let mut ptr = memory.ptr;

        let factor = memory[ptr] / linearity_factor;
        let is_exact = memory[ptr] % linearity_factor == Wrapping(0);

        ptr += *offset as usize;

        memory.check_length(ptr + Wrapping(data.len()));

        if is_exact {
          data
            .iter()
            .map(|value| value * factor)
            .enumerate()
            .for_each(|(i, value)| {
              memory[ptr + Wrapping(i)] += value;
            });
        } else {
          apply_simple_loop(&mut memory, *offset, data, 0);
        }
      }
      Instruction::SimpleLoop {
        shift,
        offset,
        data,
      } => {
        let last_ptr = memory.ptr + Wrapping(*offset as usize) + Wrapping(data.len());

        memory.check_length(last_ptr);

        apply_simple_loop(&mut memory, *offset, data, *shift);
      }
      Instruction::ModifyRun {
        shift,
        offset,
        data,
      } => modify_run(&mut memory, *offset, data, *shift),
      Instruction::Print => output.write_all(&[memory.get()]).expect("Could not output"),
      Instruction::Read => read_char(&mut input, &mut memory),
      Instruction::Clear => memory.set(0),
      Instruction::Shift(amount) => memory.shift(*amount),
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
  }

  println!("stats: {:?}", stats);

  counter
}

fn apply_simple_loop(memory: &mut Memory, offset: i32, data: &[Wrapping<u8>], shift: i32) {
  while memory.get() != 0 {
    modify_run(memory, offset, data, shift);
  }
}

fn read_char(input: &mut std::io::Bytes<impl Read>, memory: &mut Memory) {
  // if stdin empty, use NULL char
  let input = input
    .next()
    .unwrap_or(Ok(0))
    .expect("Error when reading from stdin");

  memory.set(input);
}

fn modify_run(memory: &mut Memory, offset: i32, data: &[Wrapping<u8>], shift: i32) {
  let Memory { mut ptr, .. } = memory;
  ptr += offset as usize;

  memory.check_length(ptr + Wrapping(data.len()));

  data.iter().enumerate().for_each(|(i, value)| {
    memory[ptr + Wrapping(i)] += value;
  });

  memory.shift(shift);
}
