use std::{
  fmt::{self, Debug},
  num::Wrapping,
};

use crate::instructions::ModifyRunData;

#[derive(Clone)]
pub struct Memory {
  pub data: Vec<Wrapping<u8>>,
  ptr: i32,
}

impl Memory {
  pub fn new(size: usize) -> Self {
    Self {
      data: vec![Wrapping(0); size],
      ptr: 0,
    }
  }

  #[inline]
  pub fn get(&mut self) -> Wrapping<u8> {
    self.data[self.ptr as usize]
  }

  pub fn set(&mut self, value: u8) {
    let index = self.ptr as usize;

    if index >= self.data.len() {
      self.data.resize(index + 1, Wrapping(0));
    }

    self.data[index] = Wrapping(value);
  }

  pub fn modify_run(&mut self, data: &ModifyRunData) {
    let ModifyRunData {
      shift,
      offset,
      data,
    } = data;

    let ptr = (self.ptr + offset) as usize;

    let resulting_len = ptr + data.len();
    if resulting_len > self.data.len() {
      self.data.resize(resulting_len, Wrapping(0));
    }

    for (i, value) in data.iter().enumerate() {
      self.data[ptr + i] += value;
    }

    self.shift(*shift);
  }

  pub fn shift(&mut self, delta: i32) {
    if delta > 0 {
      let resulting_len = (self.ptr as i32 + delta + 1) as usize;

      if resulting_len > self.data.len() {
        self.data.resize(resulting_len, Wrapping(0));
      }
    }

    self.ptr += delta;
  }

  pub fn size(&self) -> usize {
    self.data.len()
  }
}

impl Debug for Memory {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let mem = self
      .data
      .iter()
      .map(|x| format!("{x}"))
      .collect::<Vec<_>>()
      .join(",");

    writeln!(f, "ptr: {}, data: [{mem}]", self.ptr)
  }
}

/* #[cfg(test)]
mod tests {
  use super::Memory;

  #[test]
  pub fn shift() {
    let mut mem = Memory::new(4);
    mem.shift(2);
    assert_eq!(mem.ptr, 2);

    let mut mem = Memory::new(4);
    mem.shift(6);
    assert_eq!(mem.ptr, 2);

    let mut mem = Memory::new(4);
    mem.shift(-2);
    assert_eq!(mem.ptr, 2);
  }

  #[test]
  pub fn modify() {
    let mut mem = Memory::new(4);
    mem.modify_run(&[1, 2, 3, 4], 0, 0);
    assert_eq!(mem.data, vec![1, 2, 3, 4]);
    assert_eq!(mem.ptr, 0);

    let mut mem = Memory::new(4);
    mem.modify_run(&[3, 4], 2, 0);
    assert_eq!(mem.data, vec![0, 0, 3, 4]);
    assert_eq!(mem.ptr, 0);

    let mut mem = Memory::new(4);
    mem.modify_run(&[1, 2, 3, 4], 0, 4);
    assert_eq!(mem.data, vec![1, 2, 3, 4]);
    assert_eq!(mem.ptr, 0);

    let mut mem = Memory::new(4);
    mem.shift(2);
    mem.modify_run(&[1, 2], -2, 2);
    assert_eq!(mem.data, vec![1, 2, 0, 0]);
    assert_eq!(mem.ptr, 0);

    let mut mem = Memory::new(4);
    mem.shift(2);
    mem.modify_run(&[1, 2, 3], 0, 2);
    assert_eq!(mem.data, vec![3, 0, 1, 2]);
    assert_eq!(mem.ptr, 0);

    let mut mem = Memory::new(4);
    mem.shift(2);
    mem.modify_run(&[1, 2, 3], -3, 2);
    assert_eq!(mem.data, vec![2, 3, 0, 1]);
    assert_eq!(mem.ptr, 0);
  }
} */
