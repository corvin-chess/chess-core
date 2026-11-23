//! Standard Algebraic Notation encode/decode with FIDE disambiguation.

use crate::board::{PieceKind, Position, Square};
use crate::movegen::Move;
use std::fmt;

/// Error encoding or decoding SAN.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SanError {
    Invalid,
    Illegal,
    Ambiguous,
}

impl fmt::Display for SanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SanError::Invalid => write!(f, "invalid SAN"),
            SanError::Illegal => write!(f, "illegal SAN for this position"),
            SanError::Ambiguous => write!(f, "ambiguous SAN for this position"),
        }
    }
}

impl std::error::Error for SanError {}

impl Position {
    /// Encode a legal move as SAN (including `+` / `#` when applicable).
    pub fn to_san(&self, m: Move) -> Result<String, SanError> {
        if !self.legal_moves().contains(&m) {
            return Err(SanError::Illegal);
        }
        let moving = self.piece_at(m.from).ok_or(SanError::Illegal)?;
        let after = self.try_make_move(m).ok_or(SanError::Illegal)?;
        let check = after.in_check();
        let mate = check && after.legal_moves().is_empty();

        let mut san = if moving.kind == PieceKind::King
            && (m.to.file() as i8 - m.from.file() as i8).abs() == 2
        {
            if m.to.file() > m.from.file() {
                "O-O".to_string()
            } else {
                "O-O-O".to_string()
            }
        } else {
            let is_capture = self.is_capture(m);
            let mut body = String::with_capacity(8);

            if moving.kind == PieceKind::Pawn {
                if is_capture {
                    body.push(file_char(m.from.file()));
                    body.push('x');
                }
                body.push_str(&m.to.to_algebraic());
            } else {
                body.push(piece_char(moving.kind));
                body.push_str(&disambiguation(self, m, moving.kind));
                if is_capture {
                    body.push('x');
                }
                body.push_str(&m.to.to_algebraic());
            }

            if let Some(promo) = m.promotion {
                body.push('=');
                body.push(piece_char(promo));
            }
            body
        };

        if mate {
            san.push('#');
        } else if check {
            san.push('+');
        }
        Ok(san)
    }

    /// Resolve SAN to the unique legal move in this position.
    pub fn from_san(&self, san: &str) -> Result<Move, SanError> {
        let trimmed = strip_suffix_noise(san);
        if trimmed.is_empty() {
            return Err(SanError::Invalid);
        }

        if is_castle_kingside(trimmed) {
            return unique_castle(self, true);
        }
        if is_castle_queenside(trimmed) {
            return unique_castle(self, false);
        }

        let (core, promo) = split_promotion(trimmed)?;
        let bytes = core.as_bytes();
        if bytes.len() < 2 {
            return Err(SanError::Invalid);
        }

        let (kind, rest) = match bytes[0] {
            b'N' => (PieceKind::Knight, &core[1..]),
            b'B' => (PieceKind::Bishop, &core[1..]),
            b'R' => (PieceKind::Rook, &core[1..]),
            b'Q' => (PieceKind::Queen, &core[1..]),
            b'K' => (PieceKind::King, &core[1..]),
            b'a'..=b'h' => (PieceKind::Pawn, core),
            _ => return Err(SanError::Invalid),
        };

        let rest = rest.trim_start_matches('x');
        if rest.len() < 2 {
            return Err(SanError::Invalid);
        }

        // Destination is the trailing two algebraic chars; optional from-file/rank before that.
        let (hint, dest_str) = rest.split_at(rest.len() - 2);
        let to = Square::from_algebraic(dest_str).ok_or(SanError::Invalid)?;

        let mut from_file: Option<u8> = None;
        let mut from_rank: Option<u8> = None;
        for ch in hint.chars() {
            match ch {
                'a'..='h' => {
                    if from_file.is_some() {
                        return Err(SanError::Invalid);
                    }
                    from_file = Some(ch as u8 - b'a');
                }
                '1'..='8' => {
                    if from_rank.is_some() {
                        return Err(SanError::Invalid);
                    }
                    from_rank = Some(ch as u8 - b'1');
                }
                'x' => {}
                _ => return Err(SanError::Invalid),
            }
        }

        // Pawn captures require a from-file (`exd5`); quiet pawn pushes have empty hint.
        if kind == PieceKind::Pawn {
            let is_capture_san = san_body_has_capture(trimmed);
            if is_capture_san && from_file.is_none() {
                return Err(SanError::Invalid);
            }
        }

        let wants_capture = san_body_has_capture(trimmed);
        let mut matches: Vec<Move> = self
            .legal_moves()
            .into_iter()
            .filter(|m| {
                if m.to != to || m.promotion != promo {
                    return false;
                }
                let Some(piece) = self.piece_at(m.from) else {
                    return false;
                };
                if piece.kind != kind {
                    return false;
                }
                if let Some(f) = from_file {
                    if m.from.file() != f {
                        return false;
                    }
                }
                if let Some(r) = from_rank {
                    if m.from.rank() != r {
                        return false;
                    }
                }
                // `x` must mean a real capture (including en passant); quiet SAN must not use `x`.
                wants_capture == self.is_capture(*m)
            })
            .collect();

        match matches.len() {
            0 => Err(SanError::Illegal),
            1 => Ok(matches.pop().expect("len 1")),
            _ => Err(SanError::Ambiguous),
        }
    }

    fn is_capture(&self, m: Move) -> bool {
        if self.piece_at(m.to).is_some() {
            return true;
        }
        // En passant: pawn moves to the EP square.
        matches!(self.piece_at(m.from), Some(p) if p.kind == PieceKind::Pawn)
            && self.ep_square() == Some(m.to)
    }
}

fn disambiguation(pos: &Position, m: Move, kind: PieceKind) -> String {
    let candidates: Vec<Move> = pos
        .legal_moves()
        .into_iter()
        .filter(|other| {
            other.to == m.to
                && other.promotion == m.promotion
                && matches!(pos.piece_at(other.from), Some(p) if p.kind == kind)
        })
        .collect();

    if candidates.len() <= 1 {
        return String::new();
    }

    // Prefer file when it uniquely identifies the origin; else rank; else both.
    let file_unique = candidates
        .iter()
        .filter(|c| c.from.file() == m.from.file())
        .count()
        == 1;
    let rank_unique = candidates
        .iter()
        .filter(|c| c.from.rank() == m.from.rank())
        .count()
        == 1;

    let mut out = String::new();
    if file_unique {
        out.push(file_char(m.from.file()));
    } else if rank_unique {
        out.push(rank_char(m.from.rank()));
    } else {
        out.push(file_char(m.from.file()));
        out.push(rank_char(m.from.rank()));
    }
    out
}

fn unique_castle(pos: &Position, kingside: bool) -> Result<Move, SanError> {
    let us = pos.side_to_move();
    let (from, to) = match (us, kingside) {
        (crate::board::Color::White, true) => (Square::E1, Square::G1),
        (crate::board::Color::White, false) => (Square::E1, Square::C1),
        (crate::board::Color::Black, true) => (Square::E8, Square::G8),
        (crate::board::Color::Black, false) => (Square::E8, Square::C8),
    };
    let m = Move {
        from,
        to,
        promotion: None,
    };
    if pos.legal_moves().contains(&m) {
        Ok(m)
    } else {
        Err(SanError::Illegal)
    }
}

fn strip_suffix_noise(san: &str) -> &str {
    let mut s = san.trim();
    // Drop trailing annotation / check markers for matching; check is not required to match.
    while let Some(last) = s.chars().last() {
        if matches!(last, '+' | '#' | '!' | '?') {
            s = &s[..s.len() - last.len_utf8()];
        } else {
            break;
        }
    }
    s
}

fn is_castle_kingside(s: &str) -> bool {
    matches!(s, "O-O" | "0-0" | "o-o")
}

fn is_castle_queenside(s: &str) -> bool {
    matches!(s, "O-O-O" | "0-0-0" | "o-o-o")
}

fn split_promotion(s: &str) -> Result<(&str, Option<PieceKind>), SanError> {
    if let Some(idx) = s.find('=') {
        let (left, right) = s.split_at(idx);
        let promo = right.as_bytes().get(1).copied().ok_or(SanError::Invalid)?;
        let kind = match promo {
            b'Q' | b'q' => PieceKind::Queen,
            b'R' | b'r' => PieceKind::Rook,
            b'B' | b'b' => PieceKind::Bishop,
            b'N' | b'n' => PieceKind::Knight,
            _ => return Err(SanError::Invalid),
        };
        if right.len() != 2 {
            return Err(SanError::Invalid);
        }
        Ok((left, Some(kind)))
    } else {
        // Bare promotion suffix without '=' is not accepted (keep PGN/SAN strict).
        Ok((s, None))
    }
}

fn san_body_has_capture(s: &str) -> bool {
    s.bytes().any(|b| b == b'x')
}

fn piece_char(kind: PieceKind) -> char {
    match kind {
        PieceKind::Pawn => 'P',
        PieceKind::Knight => 'N',
        PieceKind::Bishop => 'B',
        PieceKind::Rook => 'R',
        PieceKind::Queen => 'Q',
        PieceKind::King => 'K',
    }
}

fn file_char(file: u8) -> char {
    char::from(b'a' + file)
}

fn rank_char(rank: u8) -> char {
    char::from(b'1' + rank)
}
