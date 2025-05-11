#![allow(dead_code)]

use std::fmt;

use crate::errors::BitboardError;

#[derive(Debug, Clone, Copy)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn new() -> Self {
        BitBoard(0)
    }

    /// Sets the bit at `rank` and `file`
    pub fn set_bit(&mut self, rank: usize, file: usize) -> Result<(), BitboardError> {
        if rank > 7 || file > 7 {
            return Err(BitboardError::InvalidRankOrFile);
        }

        let pos = rank * 8 + file;
        let bit: u64 = 1 << pos;
        self.0 |= bit;

        Ok(())
    }

    /// Gets the bit at `rank` and `file`
    /// Returns true if the bit is set, false otherwise
    pub fn get_bit(&self, rank: usize, file: usize) -> Result<bool, BitboardError> {
        if rank > 7 || file > 7 {
            return Err(BitboardError::InvalidRankOrFile);
        }
        let pos = rank * 8 + file;
        Ok((self.0 >> pos) & 1 == 1)
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_str = String::new();

        for rank in (0..8).rev() {
            for file in 0..8 {
                if self.get_bit(rank, file).unwrap() {
                    board_str.push('1');
                } else {
                    board_str.push('0');
                }
                if file < 7 {
                    board_str.push(' ');
                }
            }
            if rank > 0 {
                board_str.push('\n');
            }
        }
        write!(f, "{}", board_str)
    }
}
