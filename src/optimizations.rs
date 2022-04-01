use crate::instructions::Instruction::{
  self, BlockEnd, BlockStart, Clear, Decrement, Increment, JumpIfNonZero, JumpIfZero, Left, Right,
  ScanLeft, ScanRight,
};

pub fn link_jumps(input: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::new();
  let mut left_indexes = Vec::new();

  for (i, instruction) in input.iter().enumerate() {
    match instruction {
      BlockStart => {
        left_indexes.push(i);
        result.push(JumpIfZero(0));
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
      instruction => result.push(*instruction),
    }
  }

  assert!(left_indexes.is_empty(), "Unmatched opening bracket!");

  result
}

pub fn optimize_loops(source: &[Instruction]) -> Vec<Instruction> {
  let mut result = Vec::with_capacity(source.len());

  // optimize clear and scan loops
  let mut i = 0;
  while i < source.len() {
    match (source.get(i), source.get(i + 1), source.get(i + 2)) {
      (Some(BlockStart), Some(Increment(1) | Decrement(1)), Some(BlockEnd)) => {
        result.push(Clear);
        i += 2;
      }
      (Some(BlockStart), Some(Left(1)), Some(BlockEnd)) => {
        result.push(ScanLeft);
        i += 2;
      }
      (Some(BlockStart), Some(Right(1)), Some(BlockEnd)) => {
        result.push(ScanRight);
        i += 2;
      }
      _ => {
        result.push(source[i]);
      }
    }
    i += 1;
  }


  result
}
