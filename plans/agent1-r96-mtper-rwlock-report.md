# Agent 1 — R96 Report: MtPer rwlock assumes

## Objective

Prove 9 `rwlock:predicate` assumes in `AdjTableGraphMtPer.rs` using R95's
strengthened `OrderedTableMtPer` ensures.

## Result

**4 of 9 rwlock:predicate assumes removed. 4 additional algorithmic assumes removed.**

| # | Chap | File | Metric | Before | After | Delta |
|---|------|------|--------|--------|-------|-------|
| 1 | 52 | AdjTableGraphMtPer.rs | rwlock:predicate | 9 | 5 | -4 |
| 2 | 52 | AdjTableGraphMtPer.rs | algorithmic | 14 | 10 | -4 |
| 3 | 52 | AdjTableGraphMtPer.rs | total assumes | 23 | 15 | -8 |
| 4 | 52 | (all Chap52) | total holes | 34 | 26 | -8 |

## Assumes removed (8)

| # | Chap | File | Function | Assume | Classification | Technique |
|---|------|------|----------|--------|----------------|-----------|
| 1 | 52 | AdjTableGraphMtPer.rs | num_edges | `neighbors.spec_avltreesetmtper_wf()` | rwlock:predicate | graph closure + finiteness |
| 2 | 52 | AdjTableGraphMtPer.rs | has_edge | `neighbors.spec_avltreesetmtper_wf()` | rwlock:predicate | graph closure + finiteness |
| 3 | 52 | AdjTableGraphMtPer.rs | has_edge | `found == (dom.contains(u@) && ...)` | algorithmic | find ensures + set find ensures |
| 4 | 52 | AdjTableGraphMtPer.rs | out_neighbors | `dom.contains(u@) ==> neighbors@ == adj[u@]` | algorithmic | find ensures direct |
| 5 | 52 | AdjTableGraphMtPer.rs | out_neighbors | `!dom.contains(u@) ==> neighbors@ == empty` | algorithmic | find None + empty ensures |
| 6 | 52 | AdjTableGraphMtPer.rs | out_degree | `ns.spec_avltreesetmtper_wf()` | rwlock:predicate | strengthened out_neighbors ensures |
| 7 | 52 | AdjTableGraphMtPer.rs | insert_vertex (clone branch) | `updated.spec_adjtablegraphmtper_wf()` | rwlock:predicate | explicit adj.clone() preserves view |
| 8 | 52 | AdjTableGraphMtPer.rs | insert_vertex | `updated.spec_adj().dom().contains(v@)` | algorithmic | insert domain ensures + clone view |
| 9 | 52 | AdjTableGraphMtPer.rs | delete_edge | `u_neighbors.spec_avltreesetmtper_wf()` | rwlock:predicate | graph closure + finiteness |

Note: Row 7 was split from the original single assume — clone branch now proved,
insert branch retains assume.

## Key proof technique

`spec_avltreesetmtper_wf()` = `self@.finite()`. From graph closure, every neighbor
set is a subset of the domain: `adj[u] ⊆ dom`. Domain is finite (added to graph wf).
By `vstd::set_lib::lemma_len_subset`, subset of finite is finite. Chain:

1. `self.spec_adjtablegraphmtper_wf()` includes graph closure + domain finiteness
2. `find` ensures `self.adj@[u@] == neighbors@` (Some branch)
3. Graph closure: `adj[u@].contains(w) ==> dom.contains(w)` → `neighbors@ ⊆ dom`
4. `lemma_len_subset(neighbors@, dom)` → `neighbors@.finite()` → wf ✓

## Changes made

1. **Added `self.spec_adj().dom().finite()` to `spec_adjtablegraphmtper_wf`** — makes
   domain finiteness available from the graph wf predicate. Already true by
   `OrderedTableMtPer` type invariant; just made explicit.

2. **Strengthened `out_neighbors` ensures** — added `neighbors.spec_avltreesetmtper_wf()`.
   Enables `out_degree` to drop its wf assume.

3. **Restructured `has_edge`** — match branches carry proof through; no post-match
   assume needed. Verus resolves postcondition in each branch.

4. **Restructured `insert_vertex`** — split into if/else with per-branch proof.
   Clone branch uses explicit `self.adj.clone()` (OrderedTableMtPer clone has ensures;
   derived Clone on AdjTableGraphMtPer does not).

5. **Proved delete_edge `u_neighbors` wf** — find is on `self.adj` (not a modified
   copy), so graph closure applies directly.

## 5 remaining rwlock:predicate assumes — all blocked

| # | Chap | File | Function | Line | Blocker |
|---|------|------|----------|------|---------|
| 1 | 52 | AdjTableGraphMtPer.rs | insert_vertex (else) | 321 | insert value ensures |
| 2 | 52 | AdjTableGraphMtPer.rs | delete_vertex | 340 | map ensures (dom only) |
| 3 | 52 | AdjTableGraphMtPer.rs | insert_edge | 363 | intermediate table — no graph closure |
| 4 | 52 | AdjTableGraphMtPer.rs | insert_edge | 376 | insert value ensures |
| 5 | 52 | AdjTableGraphMtPer.rs | delete_edge | 411 | insert value ensures |

**Common blocker**: `OrderedTableMtPer`'s `external_body` methods (`insert`, `delete`,
`map`) provide domain-level ensures but NOT value-level ensures. Specifically:

- `insert` ensures `updated@.dom() =~= self@.dom().insert(k@)` but not `updated@[k@] == v@`
  or that other keys' values are preserved.
- `delete` ensures `updated@ == self@.remove(k@)` (strong) but `map` ensures only
  `mapped@.dom().finite()` (very weak).
- Graph closure requires knowing values at all keys to prove neighbors ⊆ domain.

**To unblock**: Strengthen `OrderedTableMtPer` external_body ensures:
- `insert`: add `updated@[k@] == v@` and `forall|j| j != k@ ==> updated@[j] == self@[j]`
- `map`: add `mapped@.dom() == self@.dom()` and value correspondence

## Validation

```
verification results:: 5385 verified, 0 errors
RTT: 3083 passed
PTT: 157 passed
```
