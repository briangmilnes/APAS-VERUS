# R95 Agent 3 Report: Clone-View Gap Fix in AdjTableGraph

## Objective

Remove 4 assumes caused by generic clone-view gap (`v.clone()@ == v@` unprovable
for `V: StT + Ord`) in AdjTableGraphStEph and AdjTableGraphStPer insert_edge/delete_edge.

## Technique

**ClonePreservesView + delete-then-insert.**

1. Added `ClonePreservesView` to V bounds on trait and impl in both files.
2. Replaced `v.clone()` with `v.clone_view()` in set insert/singleton calls,
   giving `ensures result@ == self@` so set operations prove `contains(v@)`.
3. Used **delete-then-insert** pattern to bypass the combine closure's clone gap.
   The table insert API uses `combine: Fn(&V, &V) -> V` whose `new.clone()` return
   has no view postcondition. By deleting the key first, the insert hits the
   "key absent" branch where `self@[key@] == value@` directly — no combine call.
4. For StPer delete_edge's None case, replaced `self.clone()` (struct clone gap)
   with `self.adj.delete(u)` (no-op on absent key) to get an owned table with
   provable view equality.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 52 | AdjTableGraphStEph.rs | 7 | 5 | -2 |
| 2 | 52 | AdjTableGraphStPer.rs | 7 | 6 | -1 |

**Total: -3 net assumes** (4 clone-view assumes removed, 1 wf assume added).

### Assumes removed (4)

| # | Chap | File | Function | Assume |
|---|------|------|----------|--------|
| 1 | 52 | AdjTableGraphStEph.rs | insert_edge | `assume(self.spec_adj()[u_view].contains(v_view))` |
| 2 | 52 | AdjTableGraphStEph.rs | delete_edge | `assume(!dom.contains(u@) \|\| !adj[u@].contains(v@))` |
| 3 | 52 | AdjTableGraphStPer.rs | insert_edge | `assume(updated.spec_adj()[u_view].contains(v_view))` |
| 4 | 52 | AdjTableGraphStPer.rs | delete_edge | `assume(!dom.contains(u@) \|\| !adj[u@].contains(v@))` |

### Assume added (1)

| # | Chap | File | Function | Assume | Reason |
|---|------|------|----------|--------|--------|
| 1 | 52 | AdjTableGraphStPer.rs | delete_edge | `assume(updated.spec_adjtablegraphstper_wf())` | Early return path needs wf (ICE-blocked, same category as existing wf assumes) |

## Remaining holes in these files

All remaining assumes are ICE-related (`forall` over `Set<V::V>` triggers Verus ICE)
or neighbor-set wf propagation through table operations. These are distinct from the
clone-view gap and require Verus-level fixes.

### AdjTableGraphStEph.rs (5 assumes)

| # | Function | Category |
|---|----------|----------|
| 1 | insert_vertex | wf (ICE) |
| 2 | delete_vertex | ns_ref wf (ICE) |
| 3 | delete_vertex | wf (ICE) |
| 4 | insert_edge | wf (ICE) |
| 5 | delete_edge | wf (ICE) |

### AdjTableGraphStPer.rs (6 assumes)

| # | Function | Category |
|---|----------|----------|
| 1 | insert_vertex | wf (ICE) |
| 2 | delete_vertex | ns wf (ICE) |
| 3 | delete_vertex | wf (ICE) |
| 4 | insert_edge | wf (ICE) |
| 5 | delete_edge (early return) | wf (ICE) |
| 6 | delete_edge (main path) | wf (ICE) |

## Verification

- 5386 verified, 0 errors
- 3083 RTT passed
- 157 PTT passed
- Total holes: 37 (down from 42)
