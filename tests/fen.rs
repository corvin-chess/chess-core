//! CV-102: FEN load/store round-trip and rejection tests (written before implementation).

use corvin_chess_core::{Color, FenError, Piece, PieceKind, Position, Square};

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const AFTER_E4: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
const AFTER_E4_C5_NF3: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";

#[test]
fn starting_fen_round_trips() {
    let pos = Position::from_fen(START_FEN).expect("start fen");
    assert_eq!(pos, Position::starting());
    assert_eq!(pos.to_fen(), START_FEN);
}

#[test]
fn sample_positions_round_trip_exactly() {
    for fen in [START_FEN, AFTER_E4, AFTER_E4_C5_NF3] {
        let pos = Position::from_fen(fen).unwrap_or_else(|e| panic!("parse {fen}: {e}"));
        assert_eq!(pos.to_fen(), fen, "canonical round-trip for {fen}");
        let again = Position::from_fen(&pos.to_fen()).expect("reparse");
        assert_eq!(again, pos);
    }
}

#[test]
fn after_e4_fields_are_parsed() {
    let pos = Position::from_fen(AFTER_E4).expect("after e4");
    assert_eq!(pos.side_to_move(), Color::Black);
    assert_eq!(pos.ep_square(), Some(Square::E3));
    assert_eq!(pos.halfmove_clock(), 0);
    assert_eq!(pos.fullmove_number(), 1);
    assert_eq!(
        pos.piece_at(Square::E4),
        Some(Piece {
            color: Color::White,
            kind: PieceKind::Pawn,
        })
    );
    assert!(pos.piece_at(Square::E2).is_none());
}

#[test]
fn no_castling_and_clocks_serialize() {
    let fen = "4k3/8/8/8/8/8/8/4K3 b - - 12 40";
    let pos = Position::from_fen(fen).expect("kings only");
    assert!(!pos.castling().white_king);
    assert!(!pos.castling().white_queen);
    assert!(!pos.castling().black_king);
    assert!(!pos.castling().black_queen);
    assert_eq!(pos.ep_square(), None);
    assert_eq!(pos.side_to_move(), Color::Black);
    assert_eq!(pos.halfmove_clock(), 12);
    assert_eq!(pos.fullmove_number(), 40);
    assert_eq!(pos.to_fen(), fen);
}

#[test]
fn partial_castling_keeps_kq_order() {
    let fen = "r3k2r/8/8/8/8/8/8/R3K2R w Kq - 0 1";
    let pos = Position::from_fen(fen).expect("partial castling");
    assert!(pos.castling().white_king);
    assert!(!pos.castling().white_queen);
    assert!(!pos.castling().black_king);
    assert!(pos.castling().black_queen);
    assert_eq!(pos.to_fen(), fen);
}

#[test]
fn built_position_round_trips_through_fen() {
    let mut pos = Position::empty();
    pos.set_piece(
        Square::E1,
        Piece {
            color: Color::White,
            kind: PieceKind::King,
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
        Square::D5,
        Piece {
            color: Color::White,
            kind: PieceKind::Knight,
        },
    );
    pos.set_side_to_move(Color::White);
    pos.set_halfmove_clock(3);
    pos.set_fullmove_number(7);

    let fen = pos.to_fen();
    assert_eq!(fen, "4k3/8/8/3N4/8/8/8/4K3 w - - 3 7");
    assert_eq!(Position::from_fen(&fen).expect("reparse"), pos);
}

#[test]
fn rejects_wrong_field_count() {
    assert!(matches!(
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0"),
        Err(FenError::FieldCount)
    ));
    assert!(matches!(
        Position::from_fen("too many fields a b c d e f g"),
        Err(FenError::FieldCount)
    ));
}

#[test]
fn rejects_bad_placement() {
    assert!(matches!(
        Position::from_fen("8/8/8/8/8/8/8 w KQkq - 0 1"),
        Err(FenError::Placement)
    ));
    assert!(matches!(
        Position::from_fen("9/8/8/8/8/8/8/8 w - - 0 1"),
        Err(FenError::Placement)
    ));
    assert!(matches!(
        Position::from_fen("4k3/8/8/8/8/8/8/4K2X w - - 0 1"),
        Err(FenError::Placement)
    ));
    assert!(matches!(
        Position::from_fen("44/8/8/8/8/8/8/8 w - - 0 1"),
        Err(FenError::Placement)
    ));
}

#[test]
fn rejects_non_ascii_ep_and_placement() {
    assert!(matches!(
        Position::from_fen("8/8/8/8/8/8/8/8 w - \u{0161}3 0 1"),
        Err(FenError::EnPassant)
    ));
    assert!(matches!(
        Position::from_fen("\u{FF18}/8/8/8/8/8/8/8 w - - 0 1"),
        Err(FenError::Placement)
    ));
}

#[test]
fn rejects_bad_side_castling_ep_and_clocks() {
    assert!(matches!(
        Position::from_fen("8/8/8/8/8/8/8/8 x - - 0 1"),
        Err(FenError::SideToMove)
    ));
    assert!(matches!(
        Position::from_fen("8/8/8/8/8/8/8/8 w KX - 0 1"),
        Err(FenError::Castling)
    ));
    assert!(matches!(
        Position::from_fen("8/8/8/8/8/8/8/8 w - e9 0 1"),
        Err(FenError::EnPassant)
    ));
    assert!(matches!(
        Position::from_fen("8/8/8/8/8/8/8/8 w - e4 0 1"),
        Err(FenError::EnPassant)
    ));
    assert!(matches!(
        Position::from_fen("8/8/8/8/8/8/8/8 w - - -1 1"),
        Err(FenError::HalfmoveClock)
    ));
    assert!(matches!(
        Position::from_fen("8/8/8/8/8/8/8/8 w - - 0 0"),
        Err(FenError::FullmoveNumber)
    ));
}
