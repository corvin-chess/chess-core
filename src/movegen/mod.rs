//! Legal move generation for standard chess (CV-101).
//!
//! Pseudo-legal moves are generated from the side to move, then filtered by
//! applying each candidate and rejecting any that leave the mover's king in check.
//! That single filter covers pins, discovered checks, and illegal en passant.

mod attacks;
mod move_;

pub use move_::Move;

use crate::board::{Color, PieceKind, Position, Square};
use attacks::{
    bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks,
};

impl Position {
    /// True when the side to move's king is attacked by the opponent.
    pub fn in_check(&self) -> bool {
        let us = self.side_to_move();
        match self.king_square(us) {
            Some(king) => self.is_square_attacked(king, us.opposite()),
            None => false,
        }
    }

    /// All legal moves for the side to move.
    pub fn legal_moves(&self) -> Vec<Move> {
        self.pseudo_legal_moves()
            .into_iter()
            .filter(|m| self.leaves_king_safe(*m))
            .collect()
    }

    /// Applies `m` when it is legal for the side to move; otherwise `None`.
    pub fn try_make_move(&self, m: Move) -> Option<Position> {
        if !self.pseudo_legal_moves().contains(&m) {
            return None;
        }
        if !self.leaves_king_safe(m) {
            return None;
        }
        Some(self.apply_move_unchecked(m))
    }

    fn leaves_king_safe(&self, m: Move) -> bool {
        let us = self.side_to_move();
        let after = self.apply_move_unchecked(m);
        match after.king_square(us) {
            Some(king) => !after.is_square_attacked(king, us.opposite()),
            None => false,
        }
    }

    pub(crate) fn king_square(&self, color: Color) -> Option<Square> {
        self.pieces_of(color, PieceKind::King).first()
    }

    /// True if `by` attacks `square` in this position.
    pub fn is_square_attacked(&self, square: Square, by: Color) -> bool {
        let occupied = self.occupied();

        let pawns = self.pieces_of(by, PieceKind::Pawn);
        if pawn_attacks(square, by.opposite()) & pawns.0 != 0 {
            return true;
        }

        let knights = self.pieces_of(by, PieceKind::Knight);
        if knight_attacks(square) & knights.0 != 0 {
            return true;
        }

        let kings = self.pieces_of(by, PieceKind::King);
        if king_attacks(square) & kings.0 != 0 {
            return true;
        }

        let bishops_queens = self
            .pieces_of(by, PieceKind::Bishop)
            .union(self.pieces_of(by, PieceKind::Queen));
        if bishop_attacks(square, occupied) & bishops_queens.0 != 0 {
            return true;
        }

        let rooks_queens = self
            .pieces_of(by, PieceKind::Rook)
            .union(self.pieces_of(by, PieceKind::Queen));
        rook_attacks(square, occupied) & rooks_queens.0 != 0
    }

    fn pseudo_legal_moves(&self) -> Vec<Move> {
        let us = self.side_to_move();
        let them = us.opposite();
        let occupied = self.occupied();
        let own = self.pieces(us);
        let enemy = self.pieces(them);
        let mut moves = Vec::with_capacity(64);

        for from in self.pieces_of(us, PieceKind::Pawn).squares() {
            self.gen_pawn_moves(from, us, occupied, enemy, &mut moves);
        }
        for from in self.pieces_of(us, PieceKind::Knight).squares() {
            push_targets(from, knight_attacks(from) & !own.0, None, &mut moves);
        }
        for from in self.pieces_of(us, PieceKind::Bishop).squares() {
            push_targets(
                from,
                bishop_attacks(from, occupied) & !own.0,
                None,
                &mut moves,
            );
        }
        for from in self.pieces_of(us, PieceKind::Rook).squares() {
            push_targets(
                from,
                rook_attacks(from, occupied) & !own.0,
                None,
                &mut moves,
            );
        }
        for from in self.pieces_of(us, PieceKind::Queen).squares() {
            push_targets(
                from,
                queen_attacks(from, occupied) & !own.0,
                None,
                &mut moves,
            );
        }
        for from in self.pieces_of(us, PieceKind::King).squares() {
            push_targets(from, king_attacks(from) & !own.0, None, &mut moves);
            self.gen_castling(from, us, occupied, &mut moves);
        }

        moves
    }

    fn gen_pawn_moves(
        &self,
        from: Square,
        us: Color,
        occupied: crate::board::Bitboard,
        enemy: crate::board::Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let (forward, start_rank, promo_rank) = match us {
            Color::White => (1i8, 1u8, 7u8),
            Color::Black => (-1i8, 6u8, 0u8),
        };

        if let Some(one) = offset(from, 0, forward) {
            if !occupied.contains(one) {
                push_pawn(from, one, promo_rank, moves);
                if from.rank() == start_rank {
                    if let Some(two) = offset(from, 0, forward * 2) {
                        if !occupied.contains(two) {
                            moves.push(Move {
                                from,
                                to: two,
                                promotion: None,
                            });
                        }
                    }
                }
            }
        }

        for &df in &[-1i8, 1i8] {
            if let Some(to) = offset(from, df, forward) {
                let is_ep = self.ep_square() == Some(to);
                if enemy.contains(to) || is_ep {
                    push_pawn(from, to, promo_rank, moves);
                }
            }
        }
    }

    fn gen_castling(
        &self,
        king: Square,
        us: Color,
        occupied: crate::board::Bitboard,
        moves: &mut Vec<Move>,
    ) {
        let rights = self.castling();
        let them = us.opposite();

        let (king_from, rook_k, through_k, dest_k, rook_q, through_q, dest_q, empty_q) = match us {
            Color::White => (
                Square::E1,
                rights.white_king,
                [Square::F1, Square::G1],
                Square::G1,
                rights.white_queen,
                [Square::D1, Square::C1],
                Square::C1,
                [Square::B1, Square::C1, Square::D1],
            ),
            Color::Black => (
                Square::E8,
                rights.black_king,
                [Square::F8, Square::G8],
                Square::G8,
                rights.black_queen,
                [Square::D8, Square::C8],
                Square::C8,
                [Square::B8, Square::C8, Square::D8],
            ),
        };

        if king != king_from {
            return;
        }

        // Cannot castle out of check.
        if self.is_square_attacked(king_from, them) {
            return;
        }

        if rook_k {
            let rook_sq = match us {
                Color::White => Square::H1,
                Color::Black => Square::H8,
            };
            let rook_ok = matches!(
                self.piece_at(rook_sq),
                Some(crate::board::Piece {
                    color,
                    kind: PieceKind::Rook,
                }) if color == us
            );
            let path_clear = through_k.iter().all(|sq| !occupied.contains(*sq));
            let safe = through_k
                .iter()
                .all(|sq| !self.is_square_attacked(*sq, them));
            if rook_ok && path_clear && safe {
                moves.push(Move {
                    from: king_from,
                    to: dest_k,
                    promotion: None,
                });
            }
        }

        if rook_q {
            let rook_sq = match us {
                Color::White => Square::A1,
                Color::Black => Square::A8,
            };
            let rook_ok = matches!(
                self.piece_at(rook_sq),
                Some(crate::board::Piece {
                    color,
                    kind: PieceKind::Rook,
                }) if color == us
            );
            let path_clear = empty_q.iter().all(|sq| !occupied.contains(*sq));
            let safe = through_q
                .iter()
                .all(|sq| !self.is_square_attacked(*sq, them));
            if rook_ok && path_clear && safe {
                moves.push(Move {
                    from: king_from,
                    to: dest_q,
                    promotion: None,
                });
            }
        }
    }

    /// Applies a move that has already been validated as pseudo-legal for this side.
    /// Caller must still verify king safety when filtering.
    fn apply_move_unchecked(&self, m: Move) -> Position {
        let us = self.side_to_move();
        let them = us.opposite();
        let mut next = self.clone();
        let moving = next.piece_at(m.from).expect("move from an occupied square");

        let is_ep = moving.kind == PieceKind::Pawn && self.ep_square() == Some(m.to);
        let is_castle =
            moving.kind == PieceKind::King && (m.to.file() as i8 - m.from.file() as i8).abs() == 2;

        let reset_halfmove =
            moving.kind == PieceKind::Pawn || next.piece_at(m.to).is_some() || is_ep;

        next.clear_square(m.from);

        if is_ep {
            let captured = match us {
                Color::White => offset(m.to, 0, -1),
                Color::Black => offset(m.to, 0, 1),
            }
            .expect("EP target has a capture square");
            next.clear_square(captured);
        }

        if is_castle {
            let (rook_from, rook_to) = match (us, m.to.file()) {
                (Color::White, 6) => (Square::H1, Square::F1),
                (Color::White, 2) => (Square::A1, Square::D1),
                (Color::Black, 6) => (Square::H8, Square::F8),
                (Color::Black, 2) => (Square::A8, Square::D8),
                _ => panic!("unexpected castling destination"),
            };
            next.clear_square(rook_from);
            next.set_piece(
                rook_to,
                crate::board::Piece {
                    color: us,
                    kind: PieceKind::Rook,
                },
            );
        } else if next.piece_at(m.to).is_some() {
            next.clear_square(m.to);
        }

        let placed_kind = m.promotion.unwrap_or(moving.kind);
        next.set_piece(
            m.to,
            crate::board::Piece {
                color: us,
                kind: placed_kind,
            },
        );

        // Castling rights.
        let mut rights = next.castling();
        if moving.kind == PieceKind::King {
            match us {
                Color::White => {
                    rights.white_king = false;
                    rights.white_queen = false;
                }
                Color::Black => {
                    rights.black_king = false;
                    rights.black_queen = false;
                }
            }
        }
        if moving.kind == PieceKind::Rook {
            clear_rook_right(&mut rights, us, m.from);
        }
        if let Some(captured) = self.piece_at(m.to) {
            if captured.kind == PieceKind::Rook {
                clear_rook_right(&mut rights, them, m.to);
            }
        }
        next.set_castling(rights);

        // En passant target: only after a double pawn push.
        let ep = if moving.kind == PieceKind::Pawn {
            let dr = m.to.rank() as i8 - m.from.rank() as i8;
            if dr.abs() == 2 {
                offset(m.from, 0, dr / 2)
            } else {
                None
            }
        } else {
            None
        };
        next.set_ep_square(ep);

        if reset_halfmove {
            next.set_halfmove_clock(0);
        } else {
            next.set_halfmove_clock(self.halfmove_clock().saturating_add(1));
        }

        if us == Color::Black {
            next.set_fullmove_number(self.fullmove_number().saturating_add(1));
        }

        next.set_side_to_move(them);
        next
    }
}

fn clear_rook_right(rights: &mut crate::board::CastlingRights, color: Color, square: Square) {
    match (color, square) {
        (Color::White, Square::H1) => rights.white_king = false,
        (Color::White, Square::A1) => rights.white_queen = false,
        (Color::Black, Square::H8) => rights.black_king = false,
        (Color::Black, Square::A8) => rights.black_queen = false,
        _ => {}
    }
}

fn push_targets(from: Square, targets: u64, promotion: Option<PieceKind>, moves: &mut Vec<Move>) {
    let mut bits = targets;
    while bits != 0 {
        let idx = bits.trailing_zeros() as u8;
        bits &= bits - 1;
        let to = Square::from_index(idx).expect("bit index < 64");
        moves.push(Move {
            from,
            to,
            promotion,
        });
    }
}

fn push_pawn(from: Square, to: Square, promo_rank: u8, moves: &mut Vec<Move>) {
    if to.rank() == promo_rank {
        for kind in [
            PieceKind::Queen,
            PieceKind::Rook,
            PieceKind::Bishop,
            PieceKind::Knight,
        ] {
            moves.push(Move {
                from,
                to,
                promotion: Some(kind),
            });
        }
    } else {
        moves.push(Move {
            from,
            to,
            promotion: None,
        });
    }
}

pub(crate) fn offset(square: Square, df: i8, dr: i8) -> Option<Square> {
    let file = square.file() as i8 + df;
    let rank = square.rank() as i8 + dr;
    if !(0..=7).contains(&file) || !(0..=7).contains(&rank) {
        return None;
    }
    Square::from_file_rank(file as u8, rank as u8)
}
