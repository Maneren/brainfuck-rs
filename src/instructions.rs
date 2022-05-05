use std::{fmt::Debug, num::Wrapping};

#[derive(Clone, PartialEq, Eq)]
pub enum Instruction {
  Increment,
  Decrement,
  Right,
  Left,
  Print,
  Read,
  BlockStart,
  BlockEnd,

  Set(Wrapping<u8>),
  SetOffset(Wrapping<u8>, isize),
  Shift(isize),
  Modify(Wrapping<u8>),
  ModifyOffset(Wrapping<u8>, isize),
  ModifyRun {
    shift: isize,
    offset: isize,
    data: Vec<Wrapping<u8>>,
  },
  LinearLoop {
    offset: isize,
    linearity_factor: Wrapping<u8>,
    data: Vec<Wrapping<u8>>,
  },
  SimpleLoop {
    shift: isize,
    offset: isize,
    data: Vec<Wrapping<u8>>,
  },
  SearchLoop {
    step: isize,
  },
  Loop {
    instructions: Vec<Instruction>,
  },
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
      Self::Set(amount) => write!(f, "Set({amount})"),
      Self::SetOffset(amount, offset) => write!(f, "SetOffset({amount}, {offset})"),
      Self::Shift(amount) => write!(f, "Shift({amount})"),
      Self::Modify(amount) => write!(f, "Modify({amount})"),
      Self::ModifyOffset(amount, offset) => write!(f, "ModifyOffset({amount}, {offset})"),
      Self::ModifyRun {
        shift,
        offset,
        data,
      } => f
        .debug_struct("ModifyRun")
        .field("shift", shift)
        .field("offset", offset)
        .field("data", &format!("{data:?}"))
        .finish(),
      Self::SimpleLoop {
        shift,
        offset,
        data,
      } => f
        .debug_struct("SimpleLoop")
        .field("shift", shift)
        .field("offset", offset)
        .field("data", &format!("{data:?}"))
        .finish(),
      Self::SearchLoop { step } => f.debug_struct("SearchLoop").field("step", step).finish(),
      Self::LinearLoop {
        offset,
        linearity_factor,
        data,
      } => f
        .debug_struct("LinearLoop")
        .field("linearity_factor", linearity_factor)
        .field("offset", offset)
        .field("data", &format!("{data:?}"))
        .finish(),
      Self::Loop { instructions } => f.debug_list().entries(instructions).finish(),
    }
  }
}
