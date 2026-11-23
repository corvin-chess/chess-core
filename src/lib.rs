//! Corvin chess rules library (`corvin-chess-core`).
//!
//! CV-100 delivers board representation only. Legal move generation, FEN, and
//! notation helpers land in later tasks.

#![forbid(unsafe_code)]

mod board;

pub use board::{Bitboard, CastlingRights, Color, Piece, PieceKind, Position, Square};
