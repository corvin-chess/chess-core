//! Forsyth-Edwards Notation load and store (six-field FEN).

use crate::{CastlingRights, Color, Piece, PieceKind, Position, Square};
use std::fmt;

/// Why a FEN string could not be parsed.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FenError {
    FieldCount,
    Placement,
    SideToMove,
    Castling,
    EnPassant,
    HalfmoveClock,
    FullmoveNumber,
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            FenError::FieldCount => "FEN must have exactly six fields",
            FenError::Placement => "invalid piece placement",
            FenError::SideToMove => "side to move must be w or b",
            FenError::Castling => "invalid castling field",
            FenError::EnPassant => "invalid en passant square",
            FenError::HalfmoveClock => "invalid halfmove clock",
            FenError::FullmoveNumber => "invalid fullmove number",
        };
        f.write_str(msg)
    }
}

impl std::error::Error for FenError {}

impl Position {
    /// Parse a six-field FEN into a position.
    pub fn from_fen(fen: &str) -> Result<Position, FenError> {
        let fields: Vec<&str> = fen.split_whitespace().collect();
        if fields.len() != 6 {
            return Err(FenError::FieldCount);
        }

        let mut pos = Position::empty();
        parse_placement(&mut pos, fields[0])?;
        pos.set_side_to_move(parse_side(fields[1])?);
        pos.set_castling(parse_castling(fields[2])?);
        pos.set_ep_square(parse_ep(fields[3])?);
        pos.set_halfmove_clock(parse_halfmove(fields[4])?);
        pos.set_fullmove_number(parse_fullmove(fields[5])?);
        Ok(pos)
    }

    /// Serialize this position to a six-field FEN string.
    pub fn to_fen(&self) -> String {
        let mut out = String::with_capacity(64);
        push_placement(&mut out, self);
        out.push(' ');
        out.push(match self.side_to_move() {
            Color::White => 'w',
            Color::Black => 'b',
        });
        out.push(' ');
        push_castling(&mut out, self.castling());
        out.push(' ');
        match self.ep_square() {
            Some(sq) => out.push_str(&sq.to_algebraic()),
            None => out.push('-'),
        }
        out.push(' ');
        out.push_str(&self.halfmove_clock().to_string());
        out.push(' ');
        out.push_str(&self.fullmove_number().to_string());
        out
    }
}

fn parse_placement(pos: &mut Position, placement: &str) -> Result<(), FenError> {
    let ranks: Vec<&str> = placement.split('/').collect();
    if ranks.len() != 8 {
        return Err(FenError::Placement);
    }

    for (rank_i, rank_str) in ranks.iter().enumerate() {
        // FEN lists rank 8 first.
        let rank = 7u8 - rank_i as u8;
        let mut file: u8 = 0;
        let mut prev_was_empty = false;
        for ch in rank_str.chars() {
            if !ch.is_ascii() {
                return Err(FenError::Placement);
            }
            if ch.is_ascii_digit() {
                // Spec digits are 1-8; reject '0', adjacent empties, and non-ASCII Nd.
                if prev_was_empty || !('1'..='8').contains(&ch) {
                    return Err(FenError::Placement);
                }
                let empty = (ch as u8) - b'0';
                file = file.checked_add(empty).ok_or(FenError::Placement)?;
                if file > 8 {
                    return Err(FenError::Placement);
                }
                prev_was_empty = true;
                continue;
            }
            let piece = piece_from_char(ch).ok_or(FenError::Placement)?;
            if file >= 8 {
                return Err(FenError::Placement);
            }
            let square = Square::from_file_rank(file, rank).ok_or(FenError::Placement)?;
            pos.set_piece(square, piece);
            file += 1;
            prev_was_empty = false;
        }
        if file != 8 {
            return Err(FenError::Placement);
        }
    }
    Ok(())
}

fn piece_from_char(ch: char) -> Option<Piece> {
    let (color, kind) = match ch {
        'P' => (Color::White, PieceKind::Pawn),
        'N' => (Color::White, PieceKind::Knight),
        'B' => (Color::White, PieceKind::Bishop),
        'R' => (Color::White, PieceKind::Rook),
        'Q' => (Color::White, PieceKind::Queen),
        'K' => (Color::White, PieceKind::King),
        'p' => (Color::Black, PieceKind::Pawn),
        'n' => (Color::Black, PieceKind::Knight),
        'b' => (Color::Black, PieceKind::Bishop),
        'r' => (Color::Black, PieceKind::Rook),
        'q' => (Color::Black, PieceKind::Queen),
        'k' => (Color::Black, PieceKind::King),
        _ => return None,
    };
    Some(Piece { color, kind })
}

fn piece_to_char(piece: Piece) -> char {
    let ch = match piece.kind {
        PieceKind::Pawn => 'p',
        PieceKind::Knight => 'n',
        PieceKind::Bishop => 'b',
        PieceKind::Rook => 'r',
        PieceKind::Queen => 'q',
        PieceKind::King => 'k',
    };
    match piece.color {
        Color::White => ch.to_ascii_uppercase(),
        Color::Black => ch,
    }
}

fn push_placement(out: &mut String, pos: &Position) {
    for rank in (0u8..8).rev() {
        let mut empty = 0u8;
        for file in 0u8..8 {
            let square = Square::from_file_rank(file, rank).expect("file/rank in range");
            match pos.piece_at(square) {
                None => empty += 1,
                Some(piece) => {
                    if empty > 0 {
                        out.push(char::from(b'0' + empty));
                        empty = 0;
                    }
                    out.push(piece_to_char(piece));
                }
            }
        }
        if empty > 0 {
            out.push(char::from(b'0' + empty));
        }
        if rank > 0 {
            out.push('/');
        }
    }
}

fn parse_side(field: &str) -> Result<Color, FenError> {
    match field {
        "w" => Ok(Color::White),
        "b" => Ok(Color::Black),
        _ => Err(FenError::SideToMove),
    }
}

fn parse_castling(field: &str) -> Result<CastlingRights, FenError> {
    if field == "-" {
        return Ok(CastlingRights::none());
    }
    let mut rights = CastlingRights::none();
    let mut seen = [false; 4];
    for ch in field.chars() {
        let idx = match ch {
            'K' => 0,
            'Q' => 1,
            'k' => 2,
            'q' => 3,
            _ => return Err(FenError::Castling),
        };
        if seen[idx] {
            return Err(FenError::Castling);
        }
        seen[idx] = true;
        match ch {
            'K' => rights.white_king = true,
            'Q' => rights.white_queen = true,
            'k' => rights.black_king = true,
            'q' => rights.black_queen = true,
            _ => unreachable!(),
        }
    }
    // Require standard KQkq order among the letters that appear.
    let mut expected = String::new();
    if rights.white_king {
        expected.push('K');
    }
    if rights.white_queen {
        expected.push('Q');
    }
    if rights.black_king {
        expected.push('k');
    }
    if rights.black_queen {
        expected.push('q');
    }
    if field != expected {
        return Err(FenError::Castling);
    }
    Ok(rights)
}

fn push_castling(out: &mut String, rights: CastlingRights) {
    let mut any = false;
    if rights.white_king {
        out.push('K');
        any = true;
    }
    if rights.white_queen {
        out.push('Q');
        any = true;
    }
    if rights.black_king {
        out.push('k');
        any = true;
    }
    if rights.black_queen {
        out.push('q');
        any = true;
    }
    if !any {
        out.push('-');
    }
}

fn parse_ep(field: &str) -> Result<Option<Square>, FenError> {
    if field == "-" {
        return Ok(None);
    }
    let square = Square::from_algebraic(field).ok_or(FenError::EnPassant)?;
    // Spec: EP target is only on rank 3 (white double push) or 6 (black).
    if square.rank() != 2 && square.rank() != 5 {
        return Err(FenError::EnPassant);
    }
    Ok(Some(square))
}

fn parse_halfmove(field: &str) -> Result<u16, FenError> {
    if field.is_empty() || !field.bytes().all(|b| b.is_ascii_digit()) {
        return Err(FenError::HalfmoveClock);
    }
    field.parse::<u16>().map_err(|_| FenError::HalfmoveClock)
}

fn parse_fullmove(field: &str) -> Result<u16, FenError> {
    if field.is_empty() || !field.bytes().all(|b| b.is_ascii_digit()) {
        return Err(FenError::FullmoveNumber);
    }
    let n = field.parse::<u16>().map_err(|_| FenError::FullmoveNumber)?;
    if n == 0 {
        return Err(FenError::FullmoveNumber);
    }
    Ok(n)
}
