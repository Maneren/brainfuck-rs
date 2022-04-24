#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModifyRunData {
  pub shift: i32,
  pub offset: i32,
  pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
  ModifyRun(ModifyRunData),
}
