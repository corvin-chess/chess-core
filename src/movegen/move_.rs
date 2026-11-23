//! A single chess move: origin, destination, optional promotion.

use crate::board::{PieceKind, Square};

/// From-to move with optional promotion piece.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceKind>,
}
