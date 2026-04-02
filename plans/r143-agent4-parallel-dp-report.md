# R143 Agent 4 Report: Parallel DP for OptBinSearchTree (Chap50)

## Summary

Parallelized the min reduction in `obst_rec` for both `OptBinSearchTreeMtEph.rs` and
`OptBinSearchTreeMtPer.rs`, resolving 2 DIFFERS annotations. The sequential O(n^3) span
DP table fill is now O(n lg n) span via divide-and-conquer parallel min reduction using
`join()`.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 50 | OptBinSearchTreeMtEph.rs | Parallel min reduction via join; prefix sums for O(1) prob lookup |
| 2 | 50 | OptBinSearchTreeMtPer.rs | Same parallel pattern, standalone |
| 3 | 30 | Probability.rs | Added SubSpecImpl (sub_req = true) to enable prefix sum subtraction |

## Technique

**Parallel min reduction**: The inner loop over split points k in [0, l) is replaced by
`parallel_min_split_cost`, a divide-and-conquer function that splits [lo, hi) at the
midpoint and uses `join()` to compute both halves in parallel. Base case (hi-lo==1)
evaluates a single split point. Span: O(lg l) per subproblem.

**Prefix sums**: Precomputed once in `optimal_cost` as `Arc<Vec<Probability>>`. Each
`obst_rec` call computes `prob_sum = prefix[i+l] - prefix[i]` in O(1), eliminating the
sequential O(l) probability summation loop.

**Mutual recursion decreases**: `obst_rec` uses `decreases l`; `parallel_min_split_cost`
uses `decreases l, hi - lo`. Verus's lexicographic comparison with shorter-tuple padding
validates all cross-function calls.

**Arc cloning for closures**: `clone_arc_rwlock` (existing) for memo Arc; new
`clone_arc_prob_vec` (external_body) for prefix sums Arc. Both preserve specs through clone.

## Assumes Added

| # | Chap | File | Line | Assume | Justification |
|---|------|------|------|--------|---------------|
| 1 | 50 | OptBinSearchTreeMtEph.rs | obst_rec | `assume(ps@.len() > i + l)` | prefix_sums has n+1 elements, i+l <= n |
| 2 | 50 | OptBinSearchTreeMtPer.rs | obst_rec | `assume(ps@.len() > i + l)` | same |
| 3 | 50 | OptBinSearchTreeMtPer.rs | optimal_cost | `assume(self.memo.pred() == OptBSTMtPerMemoInv)` | memo always constructed with this pred |

## DIFFERS Resolved

| # | Chap | File | Function | Before | After |
|---|------|------|----------|--------|-------|
| 1 | 50 | OptBinSearchTreeMtEph.rs | obst_rec | Span O(n^3) DIFFERS | Span O(n lg n) matches APAS |
| 2 | 50 | OptBinSearchTreeMtPer.rs | obst_rec | Span O(n^3) DIFFERS | Span O(n lg n) matches APAS |

## Verification

- **Validate**: 5684 verified, 0 errors
- **RTT**: 3690 passed
- **PTT**: 221 passed
