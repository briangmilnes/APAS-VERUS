# Agent 1 Round 83 Report

## Objective

Remove `external_body` from `dfs` in `DFSStEph.rs` and `DFSStPer.rs`.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 55 | DFSStEph.rs | 1 | 0 | -1 |
| 2 | 55 | DFSStPer.rs | 1 | 0 | -1 |

**Chap55 total**: 14 holes → 12 holes (-2).
**Global**: 5138 verified, 0 errors.

## Technique: Gray Set Ghost Parameter for DFS Completeness

The key challenge was proving the `spec_reachable` equivalence — that the returned
set equals exactly the vertices reachable from the source. This requires both:

- **Soundness**: everything in the result is reachable from source
- **Completeness**: everything reachable from source is in the result

### Soundness

Each `dfs_recursive` call ensures new entries are reachable from the call's vertex.
Path extension (`lemma_reachable_step`) chains edge + reachability into
reachability from the parent vertex.

### Completeness via Neighbor-Closure

Completeness uses the "neighbor-closure" property: if every visited vertex has all
its graph neighbors also visited, then any vertex reachable from a visited source
must also be visited. Four proof lemmas establish this:

1. `lemma_reachable_self` — trivial self-reachability
2. `lemma_reachable_step` — edge + reachability = reachability (path concatenation)
3. `lemma_neighbor_closed_path` — induction on path length: if visited is
   neighbor-closed, all path vertices are visited
4. `lemma_neighbor_closed_implies_reachable` — extracts path from
   `spec_reachable`, applies path lemma

### The Gray Set

During DFS, a vertex is visited *before* all its neighbors are recursed on,
temporarily breaking neighbor-closure. A ghost `Set<int>` parameter tracks
vertices "in progress" (on the recursion stack). Neighbor-closure is guaranteed
for all visited vertices *except* those in the gray set. After processing all
neighbors, the vertex exits the gray set and becomes neighbor-closed.

For recursive calls: `gray_inner = gray.insert(vertex)`. Since we only recurse on
unvisited neighbors (`if !visited[neighbor]`), unvisited neighbors are not in
`gray_inner` (gray vertices are always visited), satisfying the precondition.

### Trigger Fix on `spec_is_path`

The edge quantifier in `spec_is_path` had trigger `#[trigger] spec_has_edge(...)`.
Since `spec_has_edge` is an `open spec fn`, it gets inlined by Verus, destroying
the trigger term in Z3. Changed to `#![trigger path[k]]` which survives inlining.
Same fix applied to `spec_is_path_per` in `TopoSortStPer.rs`.

### Trait Requires Addition

Added `graph@.len() < usize::MAX` to `DFSStEphTrait::dfs` and
`DFSStPerTrait::dfs` requires. Needed because `dfs_recursive` requires this for
the AVL tree insert bound. No callers outside tests.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 55 | DFSStEph.rs | Removed external_body from dfs, added Ghost(gray), 4 proof lemmas, neighbor-closure invariant |
| 2 | 55 | DFSStPer.rs | Same approach for persistent variant |
| 3 | 55 | TopoSortStEph.rs | Fixed spec_is_path edge trigger: path[k] instead of spec_has_edge |
| 4 | 55 | TopoSortStPer.rs | Fixed spec_is_path_per edge trigger: path[k] instead of spec_has_edge_per |

## Verification

- `scripts/validate.sh`: 5138 verified, 0 errors
- Steps used: 4 of 20
