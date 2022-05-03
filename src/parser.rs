use crate::instructions::Instruction;

pub fn parse(string: &str) -> Vec<Instruction> {
  let mut parsed = Vec::new();

  for ch in string.chars() {
    let op = match ch {
      '+' => Instruction::Increment,
      '-' => Instruction::Decrement,
      '>' => Instruction::Right,
      '<' => Instruction::Left,
      '.' => Instruction::Print,
      ',' => Instruction::Read,
      '[' => Instruction::BlockStart,
      ']' => Instruction::BlockEnd,
      _ => {
        continue;
      }
    };

    parsed.push(op);
  }

  parsed
}

#[cfg(test)]
mod tests {
  use super::Instruction::{BlockEnd, BlockStart, Decrement, Increment, Left, Print, Right};

  #[test]
  fn parse_simple() {
    let program = "+[------->++<]>--.+++.---.";
    let parsed = super::parse(program);

    assert_eq!(
      parsed,
      &[
        Increment, BlockStart, Decrement, Decrement, Decrement, Decrement, Decrement, Decrement,
        Decrement, Right, Increment, Increment, Left, BlockEnd, Right, Decrement, Decrement, Print,
        Increment, Increment, Increment, Print, Decrement, Decrement, Decrement, Print,
      ]
    );
  }
}
