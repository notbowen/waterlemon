#![allow(dead_code)]

use crate::{
    bitboard::BitBoard, castling::Castling, errors::FenParseError, fen, piece::Color,
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
    pub fullmove_clock: u16,
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
            fullmove_clock: 1,
        };

        fen::parse_piece_placement(&mut board, split_fen[0])?;
        fen::parse_playing_side(&mut board, split_fen[1])?;
        fen::parse_castling_rights(&mut board, split_fen[2])?;
        fen::parse_en_passant(&mut board, split_fen[3])?;
        fen::parse_move_clocks(&mut board, split_fen[4], split_fen[5])?;

        Ok(board)
    }
}

#[cfg(test)]
mod board_test {
    use super::*;

    #[test]
    fn test_default_fen() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(board.is_ok(), true);

        let board = board.unwrap();
        println!("{:#?}", board);
    }
}
