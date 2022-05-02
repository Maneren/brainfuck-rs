use std::{
  fmt::{self, Debug},
  num::Wrapping,
  ops::{Index, IndexMut},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Memory {
  pub data: Vec<Wrapping<u8>>,
  pub ptr: Wrapping<usize>,
}

impl Memory {
  pub fn new(size: usize) -> Self {
    Self {
      data: vec![Wrapping(0); size],
      ptr: Wrapping(0),
    }
  }

  pub fn get(&self) -> u8 {
    self.data[self.ptr.0].0
  }

  pub fn set(&mut self, value: u8) {
    self.data[self.ptr.0] = Wrapping(value);
  }

  pub fn check_length(&mut self, length: usize) {
    if length > self.data.len() {
      self.data.resize(length, Wrapping(0));
    }
  }

  pub fn shift(&mut self, delta: i32) {
    self.ptr += delta as usize;

    self.check_length(self.ptr.0 + 1);
  }
}

impl Index<usize> for Memory {
  type Output = Wrapping<u8>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

impl IndexMut<usize> for Memory {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.data[index]
  }
}

impl Index<Wrapping<usize>> for Memory {
  type Output = Wrapping<u8>;

  fn index(&self, index: Wrapping<usize>) -> &Self::Output {
    &self.data[index.0]
  }
}

impl IndexMut<Wrapping<usize>> for Memory {
  fn index_mut(&mut self, index: Wrapping<usize>) -> &mut Self::Output {
    &mut self.data[index.0]
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
