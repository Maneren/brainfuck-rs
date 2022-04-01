
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
