//! CV-100: board representation acceptance tests (written before implementation).

use corvin_chess_core::{Bitboard, CastlingRights, Color, Piece, PieceKind, Position, Square};

#[test]
fn square_index_round_trips_a1_through_h8() {
    assert_eq!(Square::A1.index(), 0);
    assert_eq!(Square::H1.index(), 7);
    assert_eq!(Square::A8.index(), 56);
    assert_eq!(Square::H8.index(), 63);

    for index in 0u8..64 {
        let square = Square::from_index(index).expect("0..63 valid");
        assert_eq!(square.index(), index);
        assert_eq!(
            Square::from_file_rank(square.file(), square.rank()),
            Some(square)
        );
    }
}

#[test]
fn square_rejects_out_of_range_index_and_file_rank() {
    assert!(Square::from_index(64).is_none());
    assert!(Square::from_file_rank(8, 0).is_none());
    assert!(Square::from_file_rank(0, 8).is_none());
}

#[test]
fn color_opposite_swaps_sides() {
    assert_eq!(Color::White.opposite(), Color::Black);
    assert_eq!(Color::Black.opposite(), Color::White);
}

#[test]
fn bitboard_set_contains_clear_and_popcount() {
    let mut bb = Bitboard::EMPTY;
    assert_eq!(bb.popcount(), 0);
    assert!(!bb.contains(Square::E4));

    bb.set(Square::E4);
    assert!(bb.contains(Square::E4));
    assert_eq!(bb.popcount(), 1);
    assert_eq!(bb, Bitboard::from_square(Square::E4));

    bb.set(Square::D5);
    assert_eq!(bb.popcount(), 2);

    bb.clear(Square::E4);
    assert!(!bb.contains(Square::E4));
    assert!(bb.contains(Square::D5));
    assert_eq!(bb.popcount(), 1);
}

#[test]
fn empty_position_has_no_pieces_and_default_clocks() {
    let pos = Position::empty();
    assert_eq!(pos.side_to_move(), Color::White);
    assert_eq!(pos.ep_square(), None);
    assert_eq!(pos.halfmove_clock(), 0);
    assert_eq!(pos.fullmove_number(), 1);
    assert_eq!(pos.occupied().popcount(), 0);
    assert_eq!(pos.castling(), CastlingRights::none());
    assert!(pos.piece_at(Square::E1).is_none());
}

#[test]
fn starting_position_has_thirty_two_pieces_and_white_to_move() {
    let pos = Position::starting();
    assert_eq!(pos.side_to_move(), Color::White);
    assert_eq!(pos.ep_square(), None);
    assert_eq!(pos.halfmove_clock(), 0);
    assert_eq!(pos.fullmove_number(), 1);
    assert_eq!(pos.occupied().popcount(), 32);
    assert_eq!(pos.pieces(Color::White).popcount(), 16);
    assert_eq!(pos.pieces(Color::Black).popcount(), 16);
    assert_eq!(
        pos.castling(),
        CastlingRights {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        }
    );
}

#[test]
fn starting_position_places_kings_queens_and_pawns() {
    let pos = Position::starting();

    assert_eq!(
        pos.piece_at(Square::E1),
        Some(Piece {
            color: Color::White,
            kind: PieceKind::King,
        })
    );
    assert_eq!(
        pos.piece_at(Square::E8),
        Some(Piece {
            color: Color::Black,
            kind: PieceKind::King,
        })
    );
    assert_eq!(
        pos.piece_at(Square::D1),
        Some(Piece {
            color: Color::White,
            kind: PieceKind::Queen,
        })
    );
    assert_eq!(
        pos.piece_at(Square::D8),
        Some(Piece {
            color: Color::Black,
            kind: PieceKind::Queen,
        })
    );

    for file in 0u8..8 {
        let white_pawn = Square::from_file_rank(file, 1).unwrap();
        let black_pawn = Square::from_file_rank(file, 6).unwrap();
        assert_eq!(
            pos.piece_at(white_pawn),
            Some(Piece {
                color: Color::White,
                kind: PieceKind::Pawn,
            })
        );
        assert_eq!(
            pos.piece_at(black_pawn),
            Some(Piece {
                color: Color::Black,
                kind: PieceKind::Pawn,
            })
        );
    }

    assert_eq!(pos.pieces_of(Color::White, PieceKind::Pawn).popcount(), 8);
    assert_eq!(pos.pieces_of(Color::Black, PieceKind::Rook).popcount(), 2);
    assert!(pos.piece_at(Square::E4).is_none());
}

#[test]
fn set_and_clear_piece_update_occupancy() {
    let mut pos = Position::empty();
    let knight = Piece {
        color: Color::White,
        kind: PieceKind::Knight,
    };
    pos.set_piece(Square::B1, knight);
    assert_eq!(pos.piece_at(Square::B1), Some(knight));
    assert_eq!(pos.occupied().popcount(), 1);
    assert_eq!(pos.pieces_of(Color::White, PieceKind::Knight).popcount(), 1);

    pos.clear_square(Square::B1);
    assert!(pos.piece_at(Square::B1).is_none());
    assert_eq!(pos.occupied().popcount(), 0);
}
