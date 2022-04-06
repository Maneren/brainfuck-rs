#[derive(Debug, Clone)]
pub struct Memory {
  data: Vec<u8>,
  ptr: usize,
  dynamic: bool,
}

impl Memory {
  pub fn new(size: usize, dynamic: bool) -> Self {
    Self {
      data: vec![0; size],
      ptr: 0,
      dynamic,
    }
  }

  pub fn get(&self) -> u8 {
    self.data[self.ptr]
  }

  pub fn set(&mut self, offset: i64, value: u8) {
    let index = (self.ptr as i64 + offset) as usize;

    assert!(index < self.data.len(), "Invalid memory access!");

    self.data[index] = value;
  }

  pub fn modify(&mut self, data: &[i64], offset: i64, shift: i64) {
    for (i, value) in data.iter().enumerate() {
      let index = self.ptr as i64 + offset + i as i64;

      assert!(
        index < self.data.len() as i64 && index >= 0,
        "Invalid memory access!"
      );

      let index = index as usize;

      if *value >= 0 {
        self.data[index] = self.data[index].wrapping_add(*value as u8);
      } else {
        self.data[index] = self.data[index].wrapping_sub(value.abs() as u8);
      }
    }

    self.shift(shift);
  }

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

    if self.dynamic {
      if self.ptr >= len {
        self.data.resize(self.ptr + 1, 0);
      }
    } else if self.ptr >= len {
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
    while self.get() != 0 {
      self.right(1);
    }
  }

  pub fn scan_left(&mut self) {
    while self.get() != 0 {
      self.left(1);
    }
  }

  pub fn dynamic(&self) -> bool {
    self.dynamic
  }

  pub fn size(&self) -> usize {
    self.data.len()
  }
}
