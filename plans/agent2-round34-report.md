# Agent 2 Round 34 Report

## Goal
Prove external_body functions in Chap47 hash table implementations.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 47 | LinProbFlatHashTableStEph.rs | 3 | 2 | -1 |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | 3 | 2 | -1 |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | 3 | 2 | -1 |
| 4 | 47 | VecChainedHashTableStEph.rs | 1 | 0 | -1 |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 1 | 0 | -1 |
| 6 | 47 | StructChainedHashTable.rs | 1 | 1 | 0 |
| 7 | 47 | ParaHashTableStEph.rs | 2 | 2 | 0 |
| | | **Total** | **14** | **9** | **-5** |

## Verified Counts
- Before: 4158 verified, 0 errors
- After: 4176 verified, 0 errors (+18 obligations)
- RTT: 2613 passed
- PTT: 147 passed

## What Was Proven

All 5 `resize` functions across the hash table family:

1. **LinProbFlatHashTableStEph::resize** — removed external_body, added 3-phase proof
2. **QuadProbFlatHashTableStEph::resize** — same pattern as LinProb
3. **DoubleHashFlatHashTableStEph::resize** — same pattern as LinProb
4. **VecChainedHashTableStEph::resize** — inner loop with union_prefer_right chain tracking
5. **LinkedListChainedHashTableStEph::resize** — same pattern as Vec chained

## Technique

All resize functions share a 3-phase structure:

- **Phase 1**: Collect occupied pairs from old table into a Vec. Invariant tracks
  `spec_seq_pairs_to_map(pairs@) =~= spec_table_to_map(table[0..i])`.
  For flat tables, each slot is a single entry. For chained tables, an inner loop
  iterates chain entries with `union_prefer_right` tracking.
- **Phase 2**: Create empty table. Uses `lemma_table_to_map_push_empty` to prove
  `spec_table_to_map` of all-empty entries equals `Map::empty()`. Assert wf vacuously
  (no keys in any bucket).
- **Phase 3**: Reinsert all pairs via `Self::insert`. Invariant tracks
  `new_table@ =~= spec_seq_pairs_to_map(pairs@.subrange(0, j))`.

Key identity for chained tables:
`M.union_prefer_right(N).insert(k, v) =~= M.union_prefer_right(N.insert(k, v))`

Also made `lemma_table_to_map_push_empty` public in ParaHashTableStEph.rs.

## What Was Not Proven (and Why)

| # | Chap | File | Hole | Reason |
|---|------|------|------|--------|
| 1 | 47 | LinProbFlatHashTableStEph.rs | insert (external_body) | Deleted slot logic: inserts at first Empty/Deleted without checking if key exists further on probe chain. Needs code-level fix or correctness argument. |
| 2 | 47 | LinProbFlatHashTableStEph.rs | delete (external_body) | Depends on insert's wf invariant. |
| 3 | 47 | QuadProbFlatHashTableStEph.rs | insert (external_body) | Same Deleted slot issue. |
| 4 | 47 | QuadProbFlatHashTableStEph.rs | delete (external_body) | Same dependency. |
| 5 | 47 | DoubleHashFlatHashTableStEph.rs | insert (external_body) | Same Deleted slot issue. |
| 6 | 47 | DoubleHashFlatHashTableStEph.rs | delete (external_body) | Same dependency. |
| 7 | 47 | DoubleHashFlatHashTableStEph.rs | lookup (assume) | Bridges opaque second_hash to wf existential. |
| 8 | 47 | StructChainedHashTable.rs | resize (external_body) | Ordering mismatch: spec_chain_to_map gives head-wins but traversal produces tail-wins. Needs duplicate-free proof or restructuring. |
| 9 | 47 | ParaHashTableStEph.rs | call_hash_fn (external_body) | Intentional — opaque Fn closure. |

## Chapters Cleaned
- VecChainedHashTableStEph.rs — 0 holes (clean)
- LinkedListChainedHashTableStEph.rs — 0 holes (clean)
