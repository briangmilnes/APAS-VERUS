# R135 Agent 2 Report: Cache num_edges in Graph Modules

## Summary

Cached the edge count in 8 graph modules (4 AdjMatrix + 4 AdjSeq), making `num_edges()` O(1) instead of O(n²)/O(n+m). All proofs verified, no assumes/accepts added.

## Results

| # | Chap | File | Before | After | Status |
|---|------|------|--------|-------|--------|
| 1 | 52 | AdjMatrixGraphStEph.rs | O(n²) | O(1) | Verified |
| 2 | 52 | AdjMatrixGraphMtEph.rs | O(n²) | O(1) | Verified |
| 3 | 52 | AdjMatrixGraphStPer.rs | O(n²) | O(1) | Verified |
| 4 | 52 | AdjMatrixGraphMtPer.rs | O(n²) | O(1) | Verified |
| 5 | 52 | AdjSeqGraphStEph.rs | O(n+m) | O(1) | Verified |
| 6 | 52 | AdjSeqGraphMtEph.rs | O(n+m) | O(1) | Verified |
| 7 | 52 | AdjSeqGraphStPer.rs | O(n+m) | O(1) | Verified |
| 8 | 52 | AdjSeqGraphMtPer.rs | O(n+m) | O(1) | Verified |
| 9 | 52 | AdjTableGraphStEph.rs | O(n+m) | O(n+m) | Deferred |
| 10 | 52 | AdjTableGraphStPer.rs | O(n+m) | O(n+m) | Deferred |
| 11 | 52 | AdjTableGraphMtPer.rs | O(n+m) | O(n+m) | Deferred |

## What Changed Per File

### Struct
- Added `pub num_edges: usize` field to each graph struct.

### Wf Predicate
- Added invariant: `self.num_edges as nat == spec_sum_of(...)` relating cached count to actual edge count.

### Constructors
- `new()`: Initializes `num_edges: 0` with proof that sum of zero-degree/zero-row-count is 0.
- `from_matrix()`/`from_seq()`: Counts edges during construction (O(n²)/O(n+m)). Added `spec_sum_of(...) <= usize::MAX` requires for overflow safety.

### num_edges()
- Replaced O(n²)/O(n+m) counting loop with `self.num_edges` field return (O(1)).
- Removed overflow `requires` clause (wf guarantees count fits in usize).

### Mutation Functions
- `set_edge()` (Eph): Tracks ±1 change based on old value. Proof uses `lemma_sum_of_change_one` and `lemma_count_true_set_true/set_false` (matrix) or direct degree comparison (seq).
- `set_edge()` (Per): Returns new struct with adjusted count.
- `set_neighbors()`: Computes delta = new_deg - old_deg, updates accordingly.
- `insert_edge()`/`delete_edge()` (Per): Computes new count from old + delta.
- `complement()` (Matrix): Counts edges in new complement matrix.
- Added `spec_n() * spec_n() < usize::MAX` capacity requires to matrix mutation functions.
- Added `spec_sum_of(...) < usize::MAX` capacity requires to seq mutation functions.

### Proof Infrastructure (10 lemmas per file)
- `lemma_count_true_all_false` — zero count for all-false predicate
- `lemma_sum_of_all_zero` — zero sum for all-zero function
- `lemma_count_true_ext` — extensionality for count_true
- `lemma_count_true_set_true` — flip false→true increases count by 1
- `lemma_count_true_set_false` — flip true→false decreases count by 1
- `lemma_sum_of_ext` — extensionality for sum_of
- `lemma_sum_of_change_one` — one-term change formula
- `lemma_sum_of_lower_bound` — sum ≥ any single term
- `lemma_sum_of_bounded` — sum ≤ n × bound
- `lemma_count_true_at_least_one` — count ≥ 1 if predicate holds somewhere

### Clone/Debug
- Updated to include `num_edges` field.

## AdjTable Files Deferred

The 3 AdjTable files use Map-based specs (`spec_sum_adj_sizes` over `Map<V, Set<V>>`) with recursive sum over the map domain. Maintaining a cached count through `insert_vertex`, `delete_vertex`, `insert_edge`, `delete_edge` on these Map-based structures requires significantly more proof infrastructure (connecting map domain changes to sum changes). Deferred to a future round.

## Verification

- **Verified**: 5546 (was 5439 before; +107 from new lemmas and proof obligations)
- **RTT**: 3583 passed
- **PTT**: 221 passed
- **Errors**: 0
