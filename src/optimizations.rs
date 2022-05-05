use std::{collections::VecDeque, num::Wrapping};

use crate::instructions::Instruction::{
  self, BlockEnd, BlockStart, Decrement, Increment, Left, LinearLoop, Loop, Modify, ModifyOffset,
  ModifyRun, Right, SearchLoop, Set, SetOffset, Shift, SimpleLoop,
};

pub fn optimize(instructions: &[Instruction]) -> Vec<Instruction> {
  dbg!(instructions.len());
  let instructions = &compress_runs(instructions);
  dbg!(instructions.len());
  let instructions = &optimize_small_loops(instructions);
  dbg!(instructions.len());
  let instructions = &constant_folding(instructions);
  dbg!(instructions.len());
  let instructions = &remove_dead(instructions);
  dbg!(instructions.len());
  collect_loops(instructions)
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
            result.push(Loop {
              instructions: collect_loops(&buffer),
            });
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

fn optimize_small_loops(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::with_capacity(source.len());
  let mut i = 0;

  while i < source.len() {
    match (source.get(i), source.get(i + 1), source.get(i + 2)) {
      (Some(BlockStart), Some(Modify(..)), Some(BlockEnd)) => {
        result.push(Set(Wrapping(0)));

        i += 2;
      }
      (Some(BlockStart), Some(Shift(shift)), Some(BlockEnd)) => {
        result.push(SearchLoop { step: *shift });

        i += 2;
      }
      (
        Some(BlockStart),
        Some(ModifyRun {
          shift,
          offset,
          data,
        }),
        Some(BlockEnd),
      ) => {
        if *shift == 0 && *offset <= 0 {
          let linearity_factor = -data[(-offset) as usize]; // value at offset 0
          result.push(LinearLoop {
            offset: *offset,
            linearity_factor,
            data: data.clone(),
          });
        } else {
          result.push(SimpleLoop {
            shift: *shift,
            offset: *offset,
            data: data.clone(),
          });
        }

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

fn constant_folding(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::with_capacity(source.len());
  let mut i = 0;
  while i < source.len() {
    match (source.get(i), source.get(i + 1), source.get(i + 2)) {
      (
        Some(Shift(a)),
        Some(Set(..) | Modify(..) | ModifyOffset(..) | SetOffset(..)),
        Some(Shift(b)),
      ) => {
        let op = match source.get(i + 1).unwrap() {
          Set(value) => SetOffset(*value, *a),
          Modify(value) => ModifyOffset(*value, *a),

          SetOffset(value, offset) => SetOffset(*value, offset + a),
          ModifyOffset(value, offset) => ModifyOffset(*value, offset + a),
          _ => unreachable!(),
        };

        result.push(op);

        result.push(Shift(a + b));

        i += 2;
      }

      /* (
        Some(Shift(a)),
        Some(LinearLoop {
          offset,
          linearity_factor,
          data,
        }),
        Some(Shift(b)),
      ) => {
        dbg!((source.get(i), source.get(i + 1), source.get(i + 2)));
        dbg!((
          LinearLoop {
            offset: offset + a,
            linearity_factor: *linearity_factor,
            data: data.clone(),
          },
          Shift(a + b)
        ));

        result.push(LinearLoop {
          offset: offset + a,
          linearity_factor: *linearity_factor,
          data: data.clone(),
        });

        result.push(Shift(a + b));

        i += 2;
      } */
      _ => {
        result.push(source[i].clone());
      }
    }
    i += 1;
  }

  let source = result;
  let mut result = Vec::new();
  let mut i = 0;

  while i < source.len() {
    let op = match (source.get(i), source.get(i + 1)) {
      (Some(Set(a)), Some(Modify(b))) => {
        i += 1;

        Set(a + b)
      }
      (Some(Set(..)), Some(Set(b))) => {
        i += 1;

        Set(*b)
      }
      (Some(Modify(a)), Some(Modify(b))) => {
        i += 1;

        Modify(a + b)
      }
      (Some(Shift(a)), Some(Shift(b))) => {
        i += 1;
        Shift(a + b)
      }

      _ => source[i].clone(),
    };

    result.push(op);

    i += 1;
  }

  result
}

fn remove_dead(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::with_capacity(source.len());
  let mut i = 0;

  while i < source.len() {
    match source.get(i) {
      Some(Modify(Wrapping(0)) | ModifyOffset(Wrapping(0), ..) | Shift(0)) => {}
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
        } else if data.len() == 1 {
          if offset == 0 {
            result.push(Modify(data[0]));
          } else {
            result.push(ModifyOffset(data[0], offset));
          }

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
      _ => result.push(current.clone()),
    }

    i += 1;
  }
  result
}
