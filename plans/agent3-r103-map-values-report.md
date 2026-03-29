# Agent 3 — R103 Report: Strengthen MtPer map value ensures + prove delete_vertex

## Objective

Eliminate the last `assume` in `AdjTableGraphMtPer::delete_vertex` (Chap52) by
strengthening `OrderedTableMtPer::map` ensures to include value correspondence.

## Approach

**Delegated MtPer map to StPer map**, eliminating the manual iteration loop and
its `assume(result@.dom() =~= self@.dom())`. Three coordinated changes:

### 1. Changed MtPer map signature (Chap43/OrderedTableMtPer.rs)

- Closure type: `Fn(&K, &V) -> V` → `Fn(&V) -> V` (matches StPer)
- Added `Ghost(f_spec): Ghost<spec_fn(V::V) -> V::V>` parameter
- Requires: `f.ensures((&v,), r) ==> r@ == f_spec(v@)` (connects exec to spec)
- New ensures: `forall|k| self@.contains_key(k) ==> mapped@[k] == f_spec(self@[k])`

### 2. Rewrote MtPer map impl

Replaced 30-line manual loop + assume with 15-line delegation:
```
acquire_read → inner.map(f) → release_read → from_st_table(st_result)
```
Proof connects StPer's existential ensures to our f_spec ensures via `choose`.

### 3. Proved delete_vertex graph wf (Chap52/AdjTableGraphMtPer.rs)

Replaced `assume(updated.spec_adjtablegraphmtper_wf())` with full proof:
- Closure: `|neighbors| neighbors.delete(&v_clone)` with explicit ensures
- Ghost: `|ns| ns.remove(v_view)` matches closure behavior
- Graph closure: for each neighbor w of vertex u in cleaned graph,
  w was in self's neighbor set (map value ensures) and w ≠ v (removed),
  so w remains in domain by self's graph closure.

## Holes Before/After

| # | Chap | File | Holes Before | Holes After |
|---|------|------|-------------|-------------|
| 1 | 43 | OrderedTableMtPer.rs | 1 (map domain assume) | 0 |
| 2 | 52 | AdjTableGraphMtPer.rs | 1 (delete_vertex wf assume) | 0 |

## Verification

- **Full validation**: 5414 verified, 0 errors
- **RTT**: 3083 passed
- **PTT**: 157 passed
- **Chapter status**: 46 chapters, 44 clean, 2 holed, 8 holes (global), 244 modules

## Net: −2 holes (both eliminated, not moved)

The MtPer map domain assume was also eliminated as a side effect of delegating
to StPer map (which already proves domain equality).

## Techniques Used

- Ghost spec_fn companion pattern (from using_closures_standard.rs Pattern C)
- Delegation to already-proven StPer implementation
- Existential witness extraction via `choose` in proof blocks
- feq broadcast triggering for clone view equality
