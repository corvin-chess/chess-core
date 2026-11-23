//! Position: bitboard occupancy plus side-to-move and game-state fields.

use super::{Bitboard, Color, Piece, PieceKind, Square};

/// Castling rights for both sides.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CastlingRights {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl CastlingRights {
    pub const fn none() -> CastlingRights {
        CastlingRights {
            white_king: false,
            white_queen: false,
            black_king: false,
            black_queen: false,
        }
    }

    pub const fn all() -> CastlingRights {
        CastlingRights {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        }
    }
}

/// A chess position without move generation (CV-100).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    by_color: [Bitboard; 2],
    by_kind: [Bitboard; 6],
    side_to_move: Color,
    castling: CastlingRights,
    ep_square: Option<Square>,
    halfmove_clock: u16,
    fullmove_number: u16,
}

impl Position {
    /// Empty board, White to move, no castling, move 1.
    pub fn empty() -> Position {
        Position {
            by_color: [Bitboard::EMPTY; 2],
            by_kind: [Bitboard::EMPTY; 6],
            side_to_move: Color::White,
            castling: CastlingRights::none(),
            ep_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Standard starting position.
    pub fn starting() -> Position {
        let mut pos = Position::empty();
        pos.castling = CastlingRights::all();

        // White back rank.
        pos.set_piece(
            Square::A1,
            Piece {
                color: Color::White,
                kind: PieceKind::Rook,
            },
        );
        pos.set_piece(
            Square::B1,
            Piece {
                color: Color::White,
                kind: PieceKind::Knight,
            },
        );
        pos.set_piece(
            Square::C1,
            Piece {
                color: Color::White,
                kind: PieceKind::Bishop,
            },
        );
        pos.set_piece(
            Square::D1,
            Piece {
                color: Color::White,
                kind: PieceKind::Queen,
            },
        );
        pos.set_piece(
            Square::E1,
            Piece {
                color: Color::White,
                kind: PieceKind::King,
            },
        );
        pos.set_piece(
            Square::F1,
            Piece {
                color: Color::White,
                kind: PieceKind::Bishop,
            },
        );
        pos.set_piece(
            Square::G1,
            Piece {
                color: Color::White,
                kind: PieceKind::Knight,
            },
        );
        pos.set_piece(
            Square::H1,
            Piece {
                color: Color::White,
                kind: PieceKind::Rook,
            },
        );

        // Black back rank.
        pos.set_piece(
            Square::A8,
            Piece {
                color: Color::Black,
                kind: PieceKind::Rook,
            },
        );
        pos.set_piece(
            Square::B8,
            Piece {
                color: Color::Black,
                kind: PieceKind::Knight,
            },
        );
        pos.set_piece(
            Square::C8,
            Piece {
                color: Color::Black,
                kind: PieceKind::Bishop,
            },
        );
        pos.set_piece(
            Square::D8,
            Piece {
                color: Color::Black,
                kind: PieceKind::Queen,
            },
        );
        pos.set_piece(
            Square::E8,
            Piece {
                color: Color::Black,
                kind: PieceKind::King,
            },
        );
        pos.set_piece(
            Square::F8,
            Piece {
                color: Color::Black,
                kind: PieceKind::Bishop,
            },
        );
        pos.set_piece(
            Square::G8,
            Piece {
                color: Color::Black,
                kind: PieceKind::Knight,
            },
        );
        pos.set_piece(
            Square::H8,
            Piece {
                color: Color::Black,
                kind: PieceKind::Rook,
            },
        );

        for file in 0u8..8 {
            let white_pawn = Square::from_file_rank(file, 1).expect("file in range");
            let black_pawn = Square::from_file_rank(file, 6).expect("file in range");
            pos.set_piece(
                white_pawn,
                Piece {
                    color: Color::White,
                    kind: PieceKind::Pawn,
                },
            );
            pos.set_piece(
                black_pawn,
                Piece {
                    color: Color::Black,
                    kind: PieceKind::Pawn,
                },
            );
        }

        pos
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn castling(&self) -> CastlingRights {
        self.castling
    }

    pub fn ep_square(&self) -> Option<Square> {
        self.ep_square
    }

    pub fn halfmove_clock(&self) -> u16 {
        self.halfmove_clock
    }

    pub fn fullmove_number(&self) -> u16 {
        self.fullmove_number
    }

    pub fn occupied(&self) -> Bitboard {
        self.by_color[Color::White.index()].union(self.by_color[Color::Black.index()])
    }

    pub fn pieces(&self, color: Color) -> Bitboard {
        self.by_color[color.index()]
    }

    pub fn pieces_of(&self, color: Color, kind: PieceKind) -> Bitboard {
        self.by_color[color.index()].intersect(self.by_kind[kind.index()])
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        if !self.occupied().contains(square) {
            return None;
        }
        let color = if self.by_color[Color::White.index()].contains(square) {
            Color::White
        } else {
            Color::Black
        };
        for kind in PieceKind::ALL {
            if self.by_kind[kind.index()].contains(square) {
                return Some(Piece { color, kind });
            }
        }
        // Occupied but no kind bit: representation invariant broken.
        None
    }

    /// Places `piece` on `square`, replacing any occupant.
    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.clear_square(square);
        self.by_color[piece.color.index()].set(square);
        self.by_kind[piece.kind.index()].set(square);
    }

    /// Removes any piece from `square`.
    pub fn clear_square(&mut self, square: Square) {
        for color in [Color::White, Color::Black] {
            self.by_color[color.index()].clear(square);
        }
        for kind in PieceKind::ALL {
            self.by_kind[kind.index()].clear(square);
        }
    }
}
