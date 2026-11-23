//! Thin WASM/JS surface for Node and bundler consumers (CV-106).
//!
//! Enabled only with `--features wasm` on `wasm32`. Keeps the native crate
//! `forbid(unsafe_code)`; wasm-bindgen's generated glue needs unsafe.

// wasm-bindgen expands to const items newer than our crate MSRV (1.75).
#![allow(clippy::incompatible_msrv)]

use crate::board::Position;
use crate::movegen::Move;
use wasm_bindgen::prelude::*;

/// Generous upper bound for a six-field FEN (rejects abuse at the JS boundary).
const MAX_FEN_LEN: usize = 256;

/// Legal moves for `fen` as UCI strings (`e2e4`, `a7a8q`).
///
/// Throws a JS `Error` when the FEN is invalid or longer than 256 bytes.
#[wasm_bindgen(js_name = legalMoves)]
pub fn legal_moves(fen: &str) -> Result<Vec<String>, JsValue> {
    if fen.len() > MAX_FEN_LEN {
        return Err(JsValue::from_str("invalid FEN: input too long"));
    }
    let position =
        Position::from_fen(fen).map_err(|e| JsValue::from_str(&format!("invalid FEN: {e}")))?;
    Ok(position
        .legal_moves()
        .into_iter()
        .map(Move::to_uci)
        .collect())
}
