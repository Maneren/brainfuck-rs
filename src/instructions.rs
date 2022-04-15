#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
  Increment(u8),
  Decrement(u8),
  Right,
  Left,
  Shift(i64),
  Modify(i64),
  Print,
  Read,
  BlockStart,
  BlockEnd,
  JumpIfZero(usize),
  JumpIfNonZero(usize),
  ModifyRun {
    shift: i64,
    offset: i64,
    data: Vec<i64>,
  },
  Clear,
  ScanLeft,
  ScanRight,
}
