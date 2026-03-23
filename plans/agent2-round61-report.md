# Agent 2 Round 61 Report

## Summary

Closed 2 of 6 targeted capacity assumes across Chap53 PQMin files. Added
`lemma_wf_implies_len_bound_steph` to AVLTreeSeqStEph.rs (mirroring StPer version),
then used it to prove capacity bounds in PQMinStEph.rs loop bodies. The remaining 4
assumes (2 per file) in `pq_explore` involve Z3 limitations around nested Pair view
projection and clone-preserves-view chaining.

## Verification

| Metric | Before | After |
|--------|--------|-------|
| Verified | 4496 | 4498 |
| Errors | 0 | 0 |
| Project holes | 12 | 10 |
| RTT | 2610 | 2610 |
| PTT | 147 | 147 |

## Holes Before/After by File

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 37 | AVLTreeSeqStEph.rs | 0 | 0 | 0 |
| 2 | 53 | PQMinStEph.rs | 4 | 2 | -2 |
| 3 | 53 | PQMinStPer.rs | 2 | 2 | 0 |

## What Was Done

### AVLTreeSeqStEph.rs (Chap37) — infrastructure

- Made `lemma_size_eq_inorder_len` public (was private).
- Added `lemma_size_lt_usize_max` (recursive proof on Link, mirrors StPer).
- Added `lemma_wf_implies_len_bound_steph` (broadcast proof: wf implies `@.len() < usize::MAX`).
- Added `group_avltreeseqsteph_len_bound` broadcast group.

### PQMinStEph.rs (Chap53) — 4 to 2 holes

- **Closed: `priorities` loop capacity** (was line 240). Added `priorities@.len() <= j`
  loop invariant, called `lemma_wf_implies_len_bound_steph(visited_seq)` before loop,
  used `lemma_len_union` per iteration.
- **Closed: `initial_frontier` loop capacity** (was line 289). Same pattern with
  `lemma_wf_implies_len_bound_steph(sources_seq)` and `lemma_len_union`.
- **Remaining: `visited` capacity** (line 198). Assume `visited@.len() + 1 < usize::MAX`.
- **Remaining: `frontier_updated` capacity** (line 225). Assume `frontier_updated@.len() + 1 < usize::MAX`.

### PQMinStPer.rs (Chap53) — unchanged at 2 holes

- Priorities and initial_frontier loops were already closed in prior rounds.
- **Remaining: `visited` capacity** (line 183). Same Z3 blocker as StEph.
- **Remaining: `frontier_updated` capacity** (line 204). Same Z3 blocker as StEph.

## Techniques Used

- **Broadcast lemma mirroring**: Wrote StEph version of `lemma_wf_implies_len_bound` by
  following the StPer template, adapting for StEph's split `left_size + right_size` storage.
- **Loop counter invariant**: Maintained `len <= loop_counter` invariant with
  `lemma_len_union` to prove union capacity at each iteration.

## Remaining Holes — What Blocks Them

All 4 remaining Chap53 holes are in `pq_explore` (2 in StEph, 2 in StPer). The blocker
is the same for all four:

1. **visited capacity**: Proving `visited ⊆ vertex_universe` requires extracting vertex
   identity from frontier entries of type `Pair<Pair<P,V>,V>`. The Pair view maps to
   nested tuples `((P::V,V::V),V::V)`, and Z3 cannot chain clone-preserves-view (needs
   `obeys_feq_clone` trigger from feq.rs), Pair view field projection (`.1`), and
   `to_seq` membership in a single proof step.

2. **frontier_updated capacity**: Requires an injective mapping from frontier entries to
   vertices (each `Pair(Pair(p,v),v)` uniquely determines `v` since `p = priority_fn(v)`
   is deterministic). Proving this injection over the nested Pair type exceeds Z3's
   quantifier instantiation depth.

Both blockers stem from the same root: Z3 cannot reason through the combination of
clone view preservation + nested generic type projection + set membership. A possible
future fix: add a `ClonePreservesView` supertrait to `StT` (currently only in `HashOrd`),
which would let the feq axioms fire automatically.
