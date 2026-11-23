//! Corvin chess rules library (`corvin-chess-core`).
//!
//! Board representation, legal move generation, FEN load/store, notation helpers,
//! draw-rule helpers (repetition, fifty-move, insufficient material), and perft.
//! Optional WASM bindings live behind the `wasm` feature (`src/wasm.rs`).

// wasm-bindgen glue uses unsafe; keep the native library forbid(unsafe_code).
#![cfg_attr(
    not(all(target_arch = "wasm32", feature = "wasm")),
    forbid(unsafe_code)
)]

mod board;
mod draw;
mod fen;
mod movegen;
mod perft;
mod pgn;
mod san;
mod uci;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
mod wasm;

pub use board::{Bitboard, CastlingRights, Color, Piece, PieceKind, Position, Square};
pub use draw::RepetitionKey;
pub use fen::FenError;
pub use movegen::Move;
pub use pgn::{decode_movetext, encode_movetext, PgnError};
pub use san::SanError;
pub use uci::UciError;
