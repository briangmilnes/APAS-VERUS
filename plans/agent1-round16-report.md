# Agent 1 — Round 16 Report

## Summary

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 42 | TableMtEph.rs | 6 | 4 | -2 |
| 2 | 41 | AVLTreeSetMtPer.rs | 10 | 9 | -1 |
| | | **Total** | **136** | **133** | **-3** |

Verification: 4108 verified, 0 errors.
RTT: 2600 passed. PTT: 147 passed.

## Holes Proved

### TableMtEph::intersection (Chap42) — external_body removed
Ported nested-scan proof from verified TableStEph. Ghost `self_srcs` and `other_srcs` sequences track which indices contribute to the output. Post-loop proof uses `lemma_entries_to_map_subseq_value` for domain equality.

Key adaptation from StEph: Used broadcast triggers (`obeys_feq_full_trigger`, `obeys_view_eq_trigger`) instead of explicit feq/view_eq in requires, matching the pattern from the verified `difference` function.

### TableMtEph::union (Chap42) — external_body removed
Two-phase scan approach:
- Phase 1: iterate self entries, scan other for match. Combine values if found, clone if not.
- Phase 2: iterate other entries, scan self for match. Add other-only entries.

Required two new helper lemmas:
- `lemma_entries_to_map_ignore_suffix`: if suffix entries don't have key k, map value comes from prefix
- `lemma_entries_to_map_agree_on_key`: if two sequences have same keys and same values at key-k positions, their maps agree at k

Also fixed `lemma_entries_to_map_skip_prefix` — original had a bug when `n == entries.len()` (missing case for vacuous contradiction).

### AVLTreeSetMtPer::size() (Chap41) — assume removed
Strengthened `spec_avltreesetmtper_wf` to include `self.elements@.no_duplicates()`. Used vstd's `unique_seq_to_set()` lemma to prove `seq.len() == seq.to_set().len()`. All constructors (empty, singleton, external_body functions) maintain the invariant.

## Cascading Changes

Added `requires forall|v1: &V, v2: &V| combine.requires((v1, v2))` to:
- `TableMtEphTrait::intersection`, `union`, `insert` (Chap42)
- `OrderedTableMtEphTrait::intersection`, `union`, `insert` (Chap43)
- `AugOrderedTableMtEphTrait::intersection`, `union`, `insert` (Chap43)

These propagate the closure-requires obligation to callers per `using_closures_standard.rs`.

## Remaining Holes

### TableMtEph (4 remaining — all structural)
All use `join()` for fork-join parallelism (thread boundaries):
- `tabulate` — parallel key mapping
- `map` — parallel value transformation
- `filter` — parallel filtering
- `insert` — parallel insertion

### AVLTreeSetMtPer (9 remaining)
- 1 assume: `find()` — binary search needs spec-level sorted invariant (requires spec ordering for generic T::V, deep infrastructure)
- 8 external_body: all use ParaPair! (thread boundaries) or depend on external_body functions (delete→filter, difference→filter, insert→from_seq, cmp→Ord trait)

## Techniques Used
1. **Ghost source tracking**: `Seq<int>` sequences mapping output indices to input indices
2. **Subsequence value lemma**: proves map values agree between a subsequence and the original
3. **Broadcast triggers**: `obeys_feq_full_trigger`/`obeys_view_eq_trigger` for feq/view_eq without requires
4. **wf strengthening**: adding `no_duplicates()` to enable seq-to-set length proof
5. **Helper lemma pair**: ignore_suffix + agree_on_key for compositional map value proofs

## What Blocks Further Progress
- **TableMtEph**: Thread-boundary external_body is irreducible (join() spawns threads)
- **AVLTreeSetMtPer find()**: Needs spec-level ordering for generic T (TotalOrder trait or equivalent). Binary search correctness requires sorted invariant.
- **AVLTreeSetMtPer external_body**: All structural (ParaPair! thread spawning) or dependent on structural holes

## Commit
Pending — will commit and push to agent1/ready.
