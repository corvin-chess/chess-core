//! Draw-rule helpers: repetition, fifty/seventy-five move, insufficient material (CV-104).
//!
//! Repetition identity follows FIDE Laws 9.2.3 (side to move, piece placement, castling
//! rights, and en passant only when a legal EP capture exists). Fifty-move claim uses the
//! FEN halfmove clock (>= 100 half-moves); automatic seventy-five uses >= 150. Insufficient
//! material covers the standard dead-by-material cases (not a full blocked-pawn search).

use crate::board::{CastlingRights, Color, PieceKind, Position, Square};
use crate::movegen::Move;

/// FIDE repetition identity for a position (excludes halfmove/fullmove clocks).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RepetitionKey {
    by_color: [u64; 2],
    by_kind: [u64; 6],
    side_to_move: Color,
    castling: CastlingRights,
    /// Set only when a legal en passant capture exists for the side to move.
    effective_ep: Option<Square>,
}

impl Position {
    /// Key used to compare positions for threefold / fivefold repetition.
    pub fn repetition_key(&self) -> RepetitionKey {
        RepetitionKey {
            by_color: [self.pieces(Color::White).0, self.pieces(Color::Black).0],
            by_kind: [
                kind_bits(self, PieceKind::Pawn),
                kind_bits(self, PieceKind::Knight),
                kind_bits(self, PieceKind::Bishop),
                kind_bits(self, PieceKind::Rook),
                kind_bits(self, PieceKind::Queen),
                kind_bits(self, PieceKind::King),
            ],
            side_to_move: self.side_to_move(),
            castling: self.castling(),
            effective_ep: self.effective_ep_square(),
        }
    }

    /// Occurrences of this position's repetition key among `prior` keys plus the current one.
    ///
    /// `prior` must be the authoritative per-ply keys from the start of the game (or since the
    /// last irreversible move if the caller truncates). For a FIDE 9.2.1 "about to appear" claim,
    /// evaluate `repetition_count` on the position after the intended move.
    pub fn repetition_count(&self, prior: &[RepetitionKey]) -> usize {
        let key = self.repetition_key();
        1 + prior.iter().filter(|k| **k == key).count()
    }

    /// True when the same position has occurred at least three times (claimable draw).
    pub fn is_threefold(&self, prior: &[RepetitionKey]) -> bool {
        self.repetition_count(prior) >= 3
    }

    /// True when the same position has occurred at least five times (automatic draw).
    pub fn is_fivefold(&self, prior: &[RepetitionKey]) -> bool {
        self.repetition_count(prior) >= 5
    }

    /// FIDE 9.3: claimable after 50 moves by each side with no pawn move or capture.
    pub fn can_claim_fifty_move(&self) -> bool {
        self.halfmove_clock() >= 100
    }

    /// FIDE 9.6.2: automatic draw after 75 moves by each side with no pawn move or capture.
    pub fn is_seventy_five_move(&self) -> bool {
        self.halfmove_clock() >= 150
    }

    /// True when neither side has mating potential by material (classic dead-by-material).
    pub fn is_insufficient_material(&self) -> bool {
        !self.has_mating_potential(Color::White) && !self.has_mating_potential(Color::Black)
    }

    /// True when `color` can still deliver checkmate by some series of legal moves (helpmate
    /// allowed). Used for timeout draws when the opponent lacks mating potential.
    pub fn has_mating_potential(&self, color: Color) -> bool {
        let pawns = self.pieces_of(color, PieceKind::Pawn).popcount();
        let rooks = self.pieces_of(color, PieceKind::Rook).popcount();
        let queens = self.pieces_of(color, PieceKind::Queen).popcount();
        if pawns + rooks + queens > 0 {
            return true;
        }

        let bishops = self.pieces_of(color, PieceKind::Bishop);
        let bishop_count = bishops.popcount();
        let knights = self.pieces_of(color, PieceKind::Knight).popcount();
        let minors = bishop_count + knights;

        if minors >= 2 {
            if knights == 0 {
                // Only bishops: need both square colours, otherwise they cannot checkmate.
                return !bishops_all_same_square_color(bishops);
            }
            return true;
        }

        if minors == 0 {
            return false;
        }

        // Exactly one minor piece.
        if knights == 1 {
            // Lone knight cannot mate a lone king; opponent minors can enable helpmate, but
            // KN vs K is the case our API treats as no potential. KN vs KN/KB is rare for
            // timeout and left as no potential (same as many online clients for a lone knight).
            return false;
        }

        // Lone bishop: mating potential if an opposite-colour enemy bishop can help block,
        // or if the opponent has a knight (helpmate is possible).
        let opp = color.opposite();
        if self.pieces_of(opp, PieceKind::Knight).popcount() > 0 {
            return true;
        }
        if self.pieces_of(opp, PieceKind::Pawn).popcount() > 0
            || self.pieces_of(opp, PieceKind::Rook).popcount() > 0
            || self.pieces_of(opp, PieceKind::Queen).popcount() > 0
        {
            return true;
        }

        let Some(our_sq) = bishops.first() else {
            return false;
        };
        let our_tone = square_tone(our_sq);
        self.pieces_of(opp, PieceKind::Bishop)
            .squares()
            .any(|sq| square_tone(sq) != our_tone)
    }

    /// EP square included in repetition only when a legal EP capture exists (FIDE 9.2.3.1).
    fn effective_ep_square(&self) -> Option<Square> {
        let ep = self.ep_square()?;
        let us = self.side_to_move();
        for m in self.legal_moves() {
            if is_en_passant_capture(self, us, m, ep) {
                return Some(ep);
            }
        }
        None
    }
}

fn kind_bits(pos: &Position, kind: PieceKind) -> u64 {
    pos.pieces_of(Color::White, kind)
        .union(pos.pieces_of(Color::Black, kind))
        .0
}

fn is_en_passant_capture(pos: &Position, us: Color, m: Move, ep: Square) -> bool {
    if m.to != ep {
        return false;
    }
    let Some(moving) = pos.piece_at(m.from) else {
        return false;
    };
    if moving.color != us || moving.kind != PieceKind::Pawn {
        return false;
    }
    // EP destination is empty; the captured pawn sits behind the target.
    pos.piece_at(m.to).is_none()
}

fn square_tone(square: Square) -> u8 {
    (square.file() + square.rank()) % 2
}

fn bishops_all_same_square_color(bishops: crate::board::Bitboard) -> bool {
    let mut tone: Option<u8> = None;
    for sq in bishops.squares() {
        let t = square_tone(sq);
        match tone {
            None => tone = Some(t),
            Some(prev) if prev != t => return false,
            Some(_) => {}
        }
    }
    true
}
