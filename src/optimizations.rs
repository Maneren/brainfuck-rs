use std::{collections::VecDeque, num::Wrapping};

use crate::instructions::{
  Instruction::{
    self, BlockEnd, BlockStart, Clear, Decrement, Increment, JumpIfNonZero, JumpIfZero, Left,
    ModifyRun, Right, Shift,
  },
  Run,
};

pub fn link_jumps(input: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::with_capacity(input.len());
  let mut left_indexes = Vec::new();

  for (i, instruction) in input.iter().enumerate() {
    match instruction {
      BlockStart => {
        left_indexes.push(i);
        result.push(BlockStart);
      }
      BlockEnd => {
        let left = match left_indexes.pop() {
          Some(val) => val,
          None => panic!("Unmatched closing bracket!"),
        };

        result[left] = JumpIfZero(i);
        result.push(JumpIfNonZero(left));
      }
      instruction => result.push(instruction.clone()),
    }
  }

  assert!(left_indexes.is_empty(), "Unmatched opening bracket!");

  result
}

pub fn optimize(source: &[Instruction]) -> Vec<Instruction> {
  let first_stage = optimize_clear_loops(source);
  compress_runs(&first_stage)
}

fn optimize_clear_loops(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::with_capacity(source.len());
  let mut i = 0;
  while i < source.len() {
    match (source.get(i), source.get(i + 1), source.get(i + 2)) {
      (Some(BlockStart), Some(Increment | Decrement), Some(BlockEnd)) => {
        result.push(Clear);
        i += 2;
      }
      _ => {
        result.push(source[i].clone());
      }
    }
    i += 1;
  }
  result
}

fn compress_runs(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::new();
  let mut i = 0;

  while let Some(current) = source.get(i) {
    match current {
      Increment | Decrement | Right | Left => {
        let mut memory_pointer = 0;

        let mut offset = Wrapping(0);
        let mut data = VecDeque::from([Wrapping(0)]);

        while i < source.len() {
          match &source[i] {
            Increment => data[memory_pointer] += 1,
            Decrement => data[memory_pointer] -= 1,
            Right => {
              memory_pointer += 1;
              if memory_pointer >= data.len() {
                data.push_back(Wrapping(0));
              }
            }
            Left => {
              if memory_pointer > 0 {
                memory_pointer -= 1;
              } else {
                offset -= 1;

                data.push_front(Wrapping(0));
              }
            }
            _ => {
              i -= 1;
              break;
            }
          }

          i += 1;
        }

        let shift = Wrapping(memory_pointer) + offset;

        // remove unused data
        while let Some(Wrapping(0)) = data.back() {
          data.pop_back();
        }

        while let Some(Wrapping(0)) = data.front() {
          offset += 1;
          data.pop_front();
        }

        if data.is_empty() {
          if shift.0 != 0 {
            result.push(Shift(shift));
          }
        } else {
          let run = Run {
            shift,
            offset,
            data: data.into_iter().collect(),
          };
          result.push(ModifyRun(run));
        }
      }
      _ => result.push(current.clone()),
    }

    i += 1;
  }
  result
}
