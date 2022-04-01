#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
  Increment(u8),
  Decrement(u8),
  Right(usize),
  Left(usize),
  Print,
  Read,
  BlockStart,
  BlockEnd,
  JumpIfZero(usize),
  JumpIfNonZero(usize),
  Clear,
  ScanLeft,
  ScanRight,
}
