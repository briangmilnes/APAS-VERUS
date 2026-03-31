# Agent 4 — R118 Chap52 Graph Representation Specs

## Summary

Strengthened specs across 7 Chap52 graph representation files: added 11 missing
functions, strengthened 5 ensures clauses, added 2 capacity requires, and added
1 missing spec fn. All changes verified clean.

## Results

- **Verified**: 5430 (was ~5430 baseline — no regressions)
- **RTT**: 3529 passed
- **PTT**: 221 passed
- **Errors**: 0

## Changes by file

| # | Chap | File | Before | After | Change |
|---|------|------|--------|-------|--------|
| 1 | 52 | AdjMatrixGraphMtPer.rs | 8 warnings | 0 | +from_matrix, +set_edge (2 fns) |
| 2 | 52 | AdjMatrixGraphMtEph.rs | 4 warnings | 0 | has_edge/out_degree: strict requires → conditional ensures + bounds checks |
| 3 | 52 | AdjSeqGraphStEph.rs | 2 warnings | 0 | +insert_edge, +delete_edge delegating to set_edge |
| 4 | 52 | AdjSeqGraphMtPer.rs | 3 warnings | 0 | +from_seq, +insert_edge, +delete_edge (full persistent impls) |
| 5 | 52 | AdjSeqGraphMtEph.rs | 2 warnings | 0 | +from_seq, +set_neighbors |
| 6 | 52 | AdjTableGraphStEph.rs | 3 warnings | 0 | vertices/has_edge/out_neighbors ensures strengthened to match StPer |
| 7 | 52 | EdgeSetGraphMtPer.rs | 3 warnings | 0 | +spec_out_neighbors, insert_vertex/insert_edge capacity requires |

## Detail

### AdjMatrixGraphMtPer.rs
- Added `from_matrix`: constructor from raw matrix, matching StPer.
- Added `set_edge`: persistent set-edge returning new graph, ported from StPer with MtPer types.

### AdjMatrixGraphMtEph.rs
- `has_edge`: removed strict `u < spec_n(), v < spec_n()` requires. Added conditional
  ensures matching MtPer pattern: in-bounds returns edge value, out-of-bounds returns false.
  Added runtime bounds check.
- `out_degree`: same pattern — conditional ensures, runtime bounds check.

### AdjSeqGraphStEph.rs
- Added `insert_edge(&mut self, u, v)` — delegates to `set_edge(u, v, true)`.
- Added `delete_edge(&mut self, u, v)` — delegates to `set_edge(u, v, false)`.

### AdjSeqGraphMtPer.rs
- Added `from_seq`: trivial constructor wrapping adj array.
- Added `insert_edge`: full persistent implementation ported from StPer with MtPer types.
  Checks for duplicate, builds new neighbor list, tabulates new adj array.
- Added `delete_edge`: filters v from u's neighbor list, tabulates new adj array.

### AdjSeqGraphMtEph.rs
- Added `from_seq`: trivial constructor.
- Added `set_neighbors(&mut self, v, neighbors)`: sets v's neighbor list, ported from StEph.

### AdjTableGraphStEph.rs
- `vertices`: added `ensures verts@ == self.spec_adj().dom()`. Required loop invariant
  additions tracking entry-key↔verts membership, plus final proof connecting entries to
  map domain via `lemma_entries_to_map_key_in_seq` and `lemma_entries_to_map_contains_key`.
- `has_edge`: added `ensures found == (dom.contains(u@) && adj[u@].contains(v@))`.
- `out_neighbors`: added conditional ensures matching StPer.

### EdgeSetGraphMtPer.rs
- Added `spec_out_neighbors` spec fn to trait and impl (set comprehension over edges).
- `insert_vertex`: added `spec_vertices().len() + 1 < usize::MAX` requires.
- `insert_edge`: added `spec_vertices().len() + 2 < usize::MAX` and
  `spec_edges().len() + 1 < usize::MAX` requires.

## Techniques
- Port-from-St: copied StPer implementations with type substitution for Mt variants.
- Delegation: StEph insert_edge/delete_edge delegate to set_edge.
- Conditional ensures: MtEph now matches MtPer's tolerant API (bounds check + conditional ensures).
- Loop invariant proof: vertices ensures required tracking processed-entries ↔ set membership.
