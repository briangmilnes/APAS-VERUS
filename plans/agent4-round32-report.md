# Agent 4 — Round 32 Report

## Summary

R32 removed 4 proof holes across Chap45 and Chap47, with one cascade assume added to Chap57.

- **4121 verified, 0 errors**
- **2613 RTT pass**
- Net hole change: **-3** (4 removed, 1 cascade assume added)

## Tasks Completed

| # | Chap | File | Task | Result |
|---|------|------|------|--------|
| 1 | 47 | QuadProbFlatHashTableStEph.rs | Remove lookup assume | Proved (wrapping_add + mod) |
| 2 | 47 | DoubleHashFlatHashTableStEph.rs | Remove lookup wrapping assume | Proved (wrapping_add) |
| 3 | 45 | BalancedTreePQ.rs | Remove insert external_body | Proved (values_in_order + Vec::insert + Seq::insert multiset) |
| 4 | 45 | BinaryHeapPQ.rs | Remove extract_all_sorted assume | Proved (heap+sorted+boundary loop invariants) |

## Per-File Hole Changes

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 45 | BinaryHeapPQ.rs | 1 | 0 | -1 |
| 2 | 45 | BalancedTreePQ.rs | 2 | 1 | -1 |
| 3 | 47 | QuadProbFlatHashTableStEph.rs | 1 | 0 | -1 |
| 4 | 47 | DoubleHashFlatHashTableStEph.rs | 1 | 0 | -1 |
| 5 | 57 | DijkstraStEphI64.rs | 4 | 5 | +1 |

## Techniques Used

### BinaryHeapPQ extract_all_sorted (Task 4 — most complex)

Multi-session effort across 3 context windows:

1. **bubble_down_heap**: New ownership-transferring sift-down that uses `vec_swap` for
   T-level identity preservation. Ensures full heap property + multiset equality.

2. **delete_min strengthening**: Added heap preservation, root identity, and
   `le(min, remaining[i])` to ensures. Proof uses:
   - `spec_is_exec_heap_except(seq, 0)` after copying last element to position 0
   - `bubble_down_heap` to restore full heap property
   - `lemma_heap_root_le_all` + `lemma_le_preserved_by_multiset_eq` for le ensures

3. **extract_all_sorted proof**: Three-invariant loop:
   - `spec_is_exec_heap(current_heap.spec_seq())` — heap maintained via delete_min ensures
   - `spec_sorted(result.seq@)` — sorted by appending min elements
   - `forall|r, h| le(result[r], heap[h])` — boundary via `T::transitive`

4. **Clone strengthening**: Added T-level seq equality to `BinaryHeapPQ::clone` ensures,
   used to transfer heap property from self to clone at the start of extract_all_sorted.

5. **lemma_le_preserved_by_multiset_eq**: New proof fn transferring `le(min, a[j])` across
   multiset-equal sequences via bidirectional `to_multiset_contains`.

### BalancedTreePQ insert (Task 3 — runtime fix)

Original implementation used `values_in_order() + push(element)` which appended at end,
breaking sorted order. Fixed using `values_in_order() + Vec::insert(pos, element)`.
Key proof: `Seq::insert_ensures` + pointwise extensional equality connecting
`old_vals.insert(pos, element).map_values(view_fn)` to `self@.insert(pos, element@)`,
then `to_multiset_insert` (broadcast) gives the multiset postcondition.

### Chap47 lookup probes (Tasks 1-2)

Pattern: `wrapping_add` ensures `result == (a + b) % usize::MAX`, then `% table_size`
gives `result < table_size`. DoubleHash also needed `wrapping_mul` for second hash.

## Cascade

DijkstraStEphI64 now requires `spec_is_exec_heap(pq.spec_seq())` for delete_min.
Added as assume (not provable until insert/singleton guarantee heap property).
This is +1 hole in Chap57.

## Remaining Holes in Assigned Files

| # | Chap | File | Holes | What blocks |
|---|------|------|-------|-------------|
| 1 | 45 | BalancedTreePQ.rs | 1 | External impl block (BalancedTreePQExtTrait) |
| 2 | 57 | DijkstraStEphI64.rs | 5 | 2 external_body (Ord/PartialOrd), 2 assume (pq size + heap), 1 cascade |
