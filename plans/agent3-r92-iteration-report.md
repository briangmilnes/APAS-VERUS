# Agent 3 — R92 Iteration Report

## Objective

Remove `external_body` from `num_edges` and `vertices` in Chap52 AdjTableGraph
(StEph and StPer).

## Results

| # | Chap | File | Function | Before | After | Notes |
|---|------|------|----------|--------|-------|-------|
| 1 | 52 | AdjTableGraphStEph.rs | num_edges | external_body | verified | Full ensures proved |
| 2 | 52 | AdjTableGraphStEph.rs | vertices | external_body | verified | No ensures to prove |
| 3 | 52 | AdjTableGraphStPer.rs | num_edges | external_body | verified | Full ensures proved |
| 4 | 52 | AdjTableGraphStPer.rs | vertices | external_body | verified | ensures verts@ == dom proved |

## Hole Counts

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Chap52 holes | 50 | 46 | -4 |
| Chap52 external_body | 6 | 2 | -4 |
| Full crate verified | 5373 | 5377 | +4 |
| RTTs | 3083 | 3083 | 0 |

## Techniques

### num_edges — Sum Permutation Proof

The spec `spec_num_edges` uses `spec_sum_adj_sizes(map)` which recursively decomposes
the map via `choose()`. The loop iterates entries sequentially. Bridging required:

1. **`spec_sum_entry_sizes(entries, n)`** — sequential partial sum over entry sequence.
2. **`lemma_sum_adj_remove(m, k)`** — extract any key from the recursive map sum,
   regardless of which key `choose()` picks. Proved by double-IH: extract k from
   `m.remove(chosen)` and extract chosen from `m.remove(k)`, then commutativity
   of `remove`.
3. **`lemma_sum_entry_sizes_eq(entries, n)`** — connect sequential sum to map sum.
   Induction on n: at each step, the last entry's key is fresh (no-dup), so
   `insert(k,v).remove(k) =~= original_map`.
4. **`lemma_sum_entry_sizes_monotone(entries, i, j)`** — partial sums are
   non-decreasing. Used for overflow bounds: `partial_sum <= total <= usize::MAX`.

### vertices — Direct Entry Iteration

Iterated `self.adj.entries` directly (not `domain().to_seq()`) to avoid a missing
`spec_arraysetsteph_wf()` ensures in the `domain()` trait signature. Used `clone_plus()`
+ `lemma_cloned_view_eq` for key clone view preservation.

For StPer's stronger ensures (`verts@ == self.spec_adj().dom()`), maintained forward
and backward containment invariants connecting the loop set to entry keys, then
bridged to `spec_entries_to_map` domain at loop exit using existing TableStPer lemmas.

### StPer Bridge Lemmas

StPer has its own `spec_entries_to_map` (identical body, different module). Added
`lemma_entries_to_map_eq` proving StPer's equals StEph's by induction, enabling
reuse of the sum proof infrastructure.

### Capacity Bounds

Added `self.spec_adj().dom().len() < usize::MAX as nat` to `vertices` requires
(both StEph and StPer) per capacity bounds standard. Needed because
`AVLTreeSetStEph::insert` requires `old@.len() + 1 < usize::MAX`.

## ICE Avoidance

No quantifiers over `Map<V::V, Set<V::V>>` in proof bodies. All sum lemmas are
generic (`<VV>`) and work with `Seq<(VV, Set<VV>)>` directly. The Verus ICE
on `Set<V::V>` was not triggered.

## Remaining Chap52 Holes

The 2 remaining `external_body` functions are both `delete_vertex` (StEph + StPer),
which require iterating the domain AND modifying neighbor sets — a combination of
the iteration pattern proved here plus the clone/wf gap in graph closure proofs.

## Iterations Used

7 validate cycles (of 20 budget).
