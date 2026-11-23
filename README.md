<div align="center">

<a href="https://github.com/corvin-chess"><img src="https://capsule-render.vercel.app/api?type=waving&height=200&color=0:0b0d12,55:161b2b,100:2f3d63&text=Corvin&fontColor=e6e9f2&fontSize=64&fontAlignY=45&animation=twinkling" alt="Corvin" width="100%" /></a>

<img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=500&size=16&color=8FA3D9&center=true&vCenter=true&width=560&height=34&duration=2800&pause=1200&lines=chess-core;Rust+rules+library+with+FEN+and+legal+moves." alt="chess-core" />

<p>
  <a href="https://github.com/corvin-chess/chess-core"><img src="https://skillicons.dev/icons?i=rust&theme=dark" width="44" height="44" alt="chess-core" /></a>&nbsp;
  <a href="https://github.com/corvin-chess"><img src="https://skillicons.dev/icons?i=github&theme=dark" width="44" height="44" alt="GitHub" /></a>
</p>

</div>

<br/>

Rust crate `corvin-chess-core` for Corvin rules. Board representation, legal move generation for standard chess (pins, checks, castling, en passant, promotions), and FEN load/store are built. SAN/UCI/PGN helpers, repetition and fifty-move helpers, a perft suite, and WASM/TypeScript bindings remain Planned for [web](https://github.com/corvin-chess/web), [server](https://github.com/corvin-chess/server), and [engine](https://github.com/corvin-chess/engine). This library is a leaf dependency with no chess-engine search.

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=OVERVIEW" alt="Overview" /></p>

Rust crate `corvin-chess-core`. Built: bitboard board representation, starting position, legal move generation, and FEN load/store. Planned: SAN/UCI/PGN helpers, repetition and fifty-move helpers, perft suite, WASM and TypeScript bindings. Leaf dependency; no search. Consumers include [web](https://github.com/corvin-chess/web), [server](https://github.com/corvin-chess/server), and [engine](https://github.com/corvin-chess/engine).

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=GETTING+STARTED" alt="Getting Started" /></p>

```bash
git clone https://github.com/corvin-chess/chess-core.git
cd chess-core
cargo test
```

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=DEVELOPMENT" alt="Development" /></p>

Requires a stable Rust toolchain (1.75 or newer). Format and lint with:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
```

WASM build steps are Planned.

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=TESTING" alt="Testing" /></p>

```bash
cargo test
node scripts/verify-readme.mjs
```

Board representation, legal move, and FEN tests cover square indexing, bitboard ops, startpos move count, pins, checks, castling, en passant, promotions, and FEN round-trips. Property, fuzz, and perft checks are Planned.

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><img src="https://readme-typing-svg.demolab.com?font=IBM+Plex+Mono&weight=700&size=18&color=8FA3D9&center=true&vCenter=true&width=420&height=40&duration=1&pause=1000000&letterSpacing=6px&lines=ARCHITECTURE" alt="Architecture" /></p>

Built layout: `src/board` (`square`, `color`, `piece`, `bitboard`, `position`), `src/fen` (six-field FEN load/store), and `src/movegen` (`Move`, attacks, legal filter via king-safety). Planned: `src/pgn`, `tests/perft`. Rules stay in this crate; search and evaluation belong in [engine](https://github.com/corvin-chess/engine). See organization ADRs and [website](https://github.com/corvin-chess/website) once docs are published.

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
| PGN/SAN/UCI | Planned |
| Perft suite | Planned |
| WASM + TS bindings | Planned |
| Implementation | Planned |

<img src="https://capsule-render.vercel.app/api?type=rect&height=3&color=0:0b0d12,35:8fa3d9,70:c9a227,100:0b0d12" width="100%" alt="" />

<br/>

<p align="center"><sub><i>Rules without shortcuts: every legal move, every draw claim, every check.</i></sub></p>

## License

This repository is licensed under AGPL-3.0-or-later.

<img src="https://capsule-render.vercel.app/api?type=waving&section=footer&height=110&color=0:0b0d12,55:161b2b,100:2f3d63" alt="" width="100%" />
