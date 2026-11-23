//! Corvin chess rules library (`corvin-chess-core`).
//!
//! Board representation and legal move generation for standard chess.
//! FEN and notation helpers land in later tasks.

#![forbid(unsafe_code)]

mod board;
mod movegen;

pub use board::{Bitboard, CastlingRights, Color, Piece, PieceKind, Position, Square};
pub use movegen::Move;
