//! Attack bitboards for each piece kind (Corvin-original tables).

use super::offset;
use crate::board::{Bitboard, Color, Square};

const KNIGHT_DELTAS: [(i8, i8); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

const KING_DELTAS: [(i8, i8); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub(crate) fn knight_attacks(square: Square) -> u64 {
    let mut mask = 0u64;
    for &(df, dr) in &KNIGHT_DELTAS {
        if let Some(to) = offset(square, df, dr) {
            mask |= 1u64 << to.index();
        }
    }
    mask
}

pub(crate) fn king_attacks(square: Square) -> u64 {
    let mut mask = 0u64;
    for &(df, dr) in &KING_DELTAS {
        if let Some(to) = offset(square, df, dr) {
            mask |= 1u64 << to.index();
        }
    }
    mask
}

/// Squares attacked by a pawn of `color` standing on `square`.
pub(crate) fn pawn_attacks_from(square: Square, color: Color) -> u64 {
    let forward = match color {
        Color::White => 1i8,
        Color::Black => -1i8,
    };
    let mut mask = 0u64;
    for &df in &[-1i8, 1i8] {
        if let Some(to) = offset(square, df, forward) {
            mask |= 1u64 << to.index();
        }
    }
    mask
}

/// Origins that attack `square` when interpreted as pawn attacks of `looking_as`.
/// Call with `by.opposite()` so the mask equals squares occupied by `by` pawns that check `square`.
pub(crate) fn pawn_attacks(square: Square, looking_as: Color) -> u64 {
    pawn_attacks_from(square, looking_as)
}

pub(crate) fn bishop_attacks(square: Square, occupied: Bitboard) -> u64 {
    sliding(square, occupied, &BISHOP_DIRS)
}

pub(crate) fn rook_attacks(square: Square, occupied: Bitboard) -> u64 {
    sliding(square, occupied, &ROOK_DIRS)
}

pub(crate) fn queen_attacks(square: Square, occupied: Bitboard) -> u64 {
    bishop_attacks(square, occupied) | rook_attacks(square, occupied)
}

fn sliding(square: Square, occupied: Bitboard, dirs: &[(i8, i8)]) -> u64 {
    let mut mask = 0u64;
    for &(df, dr) in dirs {
        mask |= sliding_on_ray(square, occupied, df, dr);
    }
    mask
}

pub(crate) fn sliding_on_ray(square: Square, occupied: Bitboard, df: i8, dr: i8) -> u64 {
    let mut mask = 0u64;
    let mut cur = square;
    while let Some(next) = offset(cur, df, dr) {
        mask |= 1u64 << next.index();
        if occupied.contains(next) {
            break;
        }
        cur = next;
    }
    mask
}
