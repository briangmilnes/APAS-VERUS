# Agent 3 — R128b Report: Parallelize Chap62 StarPartitionMtEph

## Summary

Parallelized loops 2 (coin flips) and 3 (edge classification) in
`src/Chap62/StarPartitionMtEph.rs` using D&C with `ParaPair!`. Loops 1, 4, 5, 6
remain sequential.

## Loop Parallelization Table

| # | Loop | Parallelized? | Old Span | New Span | Reason if not |
|---|------|---------------|----------|----------|---------------|
| 1 | Vertex-to-index map | No | O(n) | O(n) | HashMap build is inherently sequential (each insert depends on prior state) |
| 2 | Coin flips | Yes | O(n) | O(lg n) | hash_coin_flips_mt: D&C with ParaPair!, deterministic hash-based coins replace sequential RNG |
| 3 | Edge classification | Yes | O(m) | O(lg m) | build_th_edges_mt: D&C with ParaPair!, each edge independently checks coin_flips |
| 4 | Initialize p_vec | No | O(n) | O(n) | Simple sequential copy; parallelizing adds thread overhead for marginal gain |
| 5 | Apply th_edges | No | O(|th_edges|) | O(|th_edges|) | Priority-based writes with potential conflicts (multiple tails may share a head) |
| 6 | Build centers/partition | No | O(n) | O(n) | Complex proof interdependencies with Loop 5's invariants; span still dominated by loops 1, 4, 5 |

## New Functions Added

| # | Chap | File | Function | Lines | Purpose |
|---|------|------|----------|-------|---------|
| 1 | 62 | StarPartitionMtEph.rs | `hash_coin` | ~10 | Deterministic hash-based coin flip (external_body) |
| 2 | 62 | StarPartitionMtEph.rs | `hash_coin_flips_mt` | ~100 | D&C parallel coin flip generation via ParaPair! |
| 3 | 62 | StarPartitionMtEph.rs | `build_th_edges_mt` | ~110 | D&C parallel edge classification via ParaPair! |
| 4 | 62 | StarPartitionMtEph.rs | `spec_valid_th_entry` | ~15 | Spec helper for th_edge invariant |

## Overall Function Span

| Metric | Before | After |
|--------|--------|-------|
| parallel_star_partition span | O(n + m) | O(n + lg m) |
| Bottleneck | All 6 loops sequential | Loops 1, 4, 5 sequential at O(n) |
| APAS target | O(lg n) | O(n + lg m) — gap remains from loops 1, 4, 5, 6 |

## Verification

- 5509 verified, 0 errors (full validation)
- 3534 RTTs pass
- 221 PTTs pass

## Key Technical Decisions

1. **Hash-based coin flips** replace sequential RNG (`seeded_rng`/`random_bool_seeded`).
   Modeled after Boruvka's `hash_coin` in `src/Chap66/BoruvkaMtEph.rs`. Deterministic
   from (seed, index), enabling parallel computation.

2. **Arc wrapping** for shared read-only data: `vertices_vec`, `vertex_to_index`,
   `coin_flips`, and `edge_vec` are wrapped in Arc for sharing across ParaPair join arms.
   Shadowed with `arc_deref` references for subsequent sequential loops.

3. **spec_valid_th_entry** spec function encapsulates the 7-conjunct th_edge invariant,
   reducing verbosity in D&C closure ensures clauses.

4. **Merge strategy**: coin flip merge uses HashMap iterator with while-loop tracking
   (`merge_done` flag for loop-exit fact). Edge classification merge uses Vec concatenation
   (per-entry invariant preserved trivially).

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 62 | StarPartitionMtEph.rs | Added parallel helpers, replaced loops 2 and 3 |
| 2 | 62 | StarContractionMtEph.rs | Updated span analysis annotation |
