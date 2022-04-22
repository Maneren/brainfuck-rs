#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModifyRunData {
  pub shift: i32,
  pub offset: i32,
  pub data: Vec<i32>,
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
  BlockStartWithData(ModifyRunData),
  BlockEndWithData(ModifyRunData),
  JumpIfZero(usize),
  JumpIfNonZero(usize),
  JumpIfZeroWithData(usize, ModifyRunData),
  JumpIfNonZeroWithData(usize, ModifyRunData),
  ModifyRun(ModifyRunData),
}
