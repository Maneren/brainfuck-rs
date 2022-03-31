#[derive(Debug, PartialEq, Eq)]
pub enum Op {
  Increment(u8),
  Decrement(u8),
  Right(usize),
  Left(usize),
  Print,
  Read,
  BlockStart,
  BlockStop,
  Invalid,
}

impl Op {
  fn is_valid(&self) -> bool {
    self != &Op::Invalid
  }
}
pub fn parse(string: &str) -> Vec<Op> {
  let mut parsed = Vec::new();

  let chars = string.chars().collect::<Vec<char>>();

  let mut index = 0;
  while let Some(ch) = chars.get(index) {
    let op = match ch {
      '+' => Op::Increment(load_multiple(&chars, &mut index)),
      '-' => Op::Decrement(load_multiple(&chars, &mut index)),
      '>' => Op::Right(load_multiple(&chars, &mut index) as usize),
      '<' => Op::Left(load_multiple(&chars, &mut index) as usize),
      '.' => Op::Print,
      ',' => Op::Read,
      '[' => Op::BlockStart,
      ']' => Op::BlockStop,
      _ => Op::Invalid,
    };

    if op.is_valid() {
      parsed.push(op);
    }

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
