# Agent 1 — Round 188 Report

## Task

Fix exec-to-view bridges in HashMap UnionFind using feq().

## Results

| # | Metric | Before | After |
|---|--------|--------|-------|
| 1 | Verified | 724 | 726 |
| 2 | Errors | 7 | 5 |

## Changes

1. Replaced `*parent_val == curr` with `feq(parent_val, &curr)` in find() —
   gives `result == (parent_val@ == curr@)`, bridging exec to view equality.
2. Replaced `root_u == root_v` with `feq(&root_u, &root_v)` in union_sets().
3. Replaced `root_u == root_v` with `feq(&root_u, &root_v)` in equals().
4. Added `obeys_feq_full::<V>()` to wf (required by feq()).
5. Changed import to `use crate::vstdplus::feq::feq::*` for full access.
6. Added `spec_is_root` to trait and find() ensures.
7. Recovered R187 work from git reflog after rebase.

## Errors fixed (2)

The exec-to-view bridge errors are fixed. `feq()` ensures `result == (a@ == b@)`,
connecting exec PartialEq to spec view equality. This resolves the root
detection in find() and the root comparison in union_sets()/equals().

## Remaining 5 errors

| # | Line | Error | Root cause |
|---|------|-------|------------|
| 1 | 96 | lemma_find_is_root postcondition | Fuel=1 gap: find(v,1) = pv(v), which might not be a root |
| 2 | 209 | find() spec_is_root postcondition | Cascades from error 1 |
| 3 | 154 | lemma_find_after_link postcondition | Fuel=2 gap: trichotomy fails for non-root at depth 2 |
| 4 | 214 | union_sets() same_set postcondition | Cascades from error 3 |
| 5 | 387 | rank_u + 1 overflow | rank_bounded gap (needs counting machinery) |

Errors 1-4 are the fuel-gap issue: fuel-based spec_pure_find can't guarantee
root-ness or trichotomy for small fuel values, even though valid forests always
have sufficient fuel. The array version solved this with rank-based
decreases_when termination. The Map version needs the same — but that requires
the rank domain bridge (the original R185-R187 problem).

The path forward: solve the rank domain bridge for the decreases_when guard,
replacing the fuel parameter. This eliminates the fuel gaps entirely.
