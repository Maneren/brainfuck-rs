#[derive(Debug, Clone)]
pub struct Run {
  pub shift: i32,
  pub offset: i32,
  pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
  Increment,
  Decrement,
  Right,
  Left,
  Print,
  Read,
  BlockStart,
  BlockEnd,

  Clear,
  Shift(i32),
  JumpIfZero(usize),
  JumpIfNonZero(usize),
  ModifyRun(Run),
}
