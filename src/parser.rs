use crate::instructions::Instruction;

pub fn parse(string: &str) -> Vec<Instruction> {
  string
    .chars()
    .filter_map(|ch| match ch {
      '+' => Some(Instruction::Increment),
      '-' => Some(Instruction::Decrement),
      '>' => Some(Instruction::Right),
      '<' => Some(Instruction::Left),
      '.' => Some(Instruction::Print),
      ',' => Some(Instruction::Read),
      '[' => Some(Instruction::BlockStart),
      ']' => Some(Instruction::BlockEnd),
      _ => None,
    })
    .collect()
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
