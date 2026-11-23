//! Corvin chess rules library (`corvin-chess-core`).
//!
//! Board representation, legal move generation, and FEN load/store for standard chess.
//! SAN/UCI/PGN helpers land in later tasks.

#![forbid(unsafe_code)]

mod board;
mod fen;
mod movegen;

pub use board::{Bitboard, CastlingRights, Color, Piece, PieceKind, Position, Square};
pub use fen::FenError;
pub use movegen::Move;
