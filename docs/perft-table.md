# Perft reference table (CV-105)

Canonical leaf-node counts for Corvin `chess-core` correctness tests.
Counts match the widely published Chess Programming Wiki perft suite
(https://chessprogramming.org/Perft_Results): leaf nodes only; draw-by
repetition, fifty-move, and insufficient material are ignored.

CI runs every row marked **ci**. Deeper rows are documented for local
`--release` checks and are not required in default CI.

## Position 1 — startpos

FEN: `rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1`

| Depth | Nodes | Suite |
|---|---|---|
| 1 | 20 | ci |
| 2 | 400 | ci |
| 3 | 8902 | ci |
| 4 | 197281 | ci |
| 5 | 4865609 | ci |
| 6 | 119060324 | local |

## Position 2 — Kiwipete

FEN: `r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1`

| Depth | Nodes | Suite |
|---|---|---|
| 1 | 48 | ci |
| 2 | 2039 | ci |
| 3 | 97862 | ci |
| 4 | 4085603 | ci |
| 5 | 193690690 | local |

## Position 3

FEN: `8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1`

| Depth | Nodes | Suite |
|---|---|---|
| 1 | 14 | ci |
| 2 | 191 | ci |
| 3 | 2812 | ci |
| 4 | 43238 | ci |
| 5 | 674624 | ci |
| 6 | 11030083 | local |

## Position 4

FEN: `r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1`

| Depth | Nodes | Suite |
|---|---|---|
| 1 | 6 | ci |
| 2 | 264 | ci |
| 3 | 9467 | ci |
| 4 | 422333 | ci |
| 5 | 15833292 | local |

## Position 5

FEN: `rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8`

| Depth | Nodes | Suite |
|---|---|---|
| 1 | 44 | ci |
| 2 | 1486 | ci |
| 3 | 62379 | ci |
| 4 | 2103487 | ci |
| 5 | 89941194 | local |

## Position 6

FEN: `r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10`

| Depth | Nodes | Suite |
|---|---|---|
| 1 | 46 | ci |
| 2 | 2079 | ci |
| 3 | 89890 | ci |
| 4 | 3894594 | ci |
| 5 | 164075551 | local |
