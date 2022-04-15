use crate::instructions::Instruction::{
  self, BlockEnd, BlockStart, Clear, Decrement, Increment, JumpIfNonZero, JumpIfZero, Left, Right,
  ScanLeft, ScanRight,
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
          Some(left) => left,
          None => panic!("Unmatched closing bracket!"),
        };

        let right = i;

        result[left] = JumpIfZero(right);
        result.push(JumpIfNonZero(left));
      }
      instruction => result.push(instruction.clone()),
    }
  }

  assert!(left_indexes.is_empty(), "Unmatched opening bracket!");

  result
}

pub fn optimize(source: &[Instruction]) -> Vec<Instruction> {
  let mut first_stage = Vec::with_capacity(source.len());

  // optimize clear and scan loops
  let mut i = 0;
  while i < source.len() {
    match (source.get(i), source.get(i + 1), source.get(i + 2)) {
      (Some(BlockStart), Some(Increment(1) | Decrement(1)), Some(BlockEnd)) => {
        first_stage.push(Clear);
        i += 2;
      }
      (Some(BlockStart), Some(Left), Some(BlockEnd)) => {
        first_stage.push(ScanLeft);
        i += 2;
      }
      (Some(BlockStart), Some(Right), Some(BlockEnd)) => {
        first_stage.push(ScanRight);
        i += 2;
      }
      _ => {
        first_stage.push(source[i].clone());
      }
    }
    i += 1;
  }

  // optimize runs
  let source = first_stage;
  let mut result = Vec::new();

  let mut i = 0;
  while i < source.len() {
    let current = &source[i];

    match current {
      Increment(..) | Decrement(..) | Right | Left => {
        let mut memory_pointer = 0;

        let mut offset = 0;
        let mut data = vec![0; 1];

        while i < source.len() {
          match &source[i] {
            Increment(amount) => {
              data[memory_pointer] += i64::from(*amount);
            }
            Decrement(amount) => data[memory_pointer] += -i64::from(*amount),
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

        let shift = memory_pointer as i64 + offset as i64;

        // remove unused data
        while let Some(0) = data.last() {
          data.pop();
        }

        while let Some(0) = data.get(0) {
          offset += 1;
          data.remove(0);
        }

        result.push(Instruction::ModifyRun {
          shift,
          offset,
          data,
        });
      }
      _ => result.push(current.clone()),
    }

    i += 1;
  }

  result
}
