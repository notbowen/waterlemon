#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match &self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::Color;

    #[test]
    fn test_opposite_color() {
        let black = Color::Black;
        let white = Color::White;

        assert_eq!(black.opposite(), Color::White);
        assert_eq!(white.opposite(), Color::Black);
    }
}
