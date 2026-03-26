# Agent 4 — Round 82b Report

## Objective

Fix 8 verification errors in `EdgeSetGraphStEph.rs` and `EdgeSetGraphStPer.rs` (Chap52).

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 52 | EdgeSetGraphStEph.rs | 4 | 0 | -4 |
| 2 | 52 | EdgeSetGraphStPer.rs | 4 | 0 | -4 |

**Total: 8 holes closed, 0 remaining.**

Chap52: 14 modules clean, 0 holed.

## Verification

- `scripts/validate.sh`: 4772 verified, 0 errors
- `scripts/ptt.sh`: 157 passed, 0 skipped
- `scripts/rtt.sh`: pre-existing Chap55 import failures (not from this change)

## Techniques

### 1. ClonePreservesView bound

Added `ClonePreservesView` to `V: StT + Ord` bounds. Required because `clone()` on generic V
has no view-preserving postcondition. Used `clone_view()` instead of `clone()` where view
preservation is needed (insert_edge vertex insertion, out_neighbors element extraction,
delete_vertex vertex clone).

### 2. PartialEq ↔ View bridge via lemma

Created `lemma_eq_spec_iff_view_eq<V>` proof function that reveals opaque
`obeys_cmp_partial_ord` and `obeys_cmp_ord` to establish
`forall|a: V, b: V| a.eq_spec(&b) <==> a@ == b@`.

Called before filter closures to make the bridge available in the closure's verification
context. Named closures with explicit `ensures keep == (edge@.0 == u@)` then verified
automatically once the bridge was in scope.

### 3. Capacity bounds via find-before-insert

The AVLTreeSet insert precondition (`self@.len() + 1 < usize::MAX`) can't be satisfied
when `neighbors@.len() == usize::MAX - 1` (off-by-one from the wf bound
`self@.len() < usize::MAX`). Solved by checking `!neighbors.find(&v)` before inserting,
skipping redundant inserts. When inserting a genuinely new element,
`neighbors@.insert(v@) ⊆ self.vertices@` via `lemma_len_subset` gives the tight bound.

### 4. wf preservation via old(self) triggers

Mutation functions (insert_vertex, insert_edge, delete_edge) need to prove the edge
invariant (`forall|u,v| edges.contains((u,v)) ==> vertices.contains(u) && vertices.contains(v)`)
is preserved. Used `old(self).spec_edges().contains(...)` as trigger to fire the old wf
quantifier, connecting old edge membership to old vertex membership, then showing new
vertices/edges preserve the invariant.

### 5. delete_vertex in StEph (iterative approach)

StEph's delete_vertex uses iteration + deletion (not filter, since it's mutable). Loop
invariant tracks `self.edges@.subset_of(old(self).edges@)` and
`old(self).spec_edgesetgraphsteph_wf()` (immutable old wf). After the loop, edges touching
v are removed; remaining edges have endpoints `!= v@`, so they're in
`old(self).vertices@.remove(v@)`.

### 6. delete_vertex in StPer (filter approach)

StPer's delete_vertex uses named closure with `ensures keep == (edge@.0 != v@ && edge@.1 != v@)`,
leveraging the eq_spec bridge from the lemma. Filter's postconditions directly give the
wf proof: filtered edges don't touch v, so their endpoints are in `vertices.remove(v@)`.
