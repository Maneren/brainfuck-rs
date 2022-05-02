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
    stats[op.index()] += 1;

    match op {
      Instruction::LinearLoop {
        offset,
        linearity_factor,
        data,
      } => {
        let ptr = (memory.ptr - Wrapping(*offset as usize)).0;
        memory.check_length(ptr + data.len());

        let factor = memory[ptr] / linearity_factor;
        let is_exact = memory[ptr] % linearity_factor == Wrapping(0);

        let cpy = (memory.clone(), factor, memory[ptr]);
        let mut mem2 = memory.clone();
        data
          .iter()
          .map(|value| value * factor)
          .enumerate()
          .for_each(|(i, value)| {
            mem2[ptr + i] += value;
          });

        apply_simple_loop(&mut memory, *offset, data, 0);

        if is_exact {
          for (i, (a, b)) in memory.data.iter().zip(mem2.data.iter()).enumerate() {
            if a != b {
              println!("{} {} {}", i, a, b);
              println!("{:?}\n{:?}", memory, mem2,);
              let (mem_cpy, factor, value) = cpy;
              panic!("\n{:?}\nop: {:?}\n{} {}", mem_cpy, op, factor, value)
            }
          }
        }

        /*  if is_exact {
          data
            .iter()
            .map(|value| value * factor)
            .enumerate()
            .for_each(|(i, value)| {
              memory[ptr + i] += value;
            });
        } else {
          apply_simple_loop(&mut memory, *offset, data, 0);
        } */
      }
      Instruction::SimpleLoop {
        shift,
        offset,
        data,
      } => {
        memory.check_length(memory.ptr.0 + *offset as usize + data.len());

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
    counter += 1;
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
  let ptr = (memory.ptr + Wrapping(offset as usize)).0;
  memory.check_length(ptr + data.len());

  data.iter().enumerate().for_each(|(i, value)| {
    memory[ptr + i] += value;
  });

  memory.shift(shift);
}
