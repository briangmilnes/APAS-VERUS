# Agent 2 — Round 76b Report

## Objective

1. Fold `obeys_feq_clone` into `spec_bstsetsplaymteph_wf()` to eliminate 8 assume holes.
2. Fix 2 `fn_missing_requires` warnings in `BSTSplayMtEph.rs`.

## Results

- **Verified**: 4830 (unchanged)
- **RTT**: 2619 passed
- **PTT**: 157 passed
- **Errors/warnings**: 0
- **Global holes**: 73 → 67 (-6)

## Hole Changes — BSTSetSplayMtEph.rs

| Metric | Before | After |
|--------|--------|-------|
| assume (obeys_feq_clone) | 8 | 2 |
| external_body | 5 | 5 |
| accept (clone) | 1 | 1 |
| Total holes | 13 | 7 |

Net: -6 holes (removed 8 assumes from function bodies, added 2 in empty/singleton).

The 2 remaining assumes are in `empty()` and `singleton()` — constructors that can't
derive `obeys_feq_clone` from a requires clause because they have no input set.

## Changes Made

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 37 | BSTSetSplayMtEph.rs | wf spec now includes `obeys_feq_clone::<T>()` |
| 2 | 37 | BSTSetSplayMtEph.rs | `values_vec`: added `obeys_feq_clone` to requires, removed assume |
| 3 | 37 | BSTSetSplayMtEph.rs | `build_from_vec`: added `requires obeys_feq_clone::<T>()` |
| 4 | 37 | BSTSetSplayMtEph.rs | `empty()`: added `assume(obeys_feq_clone)` |
| 5 | 37 | BSTSetSplayMtEph.rs | `singleton()`: added `assume(obeys_feq_clone)` |
| 6 | 37 | BSTSetSplayMtEph.rs | Removed assumes from: delete, split, join_pair, join_m, filter, reduce, iter_in_order, values_vec |
| 7 | 37 | BSTSplayMtEph.rs | `size_link`: added `// veracity: no_requires` |
| 8 | 37 | BSTSplayMtEph.rs | `update`: added `// veracity: no_requires` |

## Global Status

- 46 chapters, 42 clean, 4 holed, 67 holes, 244 modules
