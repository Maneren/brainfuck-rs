#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
  Increment(u8),
  Decrement(u8),
  Right(usize),
  Left(usize),
  Print,
  Read,
  JumpIfZero(usize),
  JumpIfNonZero(usize),
}

pub fn parse(string: &str) -> Vec<Instruction> {
  let mut parsed = Vec::new();

  let chars = string.chars().collect::<Vec<char>>();

  let mut index = 0;
  while let Some(ch) = chars.get(index) {
    let op = match ch {
      '+' => Instruction::Increment(load_multiple(&chars, &mut index)),
      '-' => Instruction::Decrement(load_multiple(&chars, &mut index)),
      '>' => Instruction::Right(load_multiple(&chars, &mut index) as usize),
      '<' => Instruction::Left(load_multiple(&chars, &mut index) as usize),
      '.' => Instruction::Print,
      ',' => Instruction::Read,
      '[' => Instruction::JumpIfZero(0),
      ']' => Instruction::JumpIfNonZero(0),
      _ => {
        index += 1;
        continue;
      }
    };

    parsed.push(op);

    index += 1;
  }

  parsed
}

fn load_multiple(chars: &[char], index: &mut usize) -> u8 {
  let char = chars[*index];
  let mut count = 1;

  while let Some(&ch) = chars.get(*index + count as usize) {
    if ch == char {
      count += 1;
    } else {
      break;
    }
  }

  *index += count as usize - 1;

  count
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
