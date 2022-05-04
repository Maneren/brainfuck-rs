use std::{
  io::{Bytes, Read, Write},
  num::Wrapping,
};

use crate::{instructions::Instruction, memory::Memory};

pub fn interpret(
  instructions: &[Instruction],
  input: impl Read,
  output: impl Write,
  memory_size: usize,
) {
  let mut memory = Memory::new(memory_size);

  let mut input = input.bytes();
  let mut output = output;

  _interpret(instructions, &mut input, &mut output, &mut memory);
}

fn _interpret(
  instructions: &[Instruction],
  reader: &mut Bytes<impl Read>,
  writer: &mut impl Write,
  memory: &mut Memory,
) {
  for op in instructions {
    match op {
      Instruction::LinearLoop {
        offset,
        linearity_factor,
        data,
      } => {
        if memory.get() == 0 {
          continue;
        }

        let factor = memory.get_raw() / linearity_factor;
        let remainder = memory.get_raw() % linearity_factor;

        let ptr = memory.ptr + Wrapping(*offset as usize);

        memory.check_length(ptr + Wrapping(data.len()));

        let factor = if remainder == Wrapping(0) {
          factor
        } else {
          let mut tmp = memory.get_raw();
          let mut i = Wrapping(0);
          while tmp != Wrapping(0) {
            tmp -= linearity_factor;
            i += 1;
          }
          i
        };

        data
          .iter()
          .map(|value| value * factor)
          .enumerate()
          .for_each(|(i, value)| {
            memory[ptr + Wrapping(i)] += value;
          });
      }
      Instruction::SimpleLoop {
        shift,
        offset,
        data,
      } => {
        let last_ptr = memory.ptr + Wrapping(*offset as usize) + Wrapping(data.len());

        memory.check_length(last_ptr);

        simple_loop(memory, *offset, data, *shift);
      }
      Instruction::SearchLoop { step } => {
        while memory.get() != 0 {
          memory.shift(*step);
        }
      }
      Instruction::Loop { instructions } => {
        while memory.get() != 0 {
          _interpret(instructions, reader, writer, memory);
        }
      }
      Instruction::ModifyRun {
        shift,
        offset,
        data,
      } => modify_run(memory, *offset, data, *shift),
      Instruction::Print => writer.write_all(&[memory.get()]).expect("Could not output"),
      Instruction::Read => read_char(reader, memory),
      Instruction::Clear => memory.set(0),
      Instruction::Shift(amount) => memory.shift(*amount),
      Instruction::Modify(amount) => {
        let ptr = memory.ptr;
        memory[ptr] += *amount;
      }
      Instruction::ModifyOffset(amount, offset) => {
        let ptr = memory.ptr + Wrapping(*offset as usize);
        memory[ptr] += *amount;
      }

      _ => unreachable!("{op:?}"),
    }
  }
}

fn simple_loop(memory: &mut Memory, offset: i32, data: &[Wrapping<u8>], shift: i32) {
  while memory.get() != 0 {
    modify_run(memory, offset, data, shift);
  }
}

fn read_char(input: &mut Bytes<impl Read>, memory: &mut Memory) {
  // if stdin empty, use NULL char
  let input = input
    .next()
    .unwrap_or(Ok(0))
    .expect("Error when reading from stdin");

  memory.set(input);
}

fn modify_run(memory: &mut Memory, offset: i32, data: &[Wrapping<u8>], shift: i32) {
  let ptr = memory.ptr + Wrapping(offset as usize);

  memory.check_length(ptr + Wrapping(data.len()));

  data.iter().enumerate().for_each(|(i, value)| {
    memory[ptr + Wrapping(i)] += value;
  });

  memory.shift(shift);
}
