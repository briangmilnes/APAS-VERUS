# R141 Agent 4 — Cache num_edges in AdjTableGraphMtPer

## Summary

Cached `num_edges` as a `usize` field in `AdjTableGraphMtPer`, reducing `num_edges()`
from O(n+m) to O(1), matching APAS CS 52.3.

## Changes

### Struct
- Added `pub num_edges: usize` field to `AdjTableGraphMtPer`.

### Spec
- `spec_num_edges()` now returns `self.num_edges as nat` (cached field).
- Edge count invariant `spec_num_edges() == spec_sum_adj_sizes(adj@)` is threaded
  through requires/ensures on each operation rather than stored in wf. This avoids
  a Z3 matching loop issue where Z3 cannot extract a conjunct involving a recursive
  spec function from a conjunction (even when the function is closed/hidden).

### Operations updated

| # | Chap | File | Function | Edge count approach |
|---|------|------|----------|-------------------|
| 1 | 52 | AdjTableGraphMtPer.rs | `empty` | Set 0; prove base case |
| 2 | 52 | AdjTableGraphMtPer.rs | `num_edges` | Return `self.num_edges` — O(1) |
| 3 | 52 | AdjTableGraphMtPer.rs | `insert_vertex` | Unchanged; empty set adds 0 to sum |
| 4 | 52 | AdjTableGraphMtPer.rs | `delete_vertex` | `count_table_edges` on result |
| 5 | 52 | AdjTableGraphMtPer.rs | `insert_edge` | +1 if edge is new, +0 if exists |
| 6 | 52 | AdjTableGraphMtPer.rs | `delete_edge` | -1 if edge existed, -0 if not |
| 7 | 52 | AdjTableGraphMtPer.rs | `Clone` | Copy field |

### New proof infrastructure

- `spec_sum_adj_sizes` changed from `open` to `closed` to prevent Z3 matching loops.
  Lemmas that need the definition use `reveal(spec_sum_adj_sizes)`.
- `lemma_sum_adj_sizes_monotone`: proves sum decreases when value sets shrink
  (used by delete_vertex overflow proof).
- `count_table_edges`: exec helper that counts edges by iterating a table
  (used by delete_vertex to compute new edge count).

### Annotation update
- `num_edges` cost annotation changed from "DIFFERS: APAS assumes cached; impl sums
  degrees" to "matches APAS; cached field".

## Verification

```
verification results:: 5610 verified, 0 errors
RTT: 3634 passed
PTT: 221 passed
```

## Z3 matching loop note

Z3 cannot extract a conjunct containing `spec_sum_adj_sizes(m)` from a conjunction,
even when the function is `closed` (opaque). The issue appears to be that recursive
spec functions with `decreases` cause Z3 to attempt unfolding that leads to matching
loops. The workaround is to thread the invariant through requires/ensures instead of
storing it in the wf predicate. The `closed` annotation prevents matching loops in
other proof contexts; `reveal(spec_sum_adj_sizes)` is used in lemmas that need the
definition.
