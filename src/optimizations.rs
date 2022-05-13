use std::{collections::VecDeque, num::Wrapping};

use crate::instructions::Instruction::{
  self, BlockEnd, BlockStart, Clear, ClearRun, Decrement, Increment, Left, LinearLoop, Loop,
  ModifyRun, Right, SearchLoop, Shift, SimpleClearLoop, SimpleLoop,
};

pub fn optimize(instructions: &[Instruction]) -> Vec<Instruction> {
  let instructions = collect_loops(instructions);
  let instructions = compress_modify_runs(&instructions);
  let instructions = optimize_small_loops(instructions);
  let instructions = compress_clear_runs(&instructions);
  optimize_simple_clear_loops(&instructions)
}

fn collect_loops(input: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::with_capacity(input.len());
  let mut buffer = Vec::new();

  let mut level = 0;

  for instruction in input {
    match instruction {
      BlockStart => {
        level += 1;

        if level > 1 {
          buffer.push(BlockStart);
        }
      }
      BlockEnd => {
        level -= 1;

        if level == 0 {
          if !buffer.is_empty() {
            result.push(Loop(collect_loops(&buffer)));
            buffer.clear();
          }
        } else {
          buffer.push(BlockEnd);
        }
      }
      _ => {
        if level == 0 {
          result.push(instruction.clone());
        } else {
          buffer.push(instruction.clone());
        }
      }
    }
  }

  assert!(level == 0, "Unbalanced brackets");

  result
}

fn optimize_small_loops(source: Vec<Instruction>) -> Vec<Instruction> {
  let mut result = Vec::with_capacity(source.len());
  let mut push = #[inline]
  |val| result.push(val);

  for op in source {
    match op {
      Loop(instructions) => match &instructions[..] {
        [] => {}

        [Shift(amount)] if *amount != 0 => push(SearchLoop { step: *amount }),

        [ModifyRun {
          shift: 0,
          offset: 0,
          data,
        }] if data.len() == 1 => push(Clear),

        [ModifyRun {
          shift,
          offset,
          data,
        }] if *shift == 0 && *offset <= 0 => push(LinearLoop {
          offset: *offset,
          linearity_factor: -data[(-offset) as usize], // value at offset 0
          data: data.clone(),
        }),

        [ModifyRun {
          shift,
          offset,
          data,
        }] => push(SimpleLoop {
          shift: *shift,
          offset: *offset,
          data: data.clone(),
        }),

        [ClearRun {
          shift,
          offset,
          data,
        }] => push(SimpleClearLoop {
          shift: *shift,
          offset: *offset,
          data: data.clone(),
        }),

        _ => push(Loop(optimize_small_loops(instructions))),
      },

      _ => push(op.clone()),
    }
  }

  result
}

fn compress_modify_runs(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::new();
  let mut i = 0;

  while let Some(current) = source.get(i) {
    match current {
      Increment | Decrement | Right | Left => {
        let mut memory_pointer = 0;

        let mut offset = 0;
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

        let shift = memory_pointer as isize + offset;

        // remove unused data
        while let Some(Wrapping(0)) = data.back() {
          data.pop_back();
        }

        while let Some(Wrapping(0)) = data.front() {
          offset += 1;
          data.pop_front();
        }

        if data.is_empty() {
          if shift != 0 {
            result.push(Shift(shift));
          }
        } else {
          result.push(ModifyRun {
            shift,
            offset,
            data: data.into_iter().collect(),
          });
        }
      }
      Loop(instructions) => {
        result.push(Loop(compress_modify_runs(instructions)));
      }
      _ => result.push(current.clone()),
    }

    i += 1;
  }
  result
}

fn compress_clear_runs(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::new();
  let mut i = 0;

  while let Some(current) = source.get(i) {
    match current {
      Shift(..) | Clear => {
        let mut memory_pointer = 0;

        let mut offset = 0;
        let mut data = VecDeque::from([false]);

        while i < source.len() {
          match &source[i] {
            Clear => data[memory_pointer] |= true,

            Shift(amount) => {
              if *amount < 0 && amount.unsigned_abs() > memory_pointer {
                let delta = amount.unsigned_abs() - memory_pointer;

                offset -= delta as isize;
                memory_pointer = 0;

                (0..delta).into_iter().for_each(|_| data.push_front(false));
              } else {
                memory_pointer = memory_pointer.wrapping_add_signed(*amount);

                if memory_pointer >= data.len() {
                  data.resize(memory_pointer + 1, false);
                }
              }
            }

            _ => {
              i -= 1;
              break;
            }
          }

          i += 1;
        }

        let shift = memory_pointer as isize + offset;

        // remove unused data
        while let Some(false) = data.back() {
          data.pop_back();
        }

        while let Some(false) = data.front() {
          offset += 1;
          data.pop_front();
        }

        if data.is_empty() {
          if shift != 0 {
            result.push(Shift(shift));
          }
        } else if data.len() == 1 && offset == 0 {
          result.push(Clear);

          if shift != 0 {
            result.push(Shift(shift));
          }
        } else if data.len() == 1 && offset == shift {
          result.push(Shift(shift));
          result.push(Clear);
        } else {
          result.push(ClearRun {
            shift,
            offset,
            data: data.into_iter().collect(),
          });
        }
      }

      Loop(instructions) => {
        result.push(Loop(compress_clear_runs(instructions)));
      }

      _ => result.push(current.clone()),
    }

    i += 1;
  }
  result
}

fn optimize_simple_clear_loops(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::with_capacity(source.len());
  let mut push = #[inline]
  |val| result.push(val);

  for op in source {
    match op {
      Loop(instructions) => match &instructions[..] {
        [] => {}

        [ClearRun {
          shift,
          offset,
          data,
        }] => push(SimpleClearLoop {
          shift: *shift,
          offset: *offset,
          data: data.clone(),
        }),

        _ => push(Loop(optimize_simple_clear_loops(instructions))),
      },

      _ => push(op.clone()),
    }
  }

  result
}
