use std::{
  fmt::{self, Debug},
  ops::{Index, IndexMut},
};

use wrapping_proc_macro::wrapping;

#[derive(Clone, PartialEq, Eq)]
pub struct Memory {
  pub data: Vec<u8>,
  pub ptr: usize,
}

impl Memory {
  #[inline]
  pub fn new(size: usize) -> Self {
    Self {
      data: vec![0; size],
      ptr: 0,
    }
  }

  #[inline]
  pub fn get(&self) -> u8 {
    self.data[self.ptr]
  }

  #[inline]
  pub fn set(&mut self, value: u8) {
    self.data[self.ptr] = value;
  }

  pub fn check_length(&mut self, length: usize) {
    if length > self.data.len() {
      self.data.resize(length, 0);
    }
  }

  #[inline]
  pub fn shift(&mut self, delta: isize) {
    wrapping! {
      self.ptr += delta as usize;
      self.check_length(self.ptr + 1);
    }
  }
}

impl Index<usize> for Memory {
  type Output = u8;

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl IndexMut<usize> for Memory {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.data[index]
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
