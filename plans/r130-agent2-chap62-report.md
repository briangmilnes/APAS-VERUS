# R130 Agent 2 — Chap62 StarPartitionMtEph: HashMap→OrderedTable Assessment + Loop Parallelization

## Summary

Investigated replacing `HashMapWithViewPlus` with `OrderedTableMtEph` in `src/Chap62/StarPartitionMtEph.rs`.
Found that OrderedTable cannot replace HashMap for 2 of 3 maps due to a fundamental type constraint.
Parallelized Loops 4 and 6 using D&C + join instead.

## OrderedTable Migration Assessment

### Type Constraint Gap

`OrderedTableStEph<K, V>` requires `K: StT + Ord` and `V: StT + Ord`. The `StT` trait
requires `View`. Primitive types `usize` and `bool` do **not** implement `View` in Verus vstd.

| # | Chap | Map | Key | Value | Can Replace? | Reason |
|---|------|-----|-----|-------|--------------|--------|
| 1 | 62 | vertex_to_index | V | usize | No | `usize` has no `View` impl → not `StT` |
| 2 | 62 | coin_flips | V | bool | No | `bool` has no `View` impl → not `StT` |
| 3 | 62 | partition_map | V | V | Yes (but regresses work) | V already `StT + Ord` |

Replacing only `partition_map` would be inconsistent and would regress build cost from
O(n) to O(n log n) without offsetting benefits. Decision: **keep all three as HashMap**.

### Work/Span Trade-off

- HashMap: O(1) expected lookup → O(n + m) total work
- OrderedTable: O(log n) lookup → O((n + m) log n) total work
- APAS specifies O(n + m) work, O(lg n) span
- OrderedTable would regress work while only helping span (which is already dominated by
  the sequential loops, not lookups)

## Parallelization Results

### Loops Parallelized

| # | Loop | Before | After | Work | Span |
|---|------|--------|-------|------|------|
| 1 | 4 (copy vertices to p_vec) | Sequential O(n) | D&C + join O(n) work, O(lg n) span | O(n) | O(lg n) |
| 2 | 6 (build centers + partition_map) | Sequential O(n) | D&C + join O(n) work, O(lg n) span | O(n) | O(lg n) |

### Loops Not Parallelized

| # | Loop | Reason |
|---|------|--------|
| 1 | 1 (vertex_to_index build) | HashMap sequential build; would need OrderedTable.tabulate which requires ArraySetStEph input |
| 2 | 5 (apply th_edges priority write) | Multiple th_edges may target same index; needs inject/ninject which is also sequential in current ArraySeqMtEph impl |

### New D&C Helpers Added

| # | Chap | Function | Purpose | Work | Span |
|---|------|----------|---------|------|------|
| 1 | 62 | `build_p_vec_mt` | Parallel clone of vertex slice into Vec | O(n) | O(lg n) |
| 2 | 62 | `build_partition_map_mt` | Parallel build of vertex→center HashMap | O(n lg n) | O(lg n) |
| 3 | 62 | `build_centers_mt` | Parallel build of centers SetStEph | O(n) | O(lg n) |

### DIFFERS Annotation Update

Before: `Loops 2, 3 use parallel D&C; loops 1, 4, 5, 6 remain sequential`
After: `Loops 1, 5 sequential; loops 2, 3, 4, 6 parallel D&C`

## Verification Results

- **Isolate Chap62**: 1333 verified, 0 errors, 0 warnings
- **Full crate**: 5549 verified, 0 errors
- **RTT**: 3536 passed
- **PTT**: 221 passed

## Proof Techniques

- **Ghost domain witnesses**: Used `ghost Map<V::V, int>` to track which vertex index
  witnesses each HashMap key, avoiding Z3 trigger issues with uninterpreted HashMap views.
  This solved the problem where Z3 couldn't connect `pre_merged.contains_key(v_view)` to
  the loop invariant's existential after a HashMap mutation.

- **Arc view invariant**: `HashMapWithViewPlus::view()` is `uninterp`, so Z3 can't connect
  Arc-cloned HashMap views to the original. Solved by removing `coin_flips` parameter from
  `build_centers_mt` (not needed at exec level — only compares vertex equality via `feq`).

- **No-duplicates contradiction**: Standard Verus pattern using `lemma_reveal_view_injective`
  to show that distinct indices in a no-duplicates sequence have distinct views.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 62 | StarPartitionMtEph.rs | Added 3 D&C helpers; parallelized loops 4, 6; updated DIFFERS |
