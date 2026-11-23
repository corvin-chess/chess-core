/**
 * CV-106: Node loads the WASM package and calls legalMoves.
 * Run after: wasm-pack build --target nodejs --out-dir pkg -- --features wasm
 */
import assert from "node:assert/strict"
import { createRequire } from "node:module"
import { dirname, join } from "node:path"
import { fileURLToPath } from "node:url"
import { existsSync, readFileSync } from "node:fs"
import test from "node:test"

const __dirname = dirname(fileURLToPath(import.meta.url))
const pkgDir = join(__dirname, "..", "..", "pkg")
const pkgEntry = join(pkgDir, "corvin_chess_core.js")
const pkgDts = join(pkgDir, "corvin_chess_core.d.ts")

const START_FEN =
  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

test("pkg exists (build wasm first)", () => {
  assert.ok(
    existsSync(pkgEntry),
    `missing ${pkgEntry}; run wasm-pack build --target nodejs --out-dir pkg -- --features wasm`
  )
})

test("TypeScript typings export legalMoves", () => {
  assert.ok(existsSync(pkgDts), `missing ${pkgDts}`)
  const dts = readFileSync(pkgDts, "utf8")
  assert.match(dts, /export function legalMoves\(fen: string\): string\[\];/)
})

test("legalMoves returns 20 UCI moves from startpos", async () => {
  // wasm-pack nodejs target is CommonJS
  const require = createRequire(import.meta.url)
  const { legalMoves } = require(pkgEntry)
  const moves = legalMoves(START_FEN)
  assert.ok(Array.isArray(moves), "legalMoves must return an array")
  assert.equal(moves.length, 20)
  for (const m of moves) {
    assert.equal(typeof m, "string")
    assert.match(m, /^[a-h][1-8][a-h][1-8][qrbn]?$/)
  }
  assert.ok(moves.includes("e2e4"))
  assert.ok(moves.includes("g1f3"))
})

test("legalMoves rejects invalid FEN", () => {
  const require = createRequire(import.meta.url)
  const { legalMoves } = require(pkgEntry)
  assert.throws(() => legalMoves("not-a-fen"), /invalid|fen/i)
})

test("legalMoves rejects oversized FEN", () => {
  const require = createRequire(import.meta.url)
  const { legalMoves } = require(pkgEntry)
  assert.throws(() => legalMoves("x".repeat(257)), /invalid|fen|long/i)
})
