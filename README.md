<div align="center">

<a href="https://github.com/corvin-chess"><img src="https://capsule-render.vercel.app/api?type=waving&height=200&color=0:0b0d12,55:161b2b,100:2f3d63&text=Corvin&fontColor=e6e9f2&fontSize=64&fontAlignY=45&animation=twinkling" alt="Corvin" width="100%" /></a>

<img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=500&size=16&color=8FA3D9&center=true&vCenter=true&width=560&height=34&duration=2800&pause=1200&lines=chess-core;Rust+rules+library+with+FEN,+SAN,+UCI,+and+legal+moves." alt="chess-core" />

<p>
  <a href="https://github.com/corvin-chess/chess-core"><img src="https://skillicons.dev/icons?i=rust&theme=dark" width="44" height="44" alt="chess-core" /></a>&nbsp;
  <a href="https://github.com/corvin-chess"><img src="https://skillicons.dev/icons?i=github&theme=dark" width="44" height="44" alt="GitHub" /></a>
</p>

</div>

<br/>

Rust crate `corvin-chess-core` for Corvin rules. Board representation, legal move generation for standard chess (pins, checks, castling, en passant, promotions), FEN load/store, UCI/SAN/PGN movetext helpers, draw-rule helpers (threefold/fivefold repetition, fifty/seventy-five move, insufficient material), a perft suite against `docs/perft-table.md`, and WASM/TypeScript bindings exposing `legalMoves` for [web](https://github.com/corvin-chess/web) (and other consumers such as [server](https://github.com/corvin-chess/server) and [engine](https://github.com/corvin-chess/engine) via the Rust crate) are built. This library is a leaf dependency with no chess-engine search.

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=OVERVIEW" alt="Overview" /></p>

Rust crate `corvin-chess-core`. Built: bitboard board representation, starting position, legal move generation, FEN load/store, UCI/SAN/PGN movetext helpers (including ambiguity SAN), draw-rule helpers (repetition keys, threefold/fivefold, fifty/seventy-five move, insufficient material), perft against the published table in `docs/perft-table.md`, and WASM/TypeScript bindings (`legalMoves(fen)` returns UCI strings). Leaf dependency; no search. Consumers include [web](https://github.com/corvin-chess/web), [server](https://github.com/corvin-chess/server), and [engine](https://github.com/corvin-chess/engine).

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=GETTING+STARTED" alt="Getting Started" /></p>

```bash
git clone https://github.com/corvin-chess/chess-core.git
cd chess-core
cargo test --release
```

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=DEVELOPMENT" alt="Development" /></p>

Requires a stable Rust toolchain (1.75 or newer) and, for WASM, the `wasm32-unknown-unknown` target plus [wasm-pack](https://rustwasm.github.io/wasm-pack/). Format and lint with:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
```

Build the Node.js package used by CI (generates `pkg/` including TypeScript typings). For a bundler such as Vite in [web](https://github.com/corvin-chess/web), use `--target bundler` the same way:

```bash
wasm-pack build --target nodejs --out-dir pkg -- --features wasm
```

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=TESTING" alt="Testing" /></p>

```bash
cargo test --release
wasm-pack build --target nodejs --out-dir pkg -- --features wasm
node --test tests/wasm/legal_moves.test.mjs
node scripts/verify-readme.mjs
```

Board representation, legal move, FEN, notation, draw-rule, and perft tests cover square indexing, bitboard ops, startpos move count, pins, checks, castling, en passant, promotions, FEN round-trips, UCI round-trips, ambiguity SAN, PGN movetext, repetition counts, fifty-move thresholds, insufficient-material cases, and exact leaf counts from `docs/perft-table.md`. The Node WASM test loads `pkg` and calls `legalMoves` on the start position (expects 20 UCI moves). Use `--release` for the CI perft depths (debug is much slower). Fuzz checks are Planned.

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=ARCHITECTURE" alt="Architecture" /></p>

Built layout: `src/board` (`square`, `color`, `piece`, `bitboard`, `position`), `src/fen` (six-field FEN load/store), `src/movegen` (`Move`, attacks, legal filter via king-safety), `src/uci`, `src/san`, `src/pgn` (movetext encode/decode), `src/draw` (repetition key, fifty/seventy-five, insufficient material), `src/perft`, `src/wasm` (`legalMoves` via wasm-bindgen, feature `wasm`), and `tests/perft` / `tests/wasm` against `docs/perft-table.md` and the Node package. Rules stay in this crate; search and evaluation belong in [engine](https://github.com/corvin-chess/engine). See organization ADRs and [website](https://github.com/corvin-chess/website) once docs are published.

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=STATUS" alt="Status" /></p>

| Area | State |
|---|---|
| README | Built |
| Crate scaffold | Built |
| Board representation | Built |
| Legal move generation | Built |
| FEN load/store | Built |
| PGN/SAN/UCI | Built |
| Draw helpers | Built |
| Perft suite | Built |
| WASM + TS bindings | Built |
| Implementation | Planned |

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><sub><i>Rules without shortcuts: every legal move, every draw claim, every check.</i></sub></p>

## License

This repository is licensed under AGPL-3.0-or-later.

<img src="https://capsule-render.vercel.app/api?type=waving&section=footer&height=110&color=0:0b0d12,55:161b2b,100:2f3d63" alt="" width="100%" />
