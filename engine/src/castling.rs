#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Castling {
    rights: u8,
}

impl Castling {
    pub const NO_CASTLING: u8 = 0;
    pub const WHITE_00: u8 = 0b00000001;
    pub const WHITE_000: u8 = 0b00000010;
    pub const BLACK_00: u8 = 0b00000100;
    pub const BLACK_000: u8 = 0b00001000;

    pub const KING_SIDE: u8 = Self::BLACK_00 | Self::WHITE_00;
    pub const QUEEN_SIDE: u8 = Self::BLACK_000 | Self::WHITE_000;
    pub const WHITE_CASTLING: u8 = Self::WHITE_00 | Self::WHITE_000;
    pub const BLACK_CASTLING: u8 = Self::BLACK_00 | Self::BLACK_000;
    pub const ANY_CASTLING: u8 = Self::BLACK_CASTLING | Self::WHITE_CASTLING;

    // Constructor for Castling with specific rights
    pub fn new(rights: u8) -> Self {
        Self { rights }
    }

    // Helper methods to create common castling states
    pub fn none() -> Self {
        Self {
            rights: Self::NO_CASTLING,
        }
    }

    pub fn all() -> Self {
        Self {
            rights: Self::ANY_CASTLING,
        }
    }

    // Methods to check castling rights
    pub fn has_kingside_white(&self) -> bool {
        self.rights & Self::WHITE_00 != 0
    }

    pub fn has_queenside_white(&self) -> bool {
        self.rights & Self::WHITE_000 != 0
    }

    pub fn has_kingside_black(&self) -> bool {
        self.rights & Self::BLACK_00 != 0
    }

    pub fn has_queenside_black(&self) -> bool {
        self.rights & Self::BLACK_000 != 0
    }

    // Methods to modify castling rights
    pub fn remove_white_castling(&mut self) {
        self.rights &= !Self::WHITE_CASTLING;
    }

    pub fn remove_black_castling(&mut self) {
        self.rights &= !Self::BLACK_CASTLING;
    }

    pub fn remove_kingside(&mut self) {
        self.rights &= !Self::KING_SIDE;
    }

    pub fn remove_queenside(&mut self) {
        self.rights &= !Self::QUEEN_SIDE;
    }
}
