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
) {
  let mut memory = Memory::new(memory_size);

  let mut input = input
    .bytes()
    .map(|b| b.expect("Error when reading from stdin"));
  let mut output = output;

  _interpret(instructions, &mut input, &mut output, &mut memory);
}

fn _interpret(
  instructions: &[Instruction],
  reader: &mut impl Iterator<Item = u8>,
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
        let is_exact = memory.get_raw() % linearity_factor == Wrapping(0);

        let ptr = memory.ptr.wrapping_add_signed(*offset);

        memory.check_length(ptr + data.len());

        if is_exact {
          data
            .iter()
            .map(|value| value * factor)
            .enumerate()
            .for_each(|(i, value)| {
              memory[ptr + i] += value;
            });
        } else {
          simple_loop(memory, *offset, data, 0);
        }
      }
      Instruction::SimpleLoop {
        shift,
        offset,
        data,
      } => {
        let last_ptr = memory.ptr + *offset as usize + data.len();

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

      Instruction::Print => write_char(writer, memory.get()),
      Instruction::Read => read_char(reader, memory),

      Instruction::Shift(amount) => memory.shift(*amount),

      Instruction::Set(value) => memory.set_raw(*value),
      Instruction::SetOffset(amount, offset) => {
        let ptr = memory.ptr.wrapping_add_signed(*offset);
        memory.check_length(ptr + 1);
        memory[ptr] = *amount;
      }

      Instruction::Modify(amount) => *memory.get_mut() += *amount,
      Instruction::ModifyOffset(amount, offset) => {
        let ptr = memory.ptr.wrapping_add_signed(*offset);
        memory.check_length(ptr + 1);
        memory[ptr] += *amount;
      }

      _ => unreachable!("{op:?}"),
    }
  }
}

fn write_char(writer: &mut impl Write, char: u8) {
  writer
    .write_all(&[char])
    .expect("Error when writing to output");
}

fn simple_loop(memory: &mut Memory, offset: isize, data: &[Wrapping<u8>], shift: isize) {
  while memory.get() != 0 {
    modify_run(memory, offset, data, shift);
  }
}

fn read_char(input: &mut impl Iterator<Item = u8>, memory: &mut Memory) {
  memory.set(input.next().unwrap_or(0));
}

fn modify_run(memory: &mut Memory, offset: isize, data: &[Wrapping<u8>], shift: isize) {
  let ptr = memory.ptr.wrapping_add_signed(offset);

  memory.check_length(ptr + data.len());

  data.iter().enumerate().for_each(|(i, value)| {
    memory[ptr + i] += value;
  });

  memory.shift(shift);
}
