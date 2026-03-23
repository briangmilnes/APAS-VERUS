# Agent 2 Round 59 Report

## Assignment

Close 12 capacity assumes in Chap53 graph search and priority queue files.
All 12 follow the pattern `assume(x.len() + y < usize::MAX as nat)` before `.union()` calls.

## Results Summary

| # | Chap | File | Holes Before | Holes After | Closed |
|---|------|------|-------------|-------------|--------|
| 1 | 53 | GraphSearchStEph.rs | 2 | 0 | 2 |
| 2 | 53 | GraphSearchStPer.rs | 2 | 0 | 2 |
| 3 | 53 | PQMinStEph.rs | 4 | 4 | 0 |
| 4 | 53 | PQMinStPer.rs | 4 | 2 | 2 |
| **Total** | | | **12** | **6** | **6** |

## Verification

- `scripts/validate.sh`: 4484 verified, 1 error (pre-existing Chap43 issue)
- `scripts/rtt.sh`: 2610 passed, 0 skipped
- `scripts/ptt.sh`: 147 passed, 0 skipped

## Techniques

### GraphSearch files (4 assumes closed, all 4)

Added `Ghost(vertex_universe: Set<<V as View>::V>)` parameter with
`requires vertex_universe.len() + 1 < usize::MAX as nat`. Maintained
`visited@.subset_of(vertex_universe)` and `frontier@.subset_of(vertex_universe)` as loop
invariants. Used `vstd::set_lib::lemma_len_subset` to prove capacity from subset + VU bound.
Applied to both `graph_search` and `graph_search_multi` entry points. Test files updated
with `Ghost::assume_new()`.

### PQMinStPer (2 assumes closed out of 4)

Used `lemma_wf_implies_len_bound_stper` (broadcast proof fn from Chap37 AVLTreeSeqStPer)
to prove `s@.len() < usize::MAX` from `s.spec_avltreeseqstper_wf()`. Combined with loop
counters and `vstd::set_lib::lemma_len_union` to maintain `set@.len() <= counter` invariants.

- **priorities loop** (pq_explore): Bound `Pair(vref.clone(), p)` to local variable,
  tracked `priorities@.len() <= j` with counter, proved capacity via lemma.
- **initial_frontier loop** (pq_min_multi): Same pattern with `entry` variable.

Two assumes remain in pq_explore: `visited@.len() + 1` and `frontier_updated@.len() + 1`.
These involve sets built from graph neighbor queries where subset relationships with the
vertex universe are hard to establish through the `Pair<Pair<P,V>,V>` frontier type
(Z3 cannot reason about nested tuple `.1` accessors on View types).

### PQMinStEph (0 assumes closed)

StEph lacks the `lemma_wf_implies_len_bound` broadcast lemma that StPer has. The
`AVLTreeSeqStEph` module has a private `lemma_size_eq_inorder_len` but no public proof
that wf implies `len < usize::MAX`. Additionally, `to_seq()` does not ensure
`no_duplicates()`, blocking the subset-based approach. Ghost parameter added for API
consistency but all 4 assumes retained.

## Remaining Holes in Chap53

| # | Chap | File | Line | Assume |
|---|------|------|------|--------|
| 1 | 53 | PQMinStEph.rs | 190 | `visited@.len() + 1 < usize::MAX` |
| 2 | 53 | PQMinStEph.rs | 212 | `frontier_updated@.len() + 1 < usize::MAX` |
| 3 | 53 | PQMinStEph.rs | 240 | `priorities@.len() + 1 < usize::MAX` |
| 4 | 53 | PQMinStEph.rs | 289 | `initial_frontier@.len() + 1 < usize::MAX` |
| 5 | 53 | PQMinStPer.rs | 183 | `visited@.len() + 1 < usize::MAX` |
| 6 | 53 | PQMinStPer.rs | 204 | `frontier_updated@.len() + 1 < usize::MAX` |

## What Would Unblock Further Progress

1. **PQMinStEph (4 assumes)**: A public `lemma_wf_implies_len_bound_steph` in
   `src/Chap37/AVLTreeSeqStEph.rs` (mirroring the StPer version) would enable closing
   the priorities and initial_frontier assumes. The visited and frontier_updated assumes
   additionally need a way to relate frontier entries to the vertex universe through the
   nested `Pair<Pair<P,V>,V>` type.

2. **PQMinStPer visited+frontier_updated (2 assumes)**: Need Z3 to reason that vertices
   extracted from frontier entries belong to the vertex universe. The triple nesting
   (`Pair<Pair<P,V>,V>` with View `((P::V, V::V), V::V)`) defeats Z3's ability to
   connect `.1` tuple projections to set membership.

## Files Modified

- `src/Chap53/GraphSearchStEph.rs` — Ghost param, subset invariants, 2 assumes removed
- `src/Chap53/GraphSearchStPer.rs` — Ghost param, subset invariants, 2 assumes removed
- `src/Chap53/PQMinStEph.rs` — Ghost param added, frontier vertex invariant removed (was attempted, failed), 4 assumes retained
- `src/Chap53/PQMinStPer.rs` — Ghost param, loop counters + lemma for 2 assumes, 2 assumes retained
- `tests/Chap53/TestGraphSearchStEph.rs` — Ghost::assume_new() added
- `tests/Chap53/TestGraphSearchStPer.rs` — Ghost::assume_new() added
- `tests/Chap53/TestPQMinStEph.rs` — Ghost::assume_new() added
- `tests/Chap53/TestPQMinStPer.rs` — Ghost::assume_new() added
