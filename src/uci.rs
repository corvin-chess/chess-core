//! UCI long algebraic move encoding (`e2e4`, `a7a8q`).

use crate::board::{PieceKind, Square};
use crate::movegen::Move;
use std::fmt;

/// Error parsing a UCI move string.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UciError {
    Invalid,
}

impl fmt::Display for UciError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UciError::Invalid => write!(f, "invalid UCI move"),
        }
    }
}

impl std::error::Error for UciError {}

impl Move {
    /// Encode as UCI (`e2e4`, `e7e8q`). Promotion uses lowercase `q r b n`.
    pub fn to_uci(self) -> String {
        let mut s = String::with_capacity(5);
        s.push_str(&self.from.to_algebraic());
        s.push_str(&self.to.to_algebraic());
        if let Some(kind) = self.promotion {
            s.push(promo_char(kind).expect("promotion is QRBN"));
        }
        s
    }

    /// Parse a UCI move. Accepts optional promotion `q|r|b|n` (case-insensitive).
    pub fn from_uci(s: &str) -> Result<Move, UciError> {
        let bytes = s.as_bytes();
        if bytes.len() != 4 && bytes.len() != 5 {
            return Err(UciError::Invalid);
        }
        let from = Square::from_algebraic(
            std::str::from_utf8(&bytes[0..2]).map_err(|_| UciError::Invalid)?,
        )
        .ok_or(UciError::Invalid)?;
        let to = Square::from_algebraic(
            std::str::from_utf8(&bytes[2..4]).map_err(|_| UciError::Invalid)?,
        )
        .ok_or(UciError::Invalid)?;
        let promotion = if bytes.len() == 5 {
            Some(parse_promo(bytes[4]).ok_or(UciError::Invalid)?)
        } else {
            None
        };
        Ok(Move {
            from,
            to,
            promotion,
        })
    }
}

fn promo_char(kind: PieceKind) -> Option<char> {
    match kind {
        PieceKind::Queen => Some('q'),
        PieceKind::Rook => Some('r'),
        PieceKind::Bishop => Some('b'),
        PieceKind::Knight => Some('n'),
        PieceKind::Pawn | PieceKind::King => None,
    }
}

fn parse_promo(c: u8) -> Option<PieceKind> {
    match c.to_ascii_lowercase() {
        b'q' => Some(PieceKind::Queen),
        b'r' => Some(PieceKind::Rook),
        b'b' => Some(PieceKind::Bishop),
        b'n' => Some(PieceKind::Knight),
        _ => None,
    }
}
