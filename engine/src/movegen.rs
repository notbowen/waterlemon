#![allow(dead_code)]

use crate::{objects::Pieces, square::Square};

struct Move {
    from: Square,
    to: Square,
    promotion: Option<Pieces>,
}
