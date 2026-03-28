# Agent 2 — R92 Report: Prove Chap52 Assumes

## Summary

Proved 7 assumes across AdjTableGraph×3 files. All remaining assumes are
blocked by one of three systemic issues: Verus ICE on `Set<V::V>` quantifiers,
generic clone gap (`v.clone()@` ≠ `v@` for generic V), or weak
OrderedTableMtPer postconditions.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 52 | AdjTableGraphStEph.rs | 10 | 8 | -2 |
| 2 | 52 | AdjTableGraphStPer.rs | 12 | 10 | -2 |
| 3 | 52 | AdjTableGraphMtPer.rs | 25 | 22 | -3 |
| | | **Total Chap52** | **50** | **43** | **-7** |

## Assumes Proved

| # | Chap | File | Function | Assume | Technique |
|---|------|------|----------|--------|-----------|
| 1 | 52 | AdjTableGraphMtPer.rs | empty | wf | Type-level preds from requires + vacuous graph closure on empty map |
| 2 | 52 | AdjTableGraphStPer.rs | out_neighbors | `dom.contains(u@) ==> ns@ == adj[u@]` | Direct from TableStPer::find ensures |
| 3 | 52 | AdjTableGraphStPer.rs | out_neighbors | `!dom.contains(u@) ==> empty@ == Set::empty()` | find None ensures + AVLTreeSetStPer::empty ensures |
| 4 | 52 | AdjTableGraphStEph.rs | insert_edge | `dom().contains(u@)` | Table insert ensures dom |
| 5 | 52 | AdjTableGraphStEph.rs | insert_edge | `dom().contains(v@)` | Conditional second insert + domain preservation |
| 6 | 52 | AdjTableGraphStPer.rs | insert_edge | `dom().contains(u@)` | Table insert ensures dom |
| 7 | 52 | AdjTableGraphStPer.rs | insert_edge | `dom().contains(v@)` | Conditional second insert + domain preservation |

## Blockers Analysis

### 1. Verus ICE on `Set<V::V>` in quantifiers
Any `assert forall` over `Map<V::V, Set<V::V>>` crashes Verus. This blocks
proving graph closure (`forall|u, v| adj.dom().contains(u) && adj[u].contains(v)
==> adj.dom().contains(v)`) and stored-value wf quantifiers. Affects wf
assumes in insert_vertex, insert_edge, delete_edge across all three files.

### 2. Generic clone gap (`v.clone()@ ≠ v@`)
For generic `V: StT + Ord`, Verus cannot prove `v.clone()@ == v@` because:
- `StT` does not include `ClonePreservesView`
- The `obeys_feq_clone` broadcast mechanism uses `cloned(x, y)` markers that
  Verus doesn't generate for generic Clone impls
- This blocks proving `adj[u@].contains(v@)` after set insert/singleton
  (which operates on `v.clone()`, not `v`)
- Also blocks proving map values are preserved through combine closures
  (`|_old, new| new.clone()`)

### 3. Weak OrderedTableMtPer postconditions
`OrderedTableMtPer::find` has NO ensures. `insert` only ensures
`updated@.dom().finite()`. `delete` only ensures `updated@.dom().finite()`.
All MtPer assumes except empty() are blocked by this weak API.

## Remaining Assumes by Category

| Category | StEph | StPer | MtPer | Total |
|----------|-------|-------|-------|-------|
| wf (ICE + clone gap) | 4 | 3 | 0 | 7 |
| adj[u].contains(v) (clone gap) | 1 | 1 | 1 | 3 |
| dom.contains (MtPer weak API) | 0 | 0 | 3 | 3 |
| Capacity bounds | 1 | 1 | 5 | 7 |
| Set wf (MtPer weak API) | 0 | 0 | 3 | 3 |
| found == ... (MtPer weak API) | 0 | 0 | 1 | 1 |
| out_neighbors (MtPer weak API) | 0 | 0 | 2 | 2 |
| Postconditions (clone gap) | 1 | 1 | 1 | 3 |
| **Total assumes** | **7** | **6** | **16** | **29** |

Note: external_body holes (3+3+2=8) and rwlock assumes (0+0+7=7) are separate.

## Verification

- `scripts/validate.sh`: 5363 verified, 0 errors
- `scripts/rtt.sh`: 3083 passed
- `scripts/ptt.sh`: 157 passed

## Steps Used: 6 of 20
