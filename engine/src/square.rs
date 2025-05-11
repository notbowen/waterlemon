#![allow(dead_code)]

use crate::errors::SquareError;

#[repr(u8)]
#[rustfmt::skip]
#[derive(Clone, Debug)]
pub enum Square {
    None,
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    /// Returns the square at `rank` and `file`. Both are 0-indexed
    pub fn from_coords(rank: u8, file: u8) -> Result<Self, SquareError> {
        if rank > 7 || file > 7 {
            return Err(SquareError::InvalidFileRank);
        }

        let position = (rank * 8) + file + 1;

        unsafe { Ok(std::mem::transmute::<u8, Square>(position)) }
    }

    /// Returns the square representation of the square string
    pub fn from_square(square: &str) -> Result<Self, SquareError> {
        if square.len() != 2 {
            return Err(SquareError::InvalidSquare);
        }

        let rank_char = square.chars().nth(0).ok_or(SquareError::InvalidSquare)?;
        let file_char = square.chars().nth(1).ok_or(SquareError::InvalidSquare)?;

        let rank = match rank_char.to_ascii_uppercase() {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => return Err(SquareError::InvalidSquare),
        };

        let file = match file_char {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Err(SquareError::InvalidSquare),
        };

        Self::from_coords(rank, file)
    }
}
