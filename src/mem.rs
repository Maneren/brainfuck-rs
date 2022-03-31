
use std::ops::{Index, IndexMut};

type MemoryData = [u8; u16::MAX as usize];

#[derive(Debug)]
pub struct Memory {
    data: MemoryData
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; u16::MAX as usize] }
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output  {
        &self.data[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output  {
        &mut self.data[index as usize]
    }
}
