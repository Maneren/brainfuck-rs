use std::{
  fmt::{self, Debug},
  num::Wrapping,
};

use crate::instructions::{Loop, Run};

pub struct Memory {
  data: Vec<Wrapping<u8>>,
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

  fn check_length(&mut self, length: usize) {
    if length > self.data.len() {
      self.data.resize(length, Wrapping(0));
    }
  }

  pub fn modify_run(&mut self, data: &Run) {
    let Run {
      shift,
      offset,
      data,
    } = data;

    let ptr = (self.ptr + Wrapping(*offset as usize)).0;

    self.check_length(ptr + data.len());

    data.iter().enumerate().for_each(|(i, value)| {
      self.data[ptr + i] += value;
    });

    self.shift(*shift);
  }

  pub fn apply_linear_loop(&mut self, data: &Loop) {
    let Loop {
      data,
      linear_factor,
    } = data;

    let ptr = self.ptr.0;

    self.check_length(ptr + data.len());

    let factor = {
      let mut tmp = Wrapping(self.get());
      let mut i = Wrapping(0);

      while tmp.0 != 0 {
        tmp -= linear_factor;
        i += 1;
      }

      i
    };

    data
      .iter()
      .map(|value| value * factor)
      .enumerate()
      .for_each(|(i, value)| {
        self.data[ptr + i] += value;
      });
  }

  pub fn shift(&mut self, delta: i32) {
    self.ptr += delta as usize;

    self.check_length(self.ptr.0 + 1);
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
