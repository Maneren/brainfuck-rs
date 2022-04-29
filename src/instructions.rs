use std::{fmt::Debug, num::Wrapping};

#[derive(Debug, Clone)]
pub struct Run {
  pub shift: i32,
  pub offset: i32,
  pub data: Vec<Wrapping<u8>>,
}

#[derive(Clone)]
pub struct Loop {
  pub data: Vec<Wrapping<u8>>,
  pub linear_factor: Wrapping<u8>,
}

impl Debug for Loop {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Loop")
      .field("data", &self.data)
      .field("linear_factor", &self.linear_factor)
      .finish()
  }
}

#[derive(Clone)]
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
  LinearLoop(Loop),
}

impl Instruction {
  pub fn index(&self) -> usize {
    match self {
      Self::Increment => 0,
      Self::Decrement => 1,
      Self::Right => 2,
      Self::Left => 3,
      Self::Print => 4,
      Self::Read => 5,
      Self::BlockStart => 6,
      Self::BlockEnd => 7,
      Self::Clear => 8,
      Self::Shift(_) => 9,
      Self::JumpIfZero(_) => 10,
      Self::JumpIfNonZero(_) => 11,
      Self::ModifyRun(_) => 12,
      Self::LinearLoop(_) => 13,
    }
  }
}

impl Debug for Instruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Increment => write!(f, "Increment"),
      Self::Decrement => write!(f, "Decrement"),
      Self::Right => write!(f, "Right"),
      Self::Left => write!(f, "Left"),
      Self::Print => write!(f, "Print"),
      Self::Read => write!(f, "Read"),
      Self::BlockStart => write!(f, "BlockStart"),
      Self::BlockEnd => write!(f, "BlockEnd"),
      Self::Clear => write!(f, "Clear"),
      Self::Shift(amount) => write!(f, "Shift({amount})"),
      Self::JumpIfZero(amount) => write!(f, "JumpIfZero({amount})"),
      Self::JumpIfNonZero(amount) => write!(f, "JumpIfNonZero({amount})"),
      Self::ModifyRun(run) => write!(f, "{run:?}"),
      Self::LinearLoop(r#loop) => write!(f, "{loop:?}"),
    }
  }
}
