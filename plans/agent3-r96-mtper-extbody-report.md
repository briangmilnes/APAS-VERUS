# Agent 3 R96 Report: AdjTableGraphMtPer assume reduction

## Objective

Reduce `assume()` holes in `AdjTableGraphMtPer.rs` using the strengthened
OrderedTableMtPer `find`/`insert`/`delete` ensures from R95 agent1.

## Results

| # | Metric | Before | After | Delta |
|---|--------|--------|-------|-------|
| 1 | Total holes | 23 | 19 | -4 |
| 2 | Algorithmic assumes | 13 | 10 | -3 |
| 3 | RwLock:predicate assumes | 10 | 9 | -1 |
| 4 | Verified count (full) | 5385 | 5385 | 0 |
| 5 | RTT | 3083 | 3083 | 0 |

## Changes by function

| # | Chap | Function | Assumes before | Assumes after | Technique |
|---|------|----------|----------------|---------------|-----------|
| 1 | 52 | out_neighbors | 2 | 0 | find ensures chain through clone/empty |
| 2 | 52 | has_edge | 2 | 1 | find ensures prove postcondition; wf assume stays |
| 3 | 52 | insert_vertex | 2 | 1 | adj.clone() (has ensures) replaces self.clone() (no ensures) in Some branch; None branch: insert ensures prove dom.contains |
| 4 | 52 | delete_edge | 3 | 2 | adj.clone() proves wf + postcondition in None branch; Some branch assumes stay |
| 5 | 52 | num_edges | 3 | 3 | No change (loop-sum proof needs domain/value coupling) |
| 6 | 52 | delete_vertex | 2 | 2 | No change (map ensures too weak) |
| 7 | 52 | insert_edge | 7 | 7 | No change (multi-step mutation, value preservation gap) |
| 8 | 52 | out_degree | 1 | 1 | No change (wf of returned set not provable) |
| 9 | 52 | empty | 0 | 0 | Already proved |
| 10 | 52 | num_vertices | 0 | 0 | Already proved |

## Key technique: adj.clone() vs self.clone()

The derived `Clone` on `AdjTableGraphMtPer` has no Verus ensures.
`OrderedTableMtPer::clone()` ensures `cloned@ == self@`. By constructing
`AdjTableGraphMtPer { adj: self.adj.clone() }` instead of `self.clone()`,
we get `updated.spec_adj() =~= self.spec_adj()`, enabling proof of graph
closure and postconditions via the self's wf from requires.

## Remaining blockers

| # | Blocker | Affects | Count |
|---|---------|---------|-------|
| 1 | OrderedTableMtPer::insert doesn't ensure value preservation (`updated@[k@] == v@`, old values preserved) | insert_vertex None, insert_edge, delete_edge Some | 9 assumes |
| 2 | OrderedTableMtPer::find doesn't ensure returned value's exec-level wf (spec_avltreesetmtper_wf) | has_edge, out_degree, num_edges, insert_edge, delete_edge | 5 assumes |
| 3 | OrderedTableMtPer::map ensures only dom.finite, not value properties | delete_vertex | 2 assumes |
| 4 | Loop-sum coupling (domain iteration + value lookup = sum) | num_edges | 3 assumes |

Adding `updated@[k@] == v@` and value preservation to OrderedTableMtPer::insert
would unlock the largest batch (~9 assumes). This is a tractable next step since
OrderedTableStPer::insert already computes these values correctly.

## Verification

```
Full validate: 5385 verified, 0 errors (150s)
Isolate Chap52: 2808 verified, 0 errors (33s)
RTT: 3083 passed, 0 skipped
```
