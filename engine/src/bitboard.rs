#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// Sets the bit at `rank` and `file`
    pub fn set_bit(mut self, rank: usize, file: usize) {
        let pos = rank * 8 + file;
        let bit: u64 = 1 << pos;
        self.0 |= bit;
    }
}
