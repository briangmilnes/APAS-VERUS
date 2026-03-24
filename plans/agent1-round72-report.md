# R72 Agent 1 Report — PTT Iterator Coverage (Chap37 Seq modules)

## Objective

Create iterator PTTs for Chap37 sequence modules to prove the iterator
requires chain end-to-end: constructor ensures wf, wf satisfies iter()
requires, iter ensures feed loop invariants.

## Assigned Modules

| # | Chap | File | Iterator Infra | PTT Created | Patterns |
|---|------|------|---------------|-------------|----------|
| 1 | 37 | AVLTreeSeq.rs | Full | Yes | loop-borrow-iter, for-borrow-iter |
| 2 | 37 | AVLTreeSeqStEph.rs | Partial | No | — |
| 3 | 37 | AVLTreeSeqStPer.rs | Partial | No | — |

## Created

- `rust_verify_test/tests/Chap37/ProveAVLTreeSeq.rs` — 2 test patterns
- Registered in `rust_verify_test/Cargo.toml`

## AVLTreeSeq.rs — Full Iterator Standard

AVLTreeSeq has complete iterator infrastructure:
- `AVLTreeSeqIter<'a, T>` with `View` type `(int, Seq<T>)`
- `AVLTreeSeqGhostIterator<'a, T>` with `View` type `Seq<T>`
- `iter_invariant` free spec fn
- `ForLoopGhostIteratorNew` and `ForLoopGhostIterator` impls
- `iter()` requires `self.spec_avltreeseq_wf()`, ensures position=0, elements match seq
- `next()` is `external_body` with full ensures (position tracking, element equality)

No `IntoIterator` impl exists, so borrow-into and consume patterns are not applicable.
Only 2 patterns: loop-borrow-iter and for-borrow-iter. Both prove clean.

## AVLTreeSeqStEph.rs / AVLTreeSeqStPer.rs — Incomplete Infrastructure

Both files have partial iterator infrastructure that prevents meaningful PTTs:

| Gap | Impact |
|-----|--------|
| No `View` on iterator struct | Cannot write `it@` in loop invariants or decreases |
| No `iter_invariant` free fn | No invariant to carry through loop |
| No `ForLoopGhostIteratorNew` / `ForLoopGhostIterator` | Cannot use `for x in iter:` syntax |
| All specs `ensures true` (iter, next, into_iter) | Nothing to prove about the iteration |

These modules need full iterator specs before PTTs can exercise any verification chain.

## Verification

- PTT: 147 tests, 147 passed, 0 skipped (was 145, +2 new)
- Verification: 4446 verified, 0 errors
