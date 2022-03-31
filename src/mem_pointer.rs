use std::ops::{Deref, AddAssign, SubAssign};


#[derive(Debug, Clone, Copy)]
pub struct MemPointer {
    data: usize,
    max: usize
}

impl MemPointer {
    pub fn new(max: usize) -> Self {
        MemPointer { data: 0, max }
    }
}

impl AddAssign<usize> for MemPointer {
    fn add_assign(&mut self, rhs: usize) {
        self.data = self.data.wrapping_add(rhs) % self.max;
    }
}

impl SubAssign<usize> for MemPointer {
    fn sub_assign(&mut self, rhs: usize) {
        self.data = self.data.wrapping_sub(rhs);
    }
}

impl Deref for MemPointer {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

