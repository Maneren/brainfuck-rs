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

  pub fn set(&mut self, value: u8) {
    self.data[self.ptr] = value;
  }

  pub fn increment(&mut self, count: u8) {
    self.data[self.ptr] = self.data[self.ptr].wrapping_add(count);
  }

  pub fn decrement(&mut self, count: u8) {
    self.data[self.ptr] = self.data[self.ptr].wrapping_sub(count);
  }

  pub fn right(&mut self, count: usize) {
    self.ptr = self.ptr.wrapping_add(count);

    if self.dynamic {
      if self.ptr >= self.data.len() {
        self.data.push(0);
      }
    } else {
      self.ptr %= self.data.len();
    }
  }

  pub fn left(&mut self, count: usize) {
    if count > self.ptr {
      let count = count - self.ptr;
      self.ptr = self.data.len() - count;
    } else {
      self.ptr -= count;
    }
  }
}