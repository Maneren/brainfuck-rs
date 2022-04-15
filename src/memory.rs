use std::fmt::{self, Debug};

#[derive(Clone)]
pub struct Memory {
  pub data: Vec<u8>,
  ptr: usize,
}

impl Memory {
  pub fn new(size: usize) -> Self {
    Self {
      data: vec![0; size],
      ptr: 0,
    }
  }

  pub fn get(&self, offset: i64) -> u8 {
    let index = self.ptr as i64 + offset;
    let index = self.normalize_index(index);

    self.data[index]
  }

  pub fn set(&mut self, offset: i64, value: u8) {
    let index = self.ptr as i64 + offset;
    let index = self.normalize_index(index);

    self.data[index] = value;
  }

  #[inline]
  fn normalize_index(&self, mut index: i64) -> usize {
    if index < 0 {
      index += self.data.len() as i64;
    }

    let mut index = index as usize;

    if index >= self.data.len() {
      index -= self.data.len();
    }

    index
  }

  #[inline]
  pub fn modify_run(&mut self, data: &[i64], offset: i64, shift: i64) {
    let mut offset = offset;

    for value in data {
      self.modify(*value, offset);
      offset += 1;
    }

    self.shift(shift);
  }

  #[inline]
  pub fn modify(&mut self, value: i64, offset: i64) {
    let new_value = if value >= 0 {
      self.get(offset).wrapping_add(value as u8)
    } else {
      self.get(offset).wrapping_sub(value.abs() as u8)
    };
    self.set(offset, new_value);
  }

  #[inline]
  pub fn shift(&mut self, delta: i64) {
    if delta >= 0 {
      self.right(delta as usize);
    } else {
      self.left(-delta as usize);
    }
  }

  fn right(&mut self, count: usize) {
    self.ptr = self.ptr.wrapping_add(count);
    let len = self.data.len();

    if self.ptr >= len {
      self.ptr -= len;
    }
  }

  fn left(&mut self, count: usize) {
    if count > self.ptr {
      let count = count - self.ptr;
      self.ptr = self.data.len() - count;
    } else {
      self.ptr -= count;
    }
  }

  pub fn scan_right(&mut self) {
    while self.get(0) != 0 {
      self.right(1);
    }
  }

  pub fn scan_left(&mut self) {
    while self.get(0) != 0 {
      self.left(1);
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

#[cfg(test)]
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
}
