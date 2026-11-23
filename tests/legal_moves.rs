//! CV-101: legal move generation acceptance tests (written before implementation).

use corvin_chess_core::{CastlingRights, Color, Move, Piece, PieceKind, Position, Square};

fn piece(color: Color, kind: PieceKind) -> Piece {
    Piece { color, kind }
}

fn quiet(from: Square, to: Square) -> Move {
    Move {
        from,
        to,
        promotion: None,
    }
}

fn promo(from: Square, to: Square, kind: PieceKind) -> Move {
    Move {
        from,
        to,
        promotion: Some(kind),
    }
}

fn has_move(moves: &[Move], expected: Move) -> bool {
    moves.contains(&expected)
}

#[test]
fn starting_position_has_twenty_legal_moves() {
    let pos = Position::starting();
    let moves = pos.legal_moves();
    assert_eq!(moves.len(), 20);
    assert!(has_move(&moves, quiet(Square::E2, Square::E4)));
    assert!(has_move(&moves, quiet(Square::G1, Square::F3)));
    assert!(!has_move(&moves, quiet(Square::E1, Square::E2)));
}

#[test]
fn pinned_piece_cannot_leave_the_pin_line() {
    // White king e1, white knight e2 pinned by black rook e8; black king a8.
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::E2, piece(Color::White, PieceKind::Knight));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::Rook));
    pos.set_piece(Square::A8, piece(Color::Black, PieceKind::King));
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    assert!(
        !has_move(&moves, quiet(Square::E2, Square::C3)),
        "pinned knight must not jump off the e-file"
    );
    assert!(
        !has_move(&moves, quiet(Square::E2, Square::D4)),
        "pinned knight must not leave the pin"
    );
    // King can step aside.
    assert!(has_move(&moves, quiet(Square::E1, Square::D1)));
    assert!(has_move(&moves, quiet(Square::E1, Square::F1)));
}

#[test]
fn in_check_only_evasions_are_legal() {
    // White king e1 checked by black rook e8; white can capture with knight or block/move king.
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::B1, piece(Color::White, PieceKind::Knight));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::Rook));
    pos.set_piece(Square::A8, piece(Color::Black, PieceKind::King));
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    assert!(pos.in_check());
    // Capturing the checker with the knight is illegal (knight is not attacking e8).
    assert!(!has_move(&moves, quiet(Square::B1, Square::A3)));
    // King steps off the file.
    assert!(has_move(&moves, quiet(Square::E1, Square::D1)));
    assert!(has_move(&moves, quiet(Square::E1, Square::F1)));
    assert!(has_move(&moves, quiet(Square::E1, Square::D2)));
    assert!(has_move(&moves, quiet(Square::E1, Square::F2)));
    // Staying on e-file remains in check.
    assert!(!has_move(&moves, quiet(Square::E1, Square::E2)));
}

#[test]
fn pawn_check_is_detected_and_must_be_resolved() {
    // Black pawn on d2 checks white king on e1.
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::D2, piece(Color::Black, PieceKind::Pawn));
    pos.set_piece(Square::A8, piece(Color::Black, PieceKind::King));
    pos.set_piece(Square::H1, piece(Color::White, PieceKind::Rook));
    pos.set_side_to_move(Color::White);

    assert!(pos.in_check());
    assert!(pos.is_square_attacked(Square::E1, Color::Black));
    let moves = pos.legal_moves();
    // Capturing the checking pawn with the king is legal.
    assert!(has_move(&moves, quiet(Square::E1, Square::D2)));
    // Rook cannot ignore the check.
    assert!(!has_move(&moves, quiet(Square::H1, Square::H8)));
}

#[test]
fn king_cannot_move_into_check() {
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::A2, piece(Color::Black, PieceKind::Rook));
    pos.set_piece(Square::A8, piece(Color::Black, PieceKind::King));
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    assert!(!has_move(&moves, quiet(Square::E1, Square::E2)));
    assert!(has_move(&moves, quiet(Square::E1, Square::D1)));
    assert!(has_move(&moves, quiet(Square::E1, Square::F1)));
}

#[test]
fn white_can_castle_kingside_when_path_clear_and_safe() {
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::H1, piece(Color::White, PieceKind::Rook));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::King));
    pos.set_castling(CastlingRights {
        white_king: true,
        white_queen: false,
        black_king: false,
        black_queen: false,
    });
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    assert!(has_move(&moves, quiet(Square::E1, Square::G1)));
}

#[test]
fn white_cannot_castle_kingside_through_check() {
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::H1, piece(Color::White, PieceKind::Rook));
    pos.set_piece(Square::F3, piece(Color::Black, PieceKind::Rook));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::King));
    pos.set_castling(CastlingRights {
        white_king: true,
        white_queen: false,
        black_king: false,
        black_queen: false,
    });
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    assert!(!has_move(&moves, quiet(Square::E1, Square::G1)));
}

#[test]
fn white_can_castle_queenside_when_path_clear_and_safe() {
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::A1, piece(Color::White, PieceKind::Rook));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::King));
    pos.set_castling(CastlingRights {
        white_king: false,
        white_queen: true,
        black_king: false,
        black_queen: false,
    });
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    assert!(has_move(&moves, quiet(Square::E1, Square::C1)));
}

#[test]
fn en_passant_capture_is_generated_and_removes_pawn() {
    // White pawn d5, black pawn e5 just double-pushed; EP target e6.
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::King));
    pos.set_piece(Square::D5, piece(Color::White, PieceKind::Pawn));
    pos.set_piece(Square::E5, piece(Color::Black, PieceKind::Pawn));
    pos.set_ep_square(Some(Square::E6));
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    let ep = quiet(Square::D5, Square::E6);
    assert!(has_move(&moves, ep));

    let after = pos.try_make_move(ep).expect("EP must be legal");
    assert!(after.piece_at(Square::E6).is_some());
    assert!(after.piece_at(Square::E5).is_none());
    assert_eq!(after.ep_square(), None);
    assert_eq!(after.side_to_move(), Color::Black);
}

#[test]
fn en_passant_that_exposes_king_is_illegal() {
    // Classic EP pin: white king e1, white pawn e5, black pawn d5, black rook e8; EP d6.
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::E5, piece(Color::White, PieceKind::Pawn));
    pos.set_piece(Square::D5, piece(Color::Black, PieceKind::Pawn));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::Rook));
    pos.set_piece(Square::A8, piece(Color::Black, PieceKind::King));
    pos.set_ep_square(Some(Square::D6));
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    assert!(!has_move(&moves, quiet(Square::E5, Square::D6)));
}

#[test]
fn pawn_promotions_generate_four_pieces() {
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::H8, piece(Color::Black, PieceKind::King));
    pos.set_piece(Square::A7, piece(Color::White, PieceKind::Pawn));
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    assert!(has_move(
        &moves,
        promo(Square::A7, Square::A8, PieceKind::Queen)
    ));
    assert!(has_move(
        &moves,
        promo(Square::A7, Square::A8, PieceKind::Rook)
    ));
    assert!(has_move(
        &moves,
        promo(Square::A7, Square::A8, PieceKind::Bishop)
    ));
    assert!(has_move(
        &moves,
        promo(Square::A7, Square::A8, PieceKind::Knight)
    ));
    assert!(!has_move(&moves, quiet(Square::A7, Square::A8)));
}

#[test]
fn promotion_capture_is_legal() {
    let mut pos = Position::empty();
    pos.set_piece(Square::H1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::A1, piece(Color::Black, PieceKind::King));
    pos.set_piece(Square::B7, piece(Color::White, PieceKind::Pawn));
    pos.set_piece(Square::A8, piece(Color::Black, PieceKind::Knight));
    pos.set_side_to_move(Color::White);

    let moves = pos.legal_moves();
    assert!(has_move(
        &moves,
        promo(Square::B7, Square::A8, PieceKind::Queen)
    ));
}

#[test]
fn try_make_move_rejects_illegal_moves() {
    let pos = Position::starting();
    assert!(pos.try_make_move(quiet(Square::E2, Square::E5)).is_none());
    assert!(pos.try_make_move(quiet(Square::E1, Square::E2)).is_none());
}
