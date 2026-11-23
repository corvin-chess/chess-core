//! Perft suite: exact leaf counts vs docs/perft-table.md (CV-105).

use corvin_chess_core::Position;
use std::fs;
use std::path::PathBuf;

struct Case {
    name: &'static str,
    fen: &'static str,
    /// (depth, expected nodes) for CI.
    expected: &'static [(u32, u64)],
}

/// CI depths from docs/perft-table.md (rows marked ci).
const CASES: &[Case] = &[
    Case {
        name: "startpos",
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        expected: &[(1, 20), (2, 400), (3, 8902), (4, 197_281), (5, 4_865_609)],
    },
    Case {
        name: "kiwipete",
        fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        expected: &[(1, 48), (2, 2039), (3, 97_862), (4, 4_085_603)],
    },
    Case {
        name: "position3",
        fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        expected: &[(1, 14), (2, 191), (3, 2812), (4, 43_238), (5, 674_624)],
    },
    Case {
        name: "position4",
        fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        expected: &[(1, 6), (2, 264), (3, 9467), (4, 422_333)],
    },
    Case {
        name: "position5",
        fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        expected: &[(1, 44), (2, 1486), (3, 62_379), (4, 2_103_487)],
    },
    Case {
        name: "position6",
        fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        expected: &[(1, 46), (2, 2079), (3, 89_890), (4, 3_894_594)],
    },
];

#[test]
fn perft_depth_zero_is_one() {
    let pos = Position::starting();
    assert_eq!(pos.perft(0), 1);
}

#[test]
fn published_table_lists_ci_counts() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/perft-table.md");
    let table = fs::read_to_string(&path).expect("docs/perft-table.md must exist");
    for case in CASES {
        assert!(
            table.contains(case.fen),
            "table missing FEN for {}",
            case.name
        );
        for &(depth, nodes) in case.expected {
            let row = format!("| {depth} | {nodes} | ci |");
            assert!(
                table.contains(&row),
                "table missing CI row for {} depth {depth}: expected `{row}`",
                case.name
            );
        }
    }
}

#[test]
fn perft_matches_published_ci_suite() {
    for case in CASES {
        let pos = Position::from_fen(case.fen).unwrap_or_else(|e| {
            panic!("{}: failed to parse FEN: {e}", case.name);
        });
        for &(depth, expected) in case.expected {
            let got = pos.perft(depth);
            assert_eq!(
                got, expected,
                "{} perft({depth}): got {got}, expected {expected}",
                case.name
            );
        }
    }
}
