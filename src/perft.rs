//! Perft: legal-move tree leaf enumeration (CV-105).

use crate::board::Position;

impl Position {
    /// Count leaf nodes of the legal-move tree to `depth`.
    ///
    /// Depth 0 returns 1 (the current position). Only legal moves are expanded.
    /// Draw claims (repetition, fifty-move, insufficient material) are ignored,
    /// matching the published reference table in `docs/perft-table.md`.
    pub fn perft(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let moves = self.legal_moves();
        if depth == 1 {
            return moves.len() as u64;
        }
        let mut nodes = 0u64;
        for m in moves {
            // Moves from `legal_moves` are already king-safe.
            nodes += self.apply_move_unchecked(m).perft(depth - 1);
        }
        nodes
    }
}
