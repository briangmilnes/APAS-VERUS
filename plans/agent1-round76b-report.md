# Agent 1 — Round 76b Report

## Objective

1. Fold `obeys_feq_clone` into `spec_bstsetrbmteph_wf()` to eliminate 8 assumes.
2. Fix 3 `fn_missing_requires` warnings in BSTRBMtEph.rs.

## Results

- **BSTSetRBMtEph holes**: 13 → 7 (-6)
- **Global holes**: 73 → 67 (-6)
- **Verified**: 4830 (unchanged)
- **RTT**: 2619 passed
- **PTT**: 157 passed

## Holes Before/After

| # | Chap | File | Function | Before | After | Notes |
|---|------|------|----------|--------|-------|-------|
| 1 | 37 | BSTSetRBMtEph.rs | values_vec | assume(feq_clone) | clean | feq_clone now in requires via caller wf |
| 2 | 37 | BSTSetRBMtEph.rs | delete | assume(feq_clone) | clean | wf provides feq_clone |
| 3 | 37 | BSTSetRBMtEph.rs | split | assume(feq_clone) | clean | wf provides feq_clone |
| 4 | 37 | BSTSetRBMtEph.rs | join_pair | assume(feq_clone) | clean | wf provides feq_clone |
| 5 | 37 | BSTSetRBMtEph.rs | join_m | assume(feq_clone) | clean | wf provides feq_clone |
| 6 | 37 | BSTSetRBMtEph.rs | filter | assume(feq_clone) | clean | wf provides feq_clone (external_body remains) |
| 7 | 37 | BSTSetRBMtEph.rs | reduce | assume(feq_clone) | clean | wf provides feq_clone (external_body remains) |
| 8 | 37 | BSTSetRBMtEph.rs | iter_in_order | assume(feq_clone) | clean | wf provides feq_clone |
| 9 | 37 | BSTSetRBMtEph.rs | empty | clean | assume(feq_clone) | New: bootstraps wf |
| 10 | 37 | BSTSetRBMtEph.rs | singleton | clean | assume(feq_clone) | New: bootstraps wf |
| 11 | 37 | BSTRBMtEph.rs | is_red | fn_missing_requires | no_requires | Works on any Link<T> |
| 12 | 37 | BSTRBMtEph.rs | size_link | fn_missing_requires | no_requires | Works on any Link<T> |
| 13 | 37 | BSTRBMtEph.rs | update | fn_missing_requires | no_requires | Works on any &mut Node<T> |

## Remaining 7 holes in BSTSetRBMtEph

- 2 × `assume(obeys_feq_clone)` in empty/singleton (bootstrap wf)
- 3 × `external_body` on union/intersection/difference (ParaPair recursive closures)
- 2 × `external_body` on filter/reduce (FnMut closure requires)
- 1 × `accept()` in Iterator::next (clone preserves value, pre-existing)
