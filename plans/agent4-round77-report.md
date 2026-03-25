# Agent 4 — Round 77 Report

## Summary

Removed 3 proof holes (34 → 31 global). Proved BSTRBMtEph height assume by relaxing
height_rec precondition and using lemma_height_le_size. Removed both euler_tour external_body
holes in TSPApproxStEph by replacing HashSetWithViewPlus<(V,V)> with Vec<(V,V)> and adding
wf propagation to LabUnDirGraphStEph::ng.

## Holes Before/After

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 37 | BSTRBMtEph.rs | 3 | 2 | -1 | height assume removed |
| 2 | 37 | BSTSplayMtEph.rs | 5 | 5 | 0 | blocked by cached size field, recursive clone |
| 3 | 64 | SpanTreeStEph.rs | 1 | 1 | 0 | blocked by star_contract closure interface |
| 4 | 64 | TSPApproxStEph.rs | 2 | 0 | -2 | both euler_tour holes removed |

## Verification

- 4875 verified, 0 errors (baseline: 4869, +6)
- 2619 RTT passed
- 157 PTT passed
- 31 holes global (baseline: 34, -3)
- 43 clean chapters (unchanged)

## Changes

### BSTRBMtEph.rs (Chap37)
- Changed `height_rec` requires from `link_height < usize::MAX` to `<= usize::MAX`.
  Added proof assertions for child height bounds in the Some branch.
- In `height()` Mt wrapper: replaced `assume(link_height(*data) < usize::MAX)` with
  `lemma_height_le_size::<T>(*data)` which establishes `link_height <= link_spec_size <= usize::MAX`
  from the lock predicate. One assume removed.
- Second assume (`h as nat == link_height(self@)`) retained — standard ghost/real mismatch
  in Mt pattern.

### TSPApproxStEph.rs (Chap64)
- Replaced `HashSetWithViewPlus<(V,V)>` visited-edge tracking with `Vec<(V,V)>` and
  linear-scan `vec_contains_pair` helper, eliminating the unresolvable
  `obeys_key_model::<(V,V)>()` requirement.
- Added `tree_edges.spec_setsteph_wf()` to requires of `euler_tour`, `euler_tour_dfs`,
  `approx_metric_tsp` (trait and impl).
- Added `ng.spec_setsteph_wf()` to `get_neighbors` ensures.
- Removed `#[verifier::external_body]` from both `euler_tour` and `euler_tour_dfs`.

### LabUnDirGraphStEph.rs (Chap06)
- Added `ng.spec_setsteph_wf()` to trait ensures of `ng()` — consistent with
  UnDirGraphStEph and Mt graph variants which already ensure wf.
- Added `ng.spec_setsteph_wf()` to loop invariant in ng() impl.

## Remaining Holes (8 in assigned files)

| # | Chap | File | Function | Type | Blocker |
|---|------|------|----------|------|---------|
| 1 | 37 | BSTRBMtEph.rs | filter_parallel | external_body | Arc::clone on closures |
| 2 | 37 | BSTRBMtEph.rs | reduce_parallel | external_body | Arc::clone on closures |
| 3 | 37 | BSTSplayMtEph.rs | build_balanced | external_body | blocked by clone |
| 4 | 37 | BSTSplayMtEph.rs | filter_parallel | external_body | blocked by clone |
| 5 | 37 | BSTSplayMtEph.rs | reduce_parallel | external_body | blocked by clone |
| 6 | 37 | BSTSplayMtEph.rs | height (Mt) | assume | cached size ≠ recursive size |
| 7 | 37 | BSTSplayMtEph.rs | clone | external_body | recursive Node clone |
| 8 | 64 | SpanTreeStEph.rs | spanning_tree_star_contraction | external_body | star_contract interface |

## Techniques Used

1. **Precondition relaxation**: Changed `height_rec` from `< usize::MAX` to `<= usize::MAX`,
   then proved child bounds from structural decomposition.
2. **Lemma application at lock boundary**: Used existing `lemma_height_le_size` to bridge
   lock predicate (size ≤ MAX) to height requirement (height ≤ MAX).
3. **Data structure replacement**: Swapped `HashSetWithViewPlus<(V,V)>` for `Vec<(V,V)>`
   to eliminate unresolvable `obeys_key_model` tuple requirement.
4. **Wf propagation**: Added `spec_setsteph_wf()` to `ng()` ensures in LabUnDirGraphStEph,
   propagated through `get_neighbors` to `euler_tour_dfs`.
