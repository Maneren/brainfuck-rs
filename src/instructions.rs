use std::{fmt::Debug, num::Wrapping};

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
  ModifyRun {
    shift: i32,
    offset: i32,
    data: Vec<Wrapping<u8>>,
  },
  LinearLoop {
    offset: i32,
    linearity_factor: Wrapping<u8>,
    data: Vec<Wrapping<u8>>,
  },
  SimpleLoop {
    shift: i32,
    offset: i32,
    data: Vec<Wrapping<u8>>,
  },
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
      Self::ModifyRun { .. } => 12,
      Self::SimpleLoop { .. } => 13,
      Self::LinearLoop { .. } => 14,
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
      Self::ModifyRun {
        shift,
        offset,
        data,
      } => f
        .debug_struct("ModifyRun")
        .field("shift", shift)
        .field("offset", offset)
        .field("data", data)
        .finish(),
      Self::SimpleLoop {
        shift,
        offset,
        data,
      } => f
        .debug_struct("SimpleLoop")
        .field("shift", shift)
        .field("offset", offset)
        .field("data", data)
        .finish(),
      Self::LinearLoop {
        offset,
        linearity_factor,
        data,
      } => f
        .debug_struct("LinearLoop")
        .field("linearity_factor", linearity_factor)
        .field("offset", offset)
        .field("data", data)
        .finish(),
    }
  }
}
