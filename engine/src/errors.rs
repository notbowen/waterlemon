use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FenParseError {
    #[error("Invalid FEN length")]
    InvalidLength,

    #[error("Invalid piece placement string")]
    InvalidPiecePlacement,

    #[error("Invalid side to play")]
    InvalidSide,

    #[error("Invalid castling rights")]
    InvalidCastlingRights,

    #[error("Invalid en-passant square")]
    InvalidEnPassant,

    #[error("Invalid move clock value")]
    InvalidMoveClock,
}

impl From<ParseIntError> for FenParseError {
    fn from(_: ParseIntError) -> Self {
        FenParseError::InvalidMoveClock
    }
}

#[derive(Error, Debug)]
pub enum BitboardError {
    #[error("Invalid rank or file")]
    InvalidRankOrFile,
}

impl From<BitboardError> for FenParseError {
    fn from(_: BitboardError) -> Self {
        FenParseError::InvalidPiecePlacement
    }
}

#[derive(Error, Debug)]
pub enum SquareError {
    #[error("Invalid rank or file")]
    InvalidFileRank,

    #[error("Invalid square")]
    InvalidSquare,
}

impl From<SquareError> for FenParseError {
    fn from(value: SquareError) -> Self {
        match value {
            SquareError::InvalidSquare => Self::InvalidEnPassant,
            _ => unimplemented!(),
        }
    }
}
