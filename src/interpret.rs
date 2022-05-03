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
          apply_simple_loop(memory, *offset, data, 0);
        }
      }
      Instruction::SimpleLoop {
        shift,
        offset,
        data,
      } => {
        let last_ptr = memory.ptr + Wrapping(*offset as usize) + Wrapping(data.len());

        memory.check_length(last_ptr);

        apply_simple_loop(memory, *offset, data, *shift);
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

      _ => unreachable!("{op:?}"),
    }
  }
}

fn apply_simple_loop(memory: &mut Memory, offset: i32, data: &[Wrapping<u8>], shift: i32) {
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
  let Memory { mut ptr, .. } = memory;
  ptr += offset as usize;

  memory.check_length(ptr + Wrapping(data.len()));

  data.iter().enumerate().for_each(|(i, value)| {
    memory[ptr + Wrapping(i)] += value;
  });

  memory.shift(shift);
}
