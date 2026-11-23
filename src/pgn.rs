//! Minimal PGN movetext helpers (no tag-pair games yet).

use crate::board::Position;
use crate::movegen::Move;
use crate::san::SanError;
use std::fmt;

/// Error encoding or decoding PGN movetext.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PgnError {
    San(SanError),
    Invalid,
    Illegal,
}

impl fmt::Display for PgnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PgnError::San(e) => write!(f, "PGN SAN error: {e}"),
            PgnError::Invalid => write!(f, "invalid PGN movetext"),
            PgnError::Illegal => write!(f, "illegal move in PGN movetext"),
        }
    }
}

impl std::error::Error for PgnError {}

impl From<SanError> for PgnError {
    fn from(value: SanError) -> Self {
        match value {
            SanError::Illegal => PgnError::Illegal,
            other => PgnError::San(other),
        }
    }
}

/// Encode a legal move sequence from `start` as PGN movetext (`1. e4 e5 2. Nf3`).
pub fn encode_movetext(start: &Position, moves: &[Move]) -> Result<String, PgnError> {
    let mut pos = start.clone();
    let mut out = String::new();

    // If Black is to move at the start, begin with an ellipsis form: `1... e5`.
    let black_starts = matches!(pos.side_to_move(), crate::board::Color::Black);

    for (ply_index, m) in moves.iter().enumerate() {
        let san = pos.to_san(*m).map_err(PgnError::from)?;
        let fullmove = pos.fullmove_number();
        match pos.side_to_move() {
            crate::board::Color::White => {
                if !out.is_empty() {
                    out.push(' ');
                }
                out.push_str(&format!("{fullmove}. {san}"));
            }
            crate::board::Color::Black => {
                if ply_index == 0 && black_starts {
                    out.push_str(&format!("{fullmove}... {san}"));
                } else {
                    out.push(' ');
                    out.push_str(&san);
                }
            }
        }
        pos = pos.try_make_move(*m).ok_or(PgnError::Illegal)?;
    }

    Ok(out)
}

/// Decode PGN movetext into a move list played from `start`.
///
/// Accepts numbered (`1. e4 e5`) or bare (`e4 e5`) tokens. Ignores result markers
/// `1-0`, `0-1`, `1/2-1/2`, and `*`.
pub fn decode_movetext(start: &Position, text: &str) -> Result<Vec<Move>, PgnError> {
    let mut pos = start.clone();
    let mut moves = Vec::new();

    for raw in text.split_whitespace() {
        let token = raw.trim_matches(|c| c == ';' || c == ',');
        if token.is_empty() {
            continue;
        }
        if is_result_token(token) {
            break;
        }
        if is_move_number(token) {
            continue;
        }
        let san = token.trim_end_matches(['!', '?', '+', '#']);
        // Keep `+`/`#` out of SAN parser input? Parser strips them; either works.
        // Prefer full token so `e8=Q+` still parses after strip inside from_san.
        let m = pos.from_san(token).or_else(|_| pos.from_san(san))?;
        pos = pos.try_make_move(m).ok_or(PgnError::Illegal)?;
        moves.push(m);
    }

    Ok(moves)
}

fn is_result_token(token: &str) -> bool {
    matches!(token, "1-0" | "0-1" | "1/2-1/2" | "*")
}

fn is_move_number(token: &str) -> bool {
    // `1.` `12.` `1...` `12...`
    let t = token.trim_end_matches('.');
    !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit())
}
