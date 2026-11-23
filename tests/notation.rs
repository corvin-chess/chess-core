//! CV-103: UCI / SAN / PGN helper acceptance tests (written before implementation).

use corvin_chess_core::{
    decode_movetext, encode_movetext, Color, Move, Piece, PieceKind, Position, SanError, Square,
};

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

fn piece(color: Color, kind: PieceKind) -> Piece {
    Piece { color, kind }
}

#[test]
fn uci_round_trips_quiet_and_promotion() {
    let e4 = quiet(Square::E2, Square::E4);
    assert_eq!(e4.to_uci(), "e2e4");
    assert_eq!(Move::from_uci("e2e4").expect("uci"), e4);

    let q = promo(Square::E7, Square::E8, PieceKind::Queen);
    assert_eq!(q.to_uci(), "e7e8q");
    assert_eq!(Move::from_uci("e7e8q").expect("promo"), q);

    let n = promo(Square::A7, Square::B8, PieceKind::Knight);
    assert_eq!(n.to_uci(), "a7b8n");
    assert_eq!(Move::from_uci("a7b8n").expect("promo n"), n);
}

#[test]
fn uci_rejects_malformed() {
    assert!(Move::from_uci("").is_err());
    assert!(Move::from_uci("e2e").is_err());
    assert!(Move::from_uci("e2e9").is_err());
    assert!(Move::from_uci("e2e4x").is_err());
    assert!(Move::from_uci("e7e8p").is_err());
}

#[test]
fn san_starting_moves() {
    let pos = Position::starting();
    assert_eq!(pos.to_san(quiet(Square::E2, Square::E4)).expect("e4"), "e4");
    assert_eq!(
        pos.to_san(quiet(Square::G1, Square::F3)).expect("Nf3"),
        "Nf3"
    );
    assert_eq!(
        pos.from_san("e4").expect("parse e4"),
        quiet(Square::E2, Square::E4)
    );
    assert_eq!(
        pos.from_san("Nf3").expect("parse Nf3"),
        quiet(Square::G1, Square::F3)
    );
}

#[test]
fn san_file_disambiguation_for_knights() {
    // Knights on b1 and f3; d2 empty so both can play there.
    let pos = Position::from_fen("4k3/8/8/8/8/5N2/8/RN2K3 w - - 0 1").expect("fen");
    let nbd2 = quiet(Square::B1, Square::D2);
    let nfd2 = quiet(Square::F3, Square::D2);
    assert!(pos.legal_moves().contains(&nbd2));
    assert!(pos.legal_moves().contains(&nfd2));
    assert_eq!(pos.to_san(nbd2).expect("Nbd2"), "Nbd2");
    assert_eq!(pos.to_san(nfd2).expect("Nfd2"), "Nfd2");
    assert_eq!(pos.from_san("Nbd2").expect("parse Nbd2"), nbd2);
    assert_eq!(pos.from_san("Nfd2").expect("parse Nfd2"), nfd2);
    assert!(matches!(pos.from_san("Nd2"), Err(SanError::Ambiguous)));
}

#[test]
fn san_rank_disambiguation_for_rooks() {
    // Rooks on a1 and a4; both can play to a2. Same file forces rank letter.
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::King));
    pos.set_piece(Square::A1, piece(Color::White, PieceKind::Rook));
    pos.set_piece(Square::A4, piece(Color::White, PieceKind::Rook));
    pos.set_side_to_move(Color::White);

    let r1a2 = quiet(Square::A1, Square::A2);
    let r4a2 = quiet(Square::A4, Square::A2);
    assert_eq!(pos.to_san(r1a2).expect("R1a2"), "R1a2");
    assert_eq!(pos.to_san(r4a2).expect("R4a2"), "R4a2");
    assert_eq!(pos.from_san("R1a2").expect("parse R1a2"), r1a2);
    assert_eq!(pos.from_san("R4a2").expect("parse R4a2"), r4a2);
}

#[test]
fn san_file_and_rank_disambiguation() {
    // Knights on b1, d1, b5 all reach c3: b1 needs both file and rank (file shared with b5, rank with d1).
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::King));
    pos.set_piece(Square::B1, piece(Color::White, PieceKind::Knight));
    pos.set_piece(Square::D1, piece(Color::White, PieceKind::Knight));
    pos.set_piece(Square::B5, piece(Color::White, PieceKind::Knight));
    pos.set_side_to_move(Color::White);

    let from_b1 = quiet(Square::B1, Square::C3);
    let from_d1 = quiet(Square::D1, Square::C3);
    let from_b5 = quiet(Square::B5, Square::C3);
    assert!(pos.legal_moves().contains(&from_b1));
    assert!(pos.legal_moves().contains(&from_d1));
    assert!(pos.legal_moves().contains(&from_b5));

    assert_eq!(pos.to_san(from_b1).expect("Nb1c3"), "Nb1c3");
    assert_eq!(pos.to_san(from_d1).expect("Ndc3"), "Ndc3");
    assert_eq!(pos.to_san(from_b5).expect("N5c3"), "N5c3");
    assert_eq!(pos.from_san("Nb1c3").expect("parse"), from_b1);
    assert_eq!(pos.from_san("Ndc3").expect("parse"), from_d1);
    assert_eq!(pos.from_san("N5c3").expect("parse"), from_b5);
}

#[test]
fn san_castling_capture_promotion_and_check() {
    // White castling kingside available.
    let castle_fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
    let pos = Position::from_fen(castle_fen).expect("castle fen");
    assert_eq!(
        pos.to_san(quiet(Square::E1, Square::G1)).expect("O-O"),
        "O-O"
    );
    assert_eq!(
        pos.to_san(quiet(Square::E1, Square::C1)).expect("O-O-O"),
        "O-O-O"
    );
    assert_eq!(
        pos.from_san("O-O").expect("parse O-O"),
        quiet(Square::E1, Square::G1)
    );
    assert_eq!(
        pos.from_san("0-0-0").expect("parse 0-0-0"),
        quiet(Square::E1, Square::C1)
    );

    // Pawn capture: white pawn e4 takes black pawn d5.
    let cap = Position::from_fen("4k3/8/8/3p4/4P3/8/8/4K3 w - - 0 1").expect("cap");
    assert_eq!(
        cap.to_san(quiet(Square::E4, Square::D5)).expect("exd5"),
        "exd5"
    );
    assert_eq!(
        cap.from_san("exd5").expect("parse exd5"),
        quiet(Square::E4, Square::D5)
    );

    // Promotion with check: white pawn e7 to e8=Q checking black king e8? King on e8 blocks.
    // Use king on d8 so Qe8+ is check.
    let promo_pos = Position::from_fen("3k4/4P3/8/8/8/8/8/4K3 w - - 0 1").expect("promo");
    let q = promo(Square::E7, Square::E8, PieceKind::Queen);
    assert_eq!(promo_pos.to_san(q).expect("e8=Q+"), "e8=Q+");
    assert_eq!(promo_pos.from_san("e8=Q+").expect("parse promo"), q);

    // Mate: queen to h7, protected by the white king on g6.
    let mate = Position::from_fen("7k/5Q2/6K1/8/8/8/8/8 w - - 0 1").expect("mate");
    let qh7 = quiet(Square::F7, Square::H7);
    assert!(mate.legal_moves().contains(&qh7));
    assert_eq!(mate.to_san(qh7).expect("Qh7#"), "Qh7#");
}

#[test]
fn san_rejects_illegal_and_ambiguous_forms() {
    let pos = Position::starting();
    assert!(matches!(pos.from_san("e5"), Err(SanError::Illegal)));
    assert!(matches!(pos.from_san(""), Err(SanError::Invalid)));
    assert!(pos.to_san(quiet(Square::E2, Square::E5)).is_err());
    // Spurious capture marker must not match a quiet knight jump.
    assert!(matches!(pos.from_san("Nxf3"), Err(SanError::Illegal)));
}

#[test]
fn pgn_movetext_round_trips_opening() {
    let start = Position::starting();
    let moves = [
        quiet(Square::E2, Square::E4),
        quiet(Square::E7, Square::E5),
        quiet(Square::G1, Square::F3),
        quiet(Square::B8, Square::C6),
    ];
    let text = encode_movetext(&start, &moves).expect("encode");
    assert_eq!(text, "1. e4 e5 2. Nf3 Nc6");
    let decoded = decode_movetext(&start, &text).expect("decode");
    assert_eq!(decoded, moves);

    // Tolerates missing numbers and alternate spacing.
    let loose = decode_movetext(&start, "e4 e5 Nf3 Nc6").expect("loose");
    assert_eq!(loose, moves);
}

#[test]
fn pgn_movetext_rejects_illegal_sequence() {
    let start = Position::starting();
    assert!(decode_movetext(&start, "1. e4 e4").is_err());
}
