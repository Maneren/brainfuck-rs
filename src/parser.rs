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
  #[test]
  fn parse_simple() {
    let program = "+[------->++<]>--.+++.---.";
    let parsed = super::parse(program);

    println!("{:?}", parsed);
  }
}
