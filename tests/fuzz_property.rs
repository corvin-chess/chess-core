//! Fuzz and property tests for chess-core (CV-107).
//!
//! Deterministic PRNG only (no new crates). Fixed iteration budgets keep CI bounded.
//! Invariants cover legal-play consistency and parse paths that must never panic.

use corvin_chess_core::{Color, Move, PieceKind, Position, Square};

/// SplitMix64-style generator for reproducible sequences across platforms.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Rng {
        Rng(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn gen_range(&mut self, max: usize) -> usize {
        debug_assert!(max > 0);
        (self.next_u32() as usize) % max
    }

    fn fill_bytes(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let n = self.next_u64().to_le_bytes();
            for (dst, src) in chunk.iter_mut().zip(n.iter()) {
                *dst = *src;
            }
        }
    }
}

fn king_count(pos: &Position, color: Color) -> u32 {
    pos.pieces_of(color, PieceKind::King).0.count_ones()
}

fn assert_position_invariants(pos: &Position, context: &str) {
    assert_eq!(king_count(pos, Color::White), 1, "{context}: white kings");
    assert_eq!(king_count(pos, Color::Black), 1, "{context}: black kings");
    let fen = pos.to_fen();
    let round = Position::from_fen(&fen)
        .unwrap_or_else(|e| panic!("{context}: FEN parse after to_fen: {e}\n{fen}"));
    assert_eq!(round, *pos, "{context}: FEN round-trip\n{fen}");
    let legal = pos.legal_moves();
    assert_eq!(
        pos.perft(1),
        legal.len() as u64,
        "{context}: perft(1) vs legal_moves"
    );
    for m in &legal {
        let uci = m.to_uci();
        let parsed = Move::from_uci(&uci)
            .unwrap_or_else(|e| panic!("{context}: UCI round-trip parse: {e} ({uci})"));
        assert_eq!(parsed, *m, "{context}: UCI round-trip {uci}");
        let san = pos
            .to_san(*m)
            .unwrap_or_else(|e| panic!("{context}: to_san: {e}"));
        let from_san = pos
            .from_san(&san)
            .unwrap_or_else(|e| panic!("{context}: from_san {san}: {e}"));
        assert_eq!(from_san, *m, "{context}: SAN round-trip {san}");
        let after = pos
            .try_make_move(*m)
            .unwrap_or_else(|| panic!("{context}: legal move must apply: {uci}"));
        assert_eq!(king_count(&after, Color::White), 1);
        assert_eq!(king_count(&after, Color::Black), 1);
    }
}

/// Property: random legal play preserves board invariants and notation round-trips.
#[test]
fn property_random_legal_play_invariants() {
    const GAMES: usize = 64;
    const MAX_PLIES: usize = 80;
    let mut rng = Rng::new(0xc0_07_11_23);

    for game in 0..GAMES {
        let mut pos = Position::starting();
        assert_position_invariants(&pos, &format!("game {game} start"));

        for ply in 0..MAX_PLIES {
            let moves = pos.legal_moves();
            if moves.is_empty() {
                break;
            }
            let m = moves[rng.gen_range(moves.len())];
            let context = format!("game {game} ply {ply} {}", m.to_uci());
            pos = pos
                .try_make_move(m)
                .unwrap_or_else(|| panic!("{context}: apply failed"));
            assert_position_invariants(&pos, &context);
        }
    }
}

/// Property: a move not in `legal_moves` is rejected by `try_make_move`.
#[test]
fn property_illegal_random_moves_rejected() {
    let mut rng = Rng::new(0x0111_e6a1);
    let pos = Position::starting();
    let legal = pos.legal_moves();
    let promotions = [
        None,
        Some(PieceKind::Queen),
        Some(PieceKind::Rook),
        Some(PieceKind::Bishop),
        Some(PieceKind::Knight),
    ];

    for _ in 0..2_000 {
        let from = Square::from_index(rng.gen_range(64) as u8).unwrap();
        let to = Square::from_index(rng.gen_range(64) as u8).unwrap();
        let promotion = promotions[rng.gen_range(promotions.len())];
        let m = Move {
            from,
            to,
            promotion,
        };
        if legal.contains(&m) {
            assert!(pos.try_make_move(m).is_some());
        } else {
            assert!(
                pos.try_make_move(m).is_none(),
                "illegal move applied: {}",
                m.to_uci()
            );
        }
    }
}

/// Regression: SAN must reject mid-codepoint destination splits instead of panicking.
#[test]
fn san_rejects_multibyte_tail_without_panic() {
    let start = Position::starting();
    // After stripping 'N', rest is "éx" (3 bytes). len-2 == 1 falls inside 'é'.
    let input = "N\u{00e9}x";
    assert!(!input.is_char_boundary(input.len() - 2));
    assert_eq!(
        start.from_san(input),
        Err(corvin_chess_core::SanError::Invalid)
    );
}

/// Fuzz: arbitrary UTF-8-ish byte strings never panic FEN / UCI / SAN parsers.
#[test]
fn fuzz_parsers_no_panic() {
    let mut rng = Rng::new(0xf022_0a12);
    let start = Position::starting();
    let mut buf = [0u8; 96];

    let cap = buf.len();
    for _ in 0..4_000 {
        let len = rng.gen_range(cap + 1);
        let fill = len.max(1).min(cap);
        rng.fill_bytes(&mut buf[..fill]);
        // Mix printable and binary; lossy UTF-8 is fine for "must not panic".
        let s = String::from_utf8_lossy(&buf[..len.min(cap)]).into_owned();
        let _ = Position::from_fen(&s);
        let _ = Move::from_uci(&s);
        let _ = start.from_san(&s);
    }

    // Empty and oversized edge cases.
    let _ = Position::from_fen("");
    let _ = Move::from_uci("");
    let _ = start.from_san("");
    let huge = "a".repeat(10_000);
    let _ = Position::from_fen(&huge);
    let _ = Move::from_uci(&huge);
    let _ = start.from_san(&huge);
}

/// Property: after any legal move, the previous side cannot still be to move.
#[test]
fn property_side_to_move_flips_on_legal_move() {
    let mut rng = Rng::new(0x51de_f11b);
    let mut pos = Position::starting();
    for _ in 0..200 {
        let moves = pos.legal_moves();
        if moves.is_empty() {
            break;
        }
        let before = pos.side_to_move();
        let m = moves[rng.gen_range(moves.len())];
        pos = pos.try_make_move(m).expect("legal apply");
        assert_eq!(pos.side_to_move(), before.opposite());
    }
}
