# Agent 2 Round 61 Report

## Summary

Closed 4 of 6 capacity `assume` holes across Chap53 PQMin files by proving
`visited@.subset_of(vertex_universe)` invariant through the exploration loop. Used
`clone_plus` (from `vstdplus::clone_plus`) to get `cloned` postconditions that standard
`Clone::clone` lacks, then leveraged the feq broadcast axiom chain to establish view
equality across clone boundaries.

## Verification

| Metric | Before | After |
|--------|--------|-------|
| Verified | 4496 | 4498 |
| Errors | 0 | 0 |
| Project holes | 12 | 8 |
| RTT | 2610 | 2610 |
| PTT | 147 | 147 |

## Holes Before/After by File

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 37 | AVLTreeSeqStEph.rs | 0 | 0 | 0 |
| 2 | 53 | PQMinStEph.rs | 4 | 1 | -3 |
| 3 | 53 | PQMinStPer.rs | 2 | 1 | -1 |
| **Total** | | | **6** | **2** | **-4** |

## What Was Done

### Prior work (already committed)

- `lemma_wf_implies_len_bound_steph` in AVLTreeSeqStEph.rs — infrastructure for capacity proofs.
- PQMinStEph priorities and initial_frontier loop capacity — closed via wf lemma + len_union.

### This session — visited capacity proofs

- **PQMinStEph visited capacity** (was line 198): Proved via `visited@.subset_of(vertex_universe)`
  + `vstd::set_lib::lemma_len_subset`. Added vertex_universe ghost parameter to `pq_explore`
  and `pq_min_multi` with constraints: `vertex_universe.finite()`,
  `vertex_universe.len() + 1 < usize::MAX`, `visited_init@.subset_of(vertex_universe)`,
  `graph.ensures(...) ==> result@.subset_of(vertex_universe)`, frontier entry vertex invariant.
- **PQMinStPer visited capacity** (was line 183): Same approach mirrored for persistent API.
- **PQMinStEph initial_frontier capacity** and **priorities capacity**: Already closed.

### Key insight: clone_plus for view tracking

Standard `Clone::clone` in vstd has **no ensures for generic types**. After `let v = x.clone()`,
Verus knows nothing about `v@`. Used `clone_plus()` (from `crate::vstdplus::clone_plus`)
which ensures `cloned(*self, result)`. Combined with the feq axiom chain:

1. `assert(obeys_feq_full_trigger::<V>())` triggers `axiom_obeys_feq_full` (admitted broadcast)
2. Gives `obeys_feq_full::<V>()` including `obeys_feq_clone::<V>()`
3. `cloned(source, result)` + `obeys_feq_clone` fires `axiom_cloned_implies_eq_owned`
4. Gives `source == result`, hence `source@ == result@`

Named clone variables (`v_clone1`, `v_clone2`, `v_for_visited`, `neighbor_clone1`,
`neighbor_clone2`) to give Verus handles for view tracking through Pair construction.

## Remaining Holes

| # | Chap | File | Line | Description |
|---|------|------|------|-------------|
| 1 | 53 | PQMinStEph.rs | 269 | frontier_updated capacity |
| 2 | 53 | PQMinStPer.rs | 255 | frontier_updated capacity |

Both require proving frontier length stays bounded. This needs an injection argument
(each frontier entry uniquely determines a vertex via `priority_fn`), which requires
`priority_fn` view-determinism that the generic `Fn` interface does not guarantee.

## Files Modified

- `src/Chap53/PQMinStEph.rs` — vertex_universe constraints, subset proof, clone_plus
- `src/Chap53/PQMinStPer.rs` — mirror of StEph changes for persistent API
