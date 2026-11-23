//! Board representation: squares, pieces, bitboards, and positions.

mod bitboard;
mod color;
mod piece;
mod position;
mod square;

pub use bitboard::Bitboard;
pub use color::Color;
pub use piece::{Piece, PieceKind};
pub use position::{CastlingRights, Position};
pub use square::Square;
