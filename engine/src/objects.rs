#![allow(dead_code)]

use std::ops::{Index, IndexMut};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Sides {
    WHITE = 0,
    BLACK = 1,
}

impl<T> Index<Sides> for [T; 2] {
    type Output = T;

    fn index(&self, side: Sides) -> &Self::Output {
        &self[side as usize]
    }
}

impl<T> IndexMut<Sides> for [T; 2] {
    fn index_mut(&mut self, side: Sides) -> &mut Self::Output {
        &mut self[side as usize]
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Pieces {
    PAWN = 0,
    BISHOP = 1,
    KNIGHT = 2,
    ROOK = 3,
    QUEEN = 4,
    KING = 5,
}

impl<T> Index<Pieces> for [T; 6] {
    type Output = T;

    fn index(&self, piece: Pieces) -> &Self::Output {
        &self[piece as usize]
    }
}

impl<T> IndexMut<Pieces> for [T; 6] {
    fn index_mut(&mut self, piece: Pieces) -> &mut Self::Output {
        &mut self[piece as usize]
    }
}
