# R99 Agent 2 Report — insert_edge capacity fix

## Summary

Fixed 1 of 3 assigned holes. The capacity assume in `insert_edge` is proven.
The 2 `delete_vertex` holes are blocked by systematically weak ensures in the
MtPer map/collect/from_st_table chain — not fixable without multi-file strengthening.

## Hole fixed

| # | Chap | File | Function | Line | What |
|---|------|------|----------|------|------|
| 1 | 52 | AdjTableGraphMtPer.rs | insert_edge | 463 | capacity assume for 3rd insert_wf |

**Fix**: Changed requires from `dom.len() + 2` to `dom.len() + 3`. The function
does 3 conditional `insert_wf` calls (add u, add v, update u's neighbors). Each
`insert_wf` requires `dom.len() + 1 < usize::MAX`. After matches 1+2, domain
grows by at most 2, so the 3rd insert needs `orig + 3`. Added domain size tracking
assertions after each match arm. The assume is replaced with a proved assertion chain.

No callers in verified code (graph algorithms in Chap53-55 don't call insert_edge).
RTT callers are unaffected (no requires at runtime).

## Holes NOT fixed (blocked)

| # | Chap | File | Function | Line | What | Blocker |
|---|------|------|----------|------|------|---------|
| 2 | 52 | AdjTableGraphMtPer.rs | delete_vertex | 362 | graph wf after map | weak map ensures |
| 3 | 52 | AdjTableGraphMtPer.rs | delete_vertex | 363 | dom exclusion after map | weak map ensures |

**Analysis**: `delete_vertex` calls `self.adj.delete(v)` (strong ensures: `remove(v@)`)
then `without_v.map(|_k, ns| ns.delete(&v_clone))` (weak ensures: only `dom.finite()`).
Proving graph closure and domain exclusion requires map to ensure:
- `mapped@.dom() == self@.dom()` (domain preservation)
- Value-level correspondence (each neighbor set has v removed)

The MtPer map's ensures chain is systematically weak:
- `BSTParaMtEph::in_order` → only length
- `OrderedTableStPer::collect` → only length + wf
- `OrderedTableMtPer::map` → only `dom.finite()`
- `from_st_table` → only `spec_orderedtablemtper_wf()` (and has its own assume)

Fixing this requires strengthening 4+ functions across Chap38 and Chap43, including
the `collect` ensures to relate entries to domain/values, the map loop invariant to
track domain membership, and `from_st_table` to propagate the abstract view. This is
a multi-round project, not a STEP 15 task.

## Verification

```
validate:  5391 verified, 0 errors
rtt:       3083 passed, 0 skipped
ptt:       157 passed, 0 skipped
```

## Holes before/after

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 52 | AdjTableGraphMtPer.rs | 5 | 4 | -1 |
