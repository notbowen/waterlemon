#![allow(dead_code)]

use crate::{
    bitboard::BitBoard,
    castling::Castling,
    fen::{self, FenParseError},
    piece::Color,
    square::Square,
};

#[derive(Clone, Debug)]
pub struct Board {
    pub piece_bb: [[BitBoard; 6]; 2],
    pub color_bb: [BitBoard; 2],
    pub occupied_bb: BitBoard,

    pub side_to_move: Color,
    pub castling_rights: Castling,
    pub en_passant_square: Option<Square>,

    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, FenParseError> {
        let split_fen = fen.split(' ').collect::<Vec<&str>>();

        if split_fen.len() != 6 {
            return Err(FenParseError::InvalidLength);
        }

        let mut board = Board {
            piece_bb: [[BitBoard(0); 6]; 2],
            color_bb: [BitBoard(0); 2],
            occupied_bb: BitBoard(0),
            side_to_move: Color::White,
            castling_rights: Castling::none(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 0,
        };

        fen::parse_piece_placement(&mut board, split_fen[0])?;

        Ok(board)
    }
}

#[cfg(test)]
mod board_test {
    use super::*;

    #[test]
    fn test_default_fen() {
        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").is_ok(),
            true
        );
    }
}
