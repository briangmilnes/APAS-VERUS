# Agent 4 — Round 42 Report

## Summary

- **Baseline**: main at `e83db19f`, 4320 verified, 153 holes, 30 clean chapters
- **Final**: 4326 verified, 0 errors, 2613 RTT pass
- **Holes**: 152 comparable (165 raw includes 13 new `cfg_hidden_exec` detections)
- **Net**: -1 actionable hole from baseline
- **Commits**: R42a (JohnsonStEphI64 fix), R42b (StructChainedHashTable resize proof)

## Holes Before/After by File

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 47 | StructChainedHashTable.rs | 1 | 0 | -1 | resize external_body proved |
| 2 | 59 | JohnsonStEphI64.rs | 1 | 1 | 0 | assume moved to reweight_graph |
| 3 | 06 | WeightedDirGraphStEphI128.rs | 0 | 0 | 0 | strengthened from_weighed_edges ensures |

## Part A: StructChainedHashTable resize — PROVED

Removed `#[verifier::external_body]` from `resize` in `src/Chap47/StructChainedHashTable.rs`.

**Key insight**: Chain key uniqueness. `spec_chain_to_map` uses head-wins ordering;
forward reinsertion uses last-wins. These agree only when chains have unique keys.

**New infrastructure**:
- `spec_chain_keys_unique` predicate: recursive no-duplicate-keys check
- `spec_impl_wf` override: adds chain uniqueness to base `spec_hashtable_wf`
- Uniqueness preservation ensures on `chain_insert` and `chain_delete`
- Updated `insert` and `delete` proof blocks to maintain chain uniqueness

**Resize proof (3 phases, ~200 lines)**:
1. **Collect pairs**: Outer loop over buckets, inner loop traverses each chain.
   Ghost `pairs_map` tracks `spec_seq_pairs_to_map(pairs@)`. Inner loop uses
   chain partition invariant: `inner.upr(chain_to_map(current)) =~= chain_to_map(original)`.
   After Phase 1 proves `pairs_map =~= table@` via extensional equality.
2. **Empty table**: Proves `spec_impl_wf` (chain_keys_unique for empty chains).
3. **Reinsert**: `new_table@ =~= spec_seq_pairs_to_map(pairs@.subrange(0, m))`,
   maintained via `subrange(0, m+1).drop_last() =~= subrange(0, m)`.

**Algebraic properties used**:
- `M.upr(N).insert(k,v) =~= M.upr(N.insert(k,v))` (always true)
- `lemma_table_to_map_unique_entry_value` + `lemma_table_to_map_not_contains`

## Part B: JohnsonMtEphI64 — Assessment

All 5 `external_body` functions in `src/Chap59/JohnsonMtEphI64.rs` are standalone parallel
implementations with `#[cfg(not(verus_keep_ghost))]`. They are NOT delegation wrappers around
StEph counterparts — they contain full algorithmic logic including `ParaPair!` fork-join,
Bellman-Ford calls, Dijkstra calls, and graph construction.

**Functions**: `johnson_apsp`, `parallel_dijkstra_all`, `add_dummy_source`, `reweight_graph`,
`create_negative_cycle_result`.

**Assessment**: These require full reimplementation inside `verus!` with verified specs.
The `parallel_dijkstra_all` recursive divide-and-conquer with `ParaPair!` is particularly
complex. This is major work — not a quick proof fix. The StEph counterpart
(`JohnsonStEphI64.rs`) already has the sequential verified implementation; the Mt version
needs closure specs and ghost tracking for the parallel decomposition.

## Part C: JohnsonStEphI64 — FIXED (R42a)

- Moved `assume(result@.A.len() <= graph@.A.len())` from `johnson_apsp` to `reweight_graph`
  where the edge bound is actually needed
- Added `graph@.A.len() * 2 + 2 <= usize::MAX as int` precondition to both trait and impl
- Strengthened `from_weighed_edges` in `WeightedDirGraphStEphI128` with `g@.A =~= edges@`
- 2 `fn_missing_requires` warnings on `adjust_distance` and `reweight_edge`: these need
  real preconditions (arithmetic overflow bounds), not `requires true`

## Part D: OrderedSet from_sorted_elements — Assessment

2 `fn_missing_requires` warnings in Chap43:
- `OrderedSetStEph::from_sorted_elements` (line 1385)
- `OrderedSetStPer::from_sorted_elements` (line 1157)

Both delegate to `AVLTreeSeqStPerS::from_vec` which has no requires, then to `from_seq`.
These genuinely have no precondition. Per CLAUDE.md rules, agents must NOT add
`requires true` or `// veracity: no_requires`. Left for user annotation.

## Techniques Used

- **Closed spec fn / uninterpreted function** pattern (from R41 DoubleHash)
- **Ghost map tracking** through loop iterations
- **Extensional equality** (`=~=`) for map equivalence proofs
- **Chain partition invariant** for compositional chain traversal
- **reveal_with_fuel** for recursive spec fn unfolding
- **clone_elem** for ghost-tracked generic cloning

## Remaining Holes

Chap47 has 5 remaining actionable holes across 4 files (DoubleHash, LinProb, QuadProb,
LinkedListChained). StructChainedHashTable is now clean.
