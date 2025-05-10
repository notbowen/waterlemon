#![allow(dead_code)]

use thiserror::Error;

use crate::{
    board::Board,
    constants::{Pieces, Sides},
};

/// Parses the piece placement string (first in the FEN string) and mutates the board state
pub fn parse_piece_placement(board: &mut Board, placement: &str) -> Result<(), FenParseError> {
    let ranks = placement.split('/').collect::<Vec<&str>>();

    for (rank, rank_str) in ranks.iter().enumerate() {
        let mut file = 0;
        for piece in rank_str.chars() {
            match piece {
                // Black Pieces
                'r' => {
                    board.piece_bb[Sides::BLACK][Pieces::ROOK].set_bit(rank, file);
                    board.color_bb[Sides::BLACK].set_bit(rank, file);
                }
                'n' => {
                    board.piece_bb[Sides::BLACK][Pieces::KNIGHT].set_bit(rank, file);
                    board.color_bb[Sides::BLACK].set_bit(rank, file);
                }
                'b' => {
                    board.piece_bb[Sides::BLACK][Pieces::BISHOP].set_bit(rank, file);
                    board.color_bb[Sides::BLACK].set_bit(rank, file);
                }
                'q' => {
                    board.piece_bb[Sides::BLACK][Pieces::QUEEN].set_bit(rank, file);
                    board.color_bb[Sides::BLACK].set_bit(rank, file);
                }
                'k' => {
                    board.piece_bb[Sides::BLACK][Pieces::KING].set_bit(rank, file);
                    board.color_bb[Sides::BLACK].set_bit(rank, file);
                }
                'p' => {
                    board.piece_bb[Sides::BLACK][Pieces::PAWN].set_bit(rank, file);
                    board.color_bb[Sides::BLACK].set_bit(rank, file);
                }

                // White Pieces
                'R' => {
                    board.piece_bb[Sides::WHITE][Pieces::ROOK].set_bit(rank, file);
                    board.color_bb[Sides::WHITE].set_bit(rank, file);
                }
                'N' => {
                    board.piece_bb[Sides::WHITE][Pieces::KNIGHT].set_bit(rank, file);
                    board.color_bb[Sides::WHITE].set_bit(rank, file);
                }
                'B' => {
                    board.piece_bb[Sides::WHITE][Pieces::BISHOP].set_bit(rank, file);
                    board.color_bb[Sides::WHITE].set_bit(rank, file);
                }
                'Q' => {
                    board.piece_bb[Sides::WHITE][Pieces::QUEEN].set_bit(rank, file);
                    board.color_bb[Sides::WHITE].set_bit(rank, file);
                }
                'K' => {
                    board.piece_bb[Sides::WHITE][Pieces::KING].set_bit(rank, file);
                    board.color_bb[Sides::WHITE].set_bit(rank, file);
                }
                'P' => {
                    board.piece_bb[Sides::WHITE][Pieces::PAWN].set_bit(rank, file);
                    board.color_bb[Sides::WHITE].set_bit(rank, file);
                }

                inc => match inc.to_digit(10) {
                    Some(f) => file += f as usize,
                    None => return Err(FenParseError::InvalidPiecePlacement),
                },
            }
        }
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum FenParseError {
    #[error("Invalid FEN length")]
    InvalidLength,

    #[error("Invalid piece placement string")]
    InvalidPiecePlacement,
}
