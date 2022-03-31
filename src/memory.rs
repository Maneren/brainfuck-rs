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

  pub fn increment(&mut self) {
    self.data[self.ptr] += 1;
  }

  pub fn decrement(&mut self) {
    self.data[self.ptr] -= 1;
  }

  pub fn right(&mut self) {
    self.ptr = self.ptr.wrapping_add(1);

    if self.dynamic {
      if self.ptr >= self.data.len() {
        self.data.push(0);
      }
    } else {
      self.ptr %= self.data.len();
    }
  }

  pub fn left(&mut self) {
    if self.ptr == 0 {
      self.ptr = self.data.len() - 1;
    } else {
      self.ptr -= 1;
    }
  }
}
