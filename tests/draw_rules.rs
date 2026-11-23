//! CV-104: repetition, fifty-move / seventy-five, insufficient material (tests first).

use corvin_chess_core::{Color, Move, Piece, PieceKind, Position, RepetitionKey, Square};

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

fn kings_only() -> Position {
    let mut pos = Position::empty();
    pos.set_piece(Square::E1, piece(Color::White, PieceKind::King));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::King));
    pos.set_side_to_move(Color::White);
    pos
}

fn play(pos: &Position, m: Move) -> Position {
    pos.try_make_move(m).expect("move must be legal")
}

#[test]
fn starting_position_is_not_insufficient_material() {
    assert!(!Position::starting().is_insufficient_material());
}

#[test]
fn king_versus_king_is_insufficient_material() {
    assert!(kings_only().is_insufficient_material());
}

#[test]
fn king_and_bishop_versus_king_is_insufficient() {
    let mut pos = kings_only();
    pos.set_piece(Square::C1, piece(Color::White, PieceKind::Bishop));
    assert!(pos.is_insufficient_material());
    assert!(!pos.has_mating_potential(Color::White));
    assert!(!pos.has_mating_potential(Color::Black));
}

#[test]
fn king_and_knight_versus_king_is_insufficient() {
    let mut pos = kings_only();
    pos.set_piece(Square::B1, piece(Color::White, PieceKind::Knight));
    assert!(pos.is_insufficient_material());
    assert!(!pos.has_mating_potential(Color::White));
}

#[test]
fn king_and_two_knights_versus_king_has_mating_potential() {
    // Helpmate is possible, so this is not a FIDE dead position by material.
    let mut pos = kings_only();
    pos.set_piece(Square::B1, piece(Color::White, PieceKind::Knight));
    pos.set_piece(Square::G1, piece(Color::White, PieceKind::Knight));
    assert!(!pos.is_insufficient_material());
    assert!(pos.has_mating_potential(Color::White));
}

#[test]
fn opposite_color_bishops_have_mating_potential() {
    let mut pos = kings_only();
    pos.set_piece(Square::C1, piece(Color::White, PieceKind::Bishop)); // light
    pos.set_piece(Square::C8, piece(Color::Black, PieceKind::Bishop)); // dark
    assert!(!pos.is_insufficient_material());
    assert!(pos.has_mating_potential(Color::White));
    assert!(pos.has_mating_potential(Color::Black));
}

#[test]
fn same_color_bishops_are_insufficient() {
    let mut pos = kings_only();
    pos.set_piece(Square::C1, piece(Color::White, PieceKind::Bishop)); // light
    pos.set_piece(Square::F8, piece(Color::Black, PieceKind::Bishop)); // light
    assert!(pos.is_insufficient_material());
    assert!(!pos.has_mating_potential(Color::White));
    assert!(!pos.has_mating_potential(Color::Black));
}

#[test]
fn lone_queen_has_mating_potential() {
    let mut pos = kings_only();
    pos.set_piece(Square::D1, piece(Color::White, PieceKind::Queen));
    assert!(!pos.is_insufficient_material());
    assert!(pos.has_mating_potential(Color::White));
    assert!(!pos.has_mating_potential(Color::Black));
}

#[test]
fn pawn_gives_mating_potential() {
    let mut pos = kings_only();
    pos.set_piece(Square::E2, piece(Color::White, PieceKind::Pawn));
    assert!(!pos.is_insufficient_material());
    assert!(pos.has_mating_potential(Color::White));
}

#[test]
fn fifty_move_claim_uses_halfmove_clock_100() {
    let mut pos = Position::starting();
    assert!(!pos.can_claim_fifty_move());
    assert!(!pos.is_seventy_five_move());
    pos.set_halfmove_clock(99);
    assert!(!pos.can_claim_fifty_move());
    pos.set_halfmove_clock(100);
    assert!(pos.can_claim_fifty_move());
    assert!(!pos.is_seventy_five_move());
    pos.set_halfmove_clock(150);
    assert!(pos.can_claim_fifty_move());
    assert!(pos.is_seventy_five_move());
}

#[test]
fn threefold_after_repeating_start_via_knights() {
    // 1.Nf3 Nf6 2.Ng1 Ng8 3.Nf3 Nf6 4.Ng1 Ng8 5.Nf3 — startpos appears thrice before fifth Nf3?
    // Occurrences of startpos: initial, after 2...Ng8, after 4...Ng8 => three times when Black
    // has just returned; White to move can claim before moving, or after 5.Nf3 the Nf3 position
    // appears for the third time with Black to move.
    let mut pos = Position::starting();
    let mut history: Vec<RepetitionKey> = Vec::new();

    let cycle = [
        quiet(Square::G1, Square::F3),
        quiet(Square::G8, Square::F6),
        quiet(Square::F3, Square::G1),
        quiet(Square::F6, Square::G8),
    ];

    // Initial startpos counts as occurrence 1 once we include current.
    assert_eq!(pos.repetition_count(&history), 1);
    assert!(!pos.is_threefold(&history));

    for _ in 0..2 {
        for m in cycle {
            history.push(pos.repetition_key());
            pos = play(&pos, m);
        }
    }
    // After two full cycles back to start: occurrences = initial + 2 returns = 3.
    assert_eq!(pos.repetition_count(&history), 3);
    assert!(pos.is_threefold(&history));
    assert!(!pos.is_fivefold(&history));
}

#[test]
fn fivefold_requires_five_occurrences() {
    let mut pos = Position::starting();
    let mut history: Vec<RepetitionKey> = Vec::new();
    let cycle = [
        quiet(Square::G1, Square::F3),
        quiet(Square::G8, Square::F6),
        quiet(Square::F3, Square::G1),
        quiet(Square::F6, Square::G8),
    ];
    for _ in 0..4 {
        for m in cycle {
            history.push(pos.repetition_key());
            pos = play(&pos, m);
        }
    }
    assert_eq!(pos.repetition_count(&history), 5);
    assert!(pos.is_fivefold(&history));
}

#[test]
fn castling_rights_change_breaks_repetition_identity() {
    let mut a = Position::starting();
    let mut b = Position::starting();
    b.set_castling(corvin_chess_core::CastlingRights::none());
    assert_ne!(a.repetition_key(), b.repetition_key());
    a.set_castling(corvin_chess_core::CastlingRights::none());
    assert_eq!(a.repetition_key(), b.repetition_key());
}

#[test]
fn halfmove_and_fullmove_do_not_affect_repetition_key() {
    let mut a = Position::starting();
    let mut b = Position::starting();
    a.set_halfmove_clock(0);
    b.set_halfmove_clock(40);
    a.set_fullmove_number(1);
    b.set_fullmove_number(20);
    assert_eq!(a.repetition_key(), b.repetition_key());
}

#[test]
fn en_passant_only_in_key_when_capture_is_legal() {
    // After 1.e4 c5 2.e5 d5 — White can capture EP on d6.
    let mut pos = Position::starting();
    pos = play(&pos, quiet(Square::E2, Square::E4));
    pos = play(&pos, quiet(Square::C7, Square::C5));
    pos = play(&pos, quiet(Square::E4, Square::E5));
    pos = play(&pos, quiet(Square::D7, Square::D5));
    assert_eq!(pos.ep_square(), Some(Square::D6));
    let with_ep = pos.repetition_key();

    // Same placement and side but EP rights cleared: different key while EP was legal.
    let mut cleared = pos.clone();
    cleared.set_ep_square(None);
    assert_ne!(with_ep, cleared.repetition_key());

    // EP square set but no capturer: effective EP must be ignored in the key.
    let mut no_capturer = kings_only();
    no_capturer.set_piece(Square::A2, piece(Color::White, PieceKind::Pawn));
    no_capturer.set_piece(Square::D5, piece(Color::Black, PieceKind::Pawn));
    no_capturer.set_side_to_move(Color::White);
    no_capturer.set_ep_square(Some(Square::D6));
    let mut no_ep_field = no_capturer.clone();
    no_ep_field.set_ep_square(None);
    assert_eq!(
        no_capturer.repetition_key(),
        no_ep_field.repetition_key(),
        "illegal EP target must not enter the repetition key"
    );
}

#[test]
fn pinned_en_passant_does_not_enter_repetition_key() {
    // White king e1, white pawn e5 pinned by black rook e8; black pawn d5 just pushed.
    // EP to d6 would expose the king, so effective EP must be None.
    let mut pos = kings_only();
    pos.set_piece(Square::E5, piece(Color::White, PieceKind::Pawn));
    pos.set_piece(Square::D5, piece(Color::Black, PieceKind::Pawn));
    pos.set_piece(Square::E8, piece(Color::Black, PieceKind::Rook));
    pos.set_side_to_move(Color::White);
    pos.set_ep_square(Some(Square::D6));
    let mut cleared = pos.clone();
    cleared.set_ep_square(None);
    assert_eq!(pos.repetition_key(), cleared.repetition_key());
}

/// Property: any position with a queen, rook, or pawn for either side is not insufficient.
#[test]
fn property_major_or_pawn_never_insufficient() {
    for kind in [PieceKind::Queen, PieceKind::Rook, PieceKind::Pawn] {
        for color in [Color::White, Color::Black] {
            let mut pos = kings_only();
            let sq = if color == Color::White {
                Square::A2
            } else {
                Square::A7
            };
            pos.set_piece(sq, piece(color, kind));
            assert!(
                !pos.is_insufficient_material(),
                "{color:?} {kind:?} must keep mating potential somewhere"
            );
            assert!(pos.has_mating_potential(color));
        }
    }
}

/// Property: repetition_count is 1 + number of matching prior keys.
#[test]
fn property_repetition_count_matches_history_filter() {
    let pos = Position::starting();
    let key = pos.repetition_key();
    let prior = vec![key, key, Position::empty().repetition_key(), key];
    assert_eq!(pos.repetition_count(&prior), 1 + 3);
    assert!(pos.is_threefold(&prior));
    assert!(!pos.is_fivefold(&prior));
    let prior5 = vec![key; 4];
    assert!(pos.is_fivefold(&prior5));
}

/// Property: can_claim_fifty_move iff halfmove_clock >= 100; seventy-five iff >= 150.
#[test]
fn property_fifty_and_seventy_five_thresholds() {
    let mut pos = kings_only();
    for clock in [0u16, 1, 50, 99, 100, 101, 149, 150, 200] {
        pos.set_halfmove_clock(clock);
        assert_eq!(pos.can_claim_fifty_move(), clock >= 100);
        assert_eq!(pos.is_seventy_five_move(), clock >= 150);
    }
}

/// Property: insufficient material is symmetric in side-to-move.
#[test]
fn property_insufficient_material_ignores_side_to_move() {
    let configs: [fn() -> Position; 4] = [
        kings_only,
        || {
            let mut p = kings_only();
            p.set_piece(Square::C1, piece(Color::White, PieceKind::Bishop));
            p
        },
        || {
            let mut p = kings_only();
            p.set_piece(Square::B1, piece(Color::White, PieceKind::Knight));
            p
        },
        || {
            let mut p = kings_only();
            p.set_piece(Square::C1, piece(Color::White, PieceKind::Bishop));
            p.set_piece(Square::F8, piece(Color::Black, PieceKind::Bishop));
            p
        },
    ];
    for build in configs {
        let mut white = build();
        white.set_side_to_move(Color::White);
        let mut black = white.clone();
        black.set_side_to_move(Color::Black);
        assert_eq!(
            white.is_insufficient_material(),
            black.is_insufficient_material()
        );
    }
}
