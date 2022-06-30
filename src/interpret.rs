use std::io::{Bytes, Read, Write};

use wrapping_proc_macro::wrapping;

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
      Instruction::LinearLoop { .. }
      | Instruction::SimpleLoop { .. }
      | Instruction::Loop(..)
      | Instruction::SearchLoop { .. }
        if memory.get() == 0 => {}

      Instruction::LinearLoop {
        offset,
        linearity_factor,
        data,
      } => {
        let factor = memory.get() / linearity_factor;
        let is_exact = memory.get() % linearity_factor == 0;

        wrapping! {
          let ptr = memory.ptr + *offset as usize;
          memory.check_length(ptr + data.len());
        }

        if is_exact {
          wrapping! {
            data
              .iter()
              .map(|value| value * factor)
              .enumerate()
              .for_each(|(i, value)| {
                 memory[ptr + i] += value;
              });
          }
        } else {
          simple_loop(memory, *offset, data, 0);
        }
      }

      Instruction::SimpleLoop {
        shift,
        offset,
        data,
      } => {
        let last_ptr = wrapping! { memory.ptr + *offset as usize + data.len() };

        memory.check_length(last_ptr);

        simple_loop(memory, *offset, data, *shift);
      }

      Instruction::SearchLoop { step } => {
        while memory.get() != 0 {
          memory.shift(*step);
        }
      }

      Instruction::Loop(instructions) => {
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

      Instruction::ClearRun {
        shift,
        offset,
        data,
      } => clear_run(memory, *offset, data, *shift),

      Instruction::SimpleClearLoop {
        shift,
        offset,
        data,
      } => {
        while memory.get() != 0 {
          clear_run(memory, *offset, data, *shift);
        }
      }

      _ => unreachable!("{op:?}"),
    }
  }
}

fn clear_run(memory: &mut Memory, offset: isize, data: &[bool], shift: isize) {
  wrapping! {
    let ptr = memory.ptr + offset as usize;

    memory.check_length(ptr + data.len());

    data.iter().enumerate().for_each(|(i, value)| {
      if *value {
        memory[ptr + i] = 0;
      }
    });
  }

  memory.shift(shift);
}

fn simple_loop(memory: &mut Memory, offset: isize, data: &[u8], shift: isize) {
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

fn modify_run(memory: &mut Memory, offset: isize, data: &[u8], shift: isize) {
  wrapping! {
    let ptr = memory.ptr + offset as usize;

    memory.check_length(ptr + data.len());

    data.iter().enumerate().for_each(|(i, value)| {
      memory[ptr + i] += *value;
    });

    memory.shift(shift);
  }
}
