use std::num::Wrapping;

#[derive(Debug, Clone)]
pub struct Run {
  pub shift: Wrapping<usize>,
  pub offset: Wrapping<usize>,
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
  Shift(Wrapping<usize>),
  JumpIfZero(usize),
  JumpIfNonZero(usize),
  ModifyRun(Run),
}
