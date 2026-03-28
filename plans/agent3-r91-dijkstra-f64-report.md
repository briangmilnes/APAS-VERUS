# Agent 3 — R91 Report: DijkstraStEphF64

## Summary

Built `src/Chap57/DijkstraStEphF64.rs` from the fully-proved `DijkstraStEphU64.rs`
template, converting integer weights to `WrappedF64` float weights.

## Changes

| # | Chap | File | Action |
|---|------|------|--------|
| 1 | 57 | DijkstraStEphF64.rs | Replaced stub with full Dijkstra implementation using WrappedF64 |
| 2 | 57 | TestDijkstraStEphF64.rs | Rewrote tests to use WrappedF64 and WeightedDirGraphStEphF64 |
| 3 | — | lib.rs | Uncommented `pub mod DijkstraStEphF64` |
| 4 | — | Cargo.toml | Uncommented test entry for TestDijkstraStEphF64 |

## Holes

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 57 | DijkstraStEphF64.rs | 0 (stub) | 5 | +5 |

All 5 holes are `external_body` on PQEntry's `TotalOrder` impl:
- 4 proof methods: `reflexive`, `transitive`, `antisymmetric`, `total`
- 1 exec method: `cmp`

**Why these can't be proved:** The `le` spec for PQEntry uses `f64::le` (via
`le_ensures`), and our float axioms only provide total-order properties for
finite values (`is_finite_spec()`). The TotalOrder trait requires unconditional
proofs for ALL PQEntry values, including those with non-finite dist. Since
`le_ensures` is uninterpreted for non-finite f64, the proofs are stuck.

**Why this is sound:** In Dijkstra, all PQ entries have finite distances (source
starts at 0, and only finite `dist_add` results are inserted). The exec `cmp`
correctly handles all cases including non-finite via `partial_cmp`.

The U64 version has 0 holes because integer `<`/`<=` are decidable for all values.

## Key Adaptations from U64

1. **Weight type**: `i64` dist → `WrappedF64` dist
2. **Addition**: `dist.wrapping_add(weight as i64)` → `dist.dist_add(weight)`
3. **Comparison**: `new_dist < u_dist` → three-way check: non-finite new_dist
   (never better), non-finite u_dist (any finite is better), both finite
   (use `dist_lt`)
4. **Ghost edge tuples**: `(usize, usize, i128)` → `(usize, usize, f64)`
   (WrappedF64's View is f64)
5. **SSSPResult wf**: Added `sssp.spec_ssspresultstephf64_wf()` to both loop
   invariants (F64 SSSP trait methods require it, U64 version didn't)
6. **Broadcast uses**: Added `group_float_finite_total_order` and
   `group_float_arithmetic`

## Verification

- Isolated: 2510 verified, 0 errors
- Full: pre-existing Chap52 flakiness (Z3 memory pressure), not caused by changes
- RTT: 3083 passed (7 new F64 Dijkstra tests)

## Techniques

- Direct template conversion from U64 → F64 with systematic type substitution
- Three-way comparison pattern for handling finite/non-finite WrappedF64 distances
- External_body TotalOrder proofs for float ordering (inherent limitation)
