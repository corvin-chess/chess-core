//! 64-bit occupancy masks aligned with [`super::Square`] indexing.

use super::Square;

/// Set of squares packed into a `u64`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);

    pub const fn from_square(square: Square) -> Bitboard {
        Bitboard(1u64 << square.index())
    }

    pub const fn contains(self, square: Square) -> bool {
        (self.0 & (1u64 << square.index())) != 0
    }

    pub fn set(&mut self, square: Square) {
        self.0 |= 1u64 << square.index();
    }

    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1u64 << square.index());
    }

    pub const fn popcount(self) -> u32 {
        self.0.count_ones()
    }

    pub const fn union(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }

    pub const fn intersect(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }

    /// Lowest set square, or `None` when empty.
    pub fn first(self) -> Option<Square> {
        if self.0 == 0 {
            return None;
        }
        Square::from_index(self.0.trailing_zeros() as u8)
    }

    /// Iterates set squares from low index to high.
    pub fn squares(self) -> Squares {
        Squares { bits: self.0 }
    }
}

/// Iterator over squares set in a bitboard.
pub struct Squares {
    bits: u64,
}

impl Iterator for Squares {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.bits == 0 {
            return None;
        }
        let index = self.bits.trailing_zeros() as u8;
        self.bits &= self.bits - 1;
        Square::from_index(index)
    }
}
