use crate::instructions::{
  Instruction::{
    self, BlockEnd, BlockEndWithData, BlockStart, BlockStartWithData, Clear, Decrement, Increment,
    JumpIfNonZero, JumpIfNonZeroWithData, JumpIfZero, JumpIfZeroWithData, Left, ModifyRun, Right,
  },
  ModifyRunData,
};

pub fn link_jumps(input: &[Instruction]) -> Vec<Instruction> {
  dbg!(input);
  let mut result = Vec::with_capacity(input.len());
  let mut left_indexes = Vec::new();
  let mut left_indexes_with_data = Vec::new();

  for (i, instruction) in input.iter().enumerate() {
    match instruction {
      BlockStartWithData(data) => {
        left_indexes_with_data.push((i, data));
        result.push(BlockStart);
      }

      BlockEndWithData(data) => {
        let (left, data_left) = match left_indexes_with_data.pop() {
          Some(val) => val,
          None => panic!("Unmatched closing bracket!"),
        };

        let right = i;

        result[left] = JumpIfZeroWithData(right, data_left.clone());
        result.push(JumpIfNonZeroWithData(left, data.clone()));
      }
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
  let first_stage = replace_clear_loops(source);
  let second_stage = compress_runs(&first_stage);
  merge_block_and_modify(&second_stage)
}

fn replace_clear_loops(source: &[Instruction]) -> Vec<Instruction> {
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
  while i < source.len() {
    let current = &source[i];

    match current {
      Increment | Decrement | Right | Left => {
        let mut memory_pointer = 0;

        let mut offset = 0;
        let mut data = vec![0; 1];

        while i < source.len() {
          match &source[i] {
            Increment => data[memory_pointer] += 1,
            Decrement => data[memory_pointer] -= 1,
            Right => {
              memory_pointer += 1;
              if memory_pointer >= data.len() {
                data.push(0);
              }
            }
            Left => {
              if memory_pointer > 0 {
                memory_pointer -= 1;
              } else {
                offset -= 1;

                data.insert(0, 0);
              }
            }
            _ => {
              i -= 1;
              break;
            }
          }

          i += 1;
        }

        let shift = memory_pointer as i32 + offset;

        // remove unused data
        while let Some(0) = data.last() {
          data.pop();
        }

        while let Some(0) = data.get(0) {
          offset += 1;
          data.remove(0);
        }

        if data.is_empty() {
          if shift != 0 {
            result.push(Instruction::Shift(shift));
          }
        } else {
          result.push(Instruction::ModifyRun(ModifyRunData {
            shift,
            offset,
            data,
          }));
        }
      }
      _ => result.push(current.clone()),
    }

    i += 1;
  }
  result
}

fn merge_block_and_modify(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::new();
  let mut i = 0;
  while i < source.len() {
    let current = &source[i];

    match current {
      BlockStart | BlockEnd => {
        let data = if let Some(ModifyRun(data)) = source.get(i + 1) {
          i += 1;
          data.clone()
        } else {
          ModifyRunData {
            shift: 0,
            offset: 0,
            data: vec![],
          }
        };

        let op = match current {
          BlockStart => BlockStartWithData(data),
          BlockEnd => BlockEndWithData(data),
          _ => unreachable!(),
        };

        result.push(op);
      }
      _ => result.push(current.clone()),
    }

    i += 1;
  }
  result
}
