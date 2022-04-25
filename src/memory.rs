use std::{
  fmt::{self, Debug},
  num::Wrapping,
};

use crate::instructions::Run;

pub struct Memory {
  pub data: Vec<Wrapping<u8>>,
  ptr: Wrapping<usize>,
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

  pub fn modify_run(&mut self, data: &Run) {
    let Run {
      shift,
      offset,
      data,
    } = data;

    let ptr = (self.ptr + offset).0;

    let resulting_len = ptr + data.len();
    if resulting_len > self.data.len() {
      self.data.resize(resulting_len, Wrapping(0));
    }

    for (i, value) in data.iter().enumerate() {
      self.data[ptr + i] += value;
    }

    self.shift(*shift);
  }

  pub fn shift(&mut self, delta: Wrapping<usize>) {
    self.ptr += delta;

    let current_len = self.ptr.0 + 1;
    if current_len > self.data.len() {
      self.data.resize(current_len, Wrapping(0));
    }
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
