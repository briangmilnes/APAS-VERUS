# Agent 1 — Round 189 Report

## Task

Replace fuel-based spec_pure_find with rank-based decreases_when termination.
Fix all 5 remaining errors from R188.

## Results

| # | Metric | Before | After |
|---|--------|--------|-------|
| 1 | Verified | 726 | 731 |
| 2 | Errors | 5 | 0 |

## Changes

1. Replaced fuel-based `spec_pure_find(parent, fuel, v)` with rank-based
   `spec_pure_find(parent, rank, n, v)` using `decreases n as int - (rank[v] as int)`
   and `decreases_when(...)` guard containing all forest invariants.
2. Deleted `lemma_more_fuel` and `lemma_fuel_ge` (no longer needed without fuel).
3. Rewrote `lemma_find_in_dom` and `lemma_find_is_root` with rank-based decreases.
   No more fuel gaps — rank-based recursion always terminates at a root.
4. Rewrote `lemma_find_after_link` with rank maps (po, ro, pn, rn) instead of fuel.
   Added explicit domain facts (`pn.dom().contains(z)`, `rn.dom().contains(z)`,
   `ro.dom().contains(z)`) to requires for trigger-safe ensures evaluation.
5. Added `lemma_find_insert_unchanged` — proves find is stable through insert of
   new element (needed for size-rank invariant maintenance).
6. Added `spec_subtree` (filtered set), `spec_size_rank_inv_map` (each root's
   subtree >= rank + 1), and added to `spec_uf_wf`.
7. Added `lemma_rank_lt_n_minus_1` — two distinct roots of equal rank r implies
   r + 1 < n, using disjoint subtree counting via vstd `lemma_set_disjoint_lens`.
8. Fixed rank_u + 1 overflow in equal-rank union using lemma_rank_lt_n_minus_1.
9. Updated `find()` loop: added `rank[curr] >= steps` invariant, proving post-loop
   unreachable (rank >= n contradicts rank_bounded).
10. Updated all `union_sets()` branches: maintain spec_size_rank_inv through
    subtree extensional equality proofs (winner = disjoint union, others unchanged).
11. Added `#[verifier::rlimit(20)]` on lemma_find_after_link and
    `#[verifier::rlimit(30)]` on union_sets (Map quantifier matching is expensive).

## Errors fixed (5)

| # | R188 Error | Fix |
|---|-----------|-----|
| 1 | lemma_find_is_root postcondition (fuel=1 gap) | Rank-based decreases: no fuel gaps |
| 2 | find() spec_is_root postcondition (cascade) | Rank-based: lemma always reaches root |
| 3 | lemma_find_after_link postcondition (fuel=2 gap) | Rank-based with explicit domain requires |
| 4 | union_sets() same_set postcondition (cascade) | Fixed by error 3 fix |
| 5 | rank_u + 1 overflow | lemma_rank_lt_n_minus_1 via spec_size_rank_inv |

## Remaining errors

None. 731 verified, 0 errors.
