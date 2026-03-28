# R95 Agent 4 Report: StEph/StPer insert_edge & delete_edge proof work

## Summary

Proved edge-membership postconditions and graph closure invariants in
AdjTableGraphStEph and AdjTableGraphStPer. The "capacity off-by-one" assumes
were already removed by agent2 in R92. This round focused on the remaining
provable assumes: edge membership, edge removal postconditions, and graph
closure within the wf predicate.

## Technique

Two key insights enabled the proofs:

1. **Combine closure ensures**: Adding explicit `ensures r@ == new@` to the
   `|_old, new| new.clone()` combine closures lets Verus propagate
   `AVLTreeSetSt{Eph,Per}::clone` ensures (`cloned@ == self@`) through
   `Table::insert`'s existential postcondition. This proves
   `adj[u@] == neighbors@` after insert.

2. **clone_plus + lemma_cloned_view_eq**: Using `v.clone_plus()` instead of
   `v.clone()` gives `cloned(v, vc)`, then `lemma_cloned_view_eq` proves
   `vc@ == v@`. This bridges the clone gap for the element value, proving
   `neighbors@.contains(v@)`.

3. **Graph closure via subset**: For insert_edge, proved
   `neighbors_view.subset_of(old_dom.insert(v@))` from the old graph closure
   invariant. For delete_edge, used `nn_view ⊆ old_adj[u@] ⊆ old_dom`.
   For insert_vertex, case-split on whether the vertex was new.

## Results

| # | Chap | File | Function | Before | After | Delta | What changed |
|---|------|------|----------|--------|-------|-------|-------------|
| 1 | 52 | AdjTableGraphStEph.rs | insert_vertex | 1 (full wf) | 1 (stored-value wf) | 0 | graph closure proved |
| 2 | 52 | AdjTableGraphStEph.rs | insert_edge | 2 (wf + edge-member) | 1 (stored-value wf) | -1 | edge membership + graph closure proved |
| 3 | 52 | AdjTableGraphStEph.rs | delete_edge | 2 (wf + postcond) | 1 (stored-value wf) | -1 | postcondition + graph closure proved |
| 4 | 52 | AdjTableGraphStEph.rs | delete_vertex | 2 (inner + outer) | 2 (inner + outer) | 0 | unchanged (loop + clone gap) |
| 5 | 52 | AdjTableGraphStPer.rs | insert_vertex | 1 (full wf) | 1 (stored-value wf) | 0 | graph closure proved |
| 6 | 52 | AdjTableGraphStPer.rs | insert_edge | 2 (wf + edge-member) | 1 (stored-value wf) | -1 | edge membership + graph closure proved |
| 7 | 52 | AdjTableGraphStPer.rs | delete_edge | 2 (wf + postcond) | 2 (stored-value wf + clone wf) | 0 | postcondition + graph closure proved (true branch); clone branch needs wf |
| 8 | 52 | AdjTableGraphStPer.rs | delete_vertex | 2 (inner + outer) | 2 (inner + outer) | 0 | unchanged (loop + clone gap) |

**StEph: 7 → 5 (-2). StPer: 7 → 6 (-1). Net: -3 holes.**

## Remaining assumes (all clone-gap blocked)

All remaining assumes are `stored-value wf` quantifiers or full wf.
They are blocked by the **Table::insert combine clone gap**: when
Table::insert calls `combine(old, new)`, the result is `old.clone()` or
`new.clone()`. The clone preserves view (`result@ == self@`) but NOT
exec-level well-formedness (`result.spec_avltreesetsteph_wf()`), because
wf includes `self.tree.spec_bstparasteph_wf()` which depends on internal
BST structure, not just the set view.

To resolve, one of:
- Add a `clone_wf` variant of Table::insert that uses `clone_wf` in the
  combine, requiring stored-value wf as a loop invariant.
- Add `spec_stored_value` preservation to Table::insert ensures (for keys
  not being updated).
- Wait for Verus to add view-determined wf support.

## Verification

- Full validate: 5386 verified, 0 errors
- RTT: 3083 passed
- PTT: 157 passed

## Chap52 holes

- Before: 35
- After: 32
- Delta: -3
