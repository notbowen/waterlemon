#![allow(dead_code)]

use crate::{
    board::Board,
    castling::Castling,
    errors::FenParseError,
    objects::{Pieces, Sides},
    square::Square,
};

/// Parses the piece placement string (first in the FEN string) and mutates the board state
pub fn parse_piece_placement(board: &mut Board, placement: &str) -> Result<(), FenParseError> {
    let ranks = placement.split('/').collect::<Vec<&str>>();

    for (rank, rank_str) in ranks.iter().enumerate() {
        let rank = rank as u8;
        let mut file = 0;

        for piece in rank_str.chars() {
            match piece {
                // Black Pieces
                'r' => {
                    board.piece_bb[Sides::BLACK][Pieces::ROOK].set_bit(rank, file)?;
                    board.color_bb[Sides::BLACK].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'n' => {
                    board.piece_bb[Sides::BLACK][Pieces::KNIGHT].set_bit(rank, file)?;
                    board.color_bb[Sides::BLACK].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'b' => {
                    board.piece_bb[Sides::BLACK][Pieces::BISHOP].set_bit(rank, file)?;
                    board.color_bb[Sides::BLACK].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'q' => {
                    board.piece_bb[Sides::BLACK][Pieces::QUEEN].set_bit(rank, file)?;
                    board.color_bb[Sides::BLACK].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'k' => {
                    board.piece_bb[Sides::BLACK][Pieces::KING].set_bit(rank, file)?;
                    board.color_bb[Sides::BLACK].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'p' => {
                    board.piece_bb[Sides::BLACK][Pieces::PAWN].set_bit(rank, file)?;
                    board.color_bb[Sides::BLACK].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }

                // White Pieces
                'R' => {
                    board.piece_bb[Sides::WHITE][Pieces::ROOK].set_bit(rank, file)?;
                    board.color_bb[Sides::WHITE].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'N' => {
                    board.piece_bb[Sides::WHITE][Pieces::KNIGHT].set_bit(rank, file)?;
                    board.color_bb[Sides::WHITE].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'B' => {
                    board.piece_bb[Sides::WHITE][Pieces::BISHOP].set_bit(rank, file)?;
                    board.color_bb[Sides::WHITE].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'Q' => {
                    board.piece_bb[Sides::WHITE][Pieces::QUEEN].set_bit(rank, file)?;
                    board.color_bb[Sides::WHITE].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'K' => {
                    board.piece_bb[Sides::WHITE][Pieces::KING].set_bit(rank, file)?;
                    board.color_bb[Sides::WHITE].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }
                'P' => {
                    board.piece_bb[Sides::WHITE][Pieces::PAWN].set_bit(rank, file)?;
                    board.color_bb[Sides::WHITE].set_bit(rank, file)?;
                    board.occupied_bb.set_bit(rank, file)?;
                }

                inc => match inc.to_digit(10) {
                    Some(f) => file += (f as u8) - 1,
                    None => return Err(FenParseError::InvalidPiecePlacement),
                },
            }

            file += 1;
        }
    }

    Ok(())
}

/// Sets the side that's playing (second in FEN string)
pub fn parse_playing_side(board: &mut Board, side: &str) -> Result<(), FenParseError> {
    match side {
        "w" => board.side_to_move = Sides::WHITE,
        "b" => board.side_to_move = Sides::BLACK,
        _ => return Err(FenParseError::InvalidSide),
    }

    Ok(())
}

/// Sets the castling rights (third in FEN string)
pub fn parse_castling_rights(
    board: &mut Board,
    castling_rights: &str,
) -> Result<(), FenParseError> {
    let mut rights = Castling::none();

    for c in castling_rights.chars() {
        match c {
            '-' => {}
            'K' => rights.add_white_kingside(),
            'Q' => rights.add_white_queenside(),
            'k' => rights.add_black_kingside(),
            'q' => rights.add_black_queenside(),
            _ => return Err(FenParseError::InvalidCastlingRights),
        }
    }

    board.castling_rights = rights;
    Ok(())
}

/// Sets the en-passant square (fourth in FEN string)
pub fn parse_en_passant(board: &mut Board, en_passant: &str) -> Result<(), FenParseError> {
    if en_passant.len() == 1 && en_passant == "-" {
        board.en_passant_square = None;
        return Ok(());
    }

    let square = Square::from_square(en_passant)?;
    board.en_passant_square = Some(square);

    Ok(())
}

/// Parses the halfmove and fullmove clocks (fifth and sixth in FEN string)
pub fn parse_move_clocks(
    board: &mut Board,
    halfmove: &str,
    fullmove: &str,
) -> Result<(), FenParseError> {
    let halfmove = str::parse::<u8>(halfmove)?;
    let fullmove = str::parse::<u16>(fullmove)?;

    board.halfmove_clock = halfmove;
    board.fullmove_clock = fullmove;

    Ok(())
}
