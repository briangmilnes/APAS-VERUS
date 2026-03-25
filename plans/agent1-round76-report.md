# Agent 1 — Round 76 Report

## Objective

Rewrite BSTSetRBMtEph.rs to remove `std::collections::BTreeSet` dependency, following
the AVL pattern from Agent 5's R75 work.

## Results

- **Holes**: 16 → 13 (-3 net; 11 external_body/external eliminated, 8 assume(obeys_feq_clone) added)
- **Verified**: 4794 → 4814 (+20)
- **RTT**: 2619 passed
- **PTT**: 157 passed
- **Global holes**: 76 → 73

## Holes Before/After

| # | Chap | File | Function | Before | After | Notes |
|---|------|------|----------|--------|-------|-------|
| 1 | 37 | BSTSetRBMtEph.rs | values_vec | external_body | assume(feq_clone) | Verified loop body |
| 2 | 37 | BSTSetRBMtEph.rs | copy_set | external_body | clean | Uses build_from_vec |
| 3 | 37 | BSTSetRBMtEph.rs | from_sorted_iter | external_body | eliminated | Replaced by rebuild_from_vec + build_from_vec |
| 4 | 37 | BSTSetRBMtEph.rs | delete | external_body | assume(feq_clone) | Verified loop body |
| 5 | 37 | BSTSetRBMtEph.rs | union | external_body | external_body | ParaPair recursive — same as AVL |
| 6 | 37 | BSTSetRBMtEph.rs | intersection | external_body | external_body | ParaPair recursive — same as AVL |
| 7 | 37 | BSTSetRBMtEph.rs | difference | external_body | external_body | ParaPair recursive — same as AVL |
| 8 | 37 | BSTSetRBMtEph.rs | split | external_body | assume(feq_clone) | Verified loop body |
| 9 | 37 | BSTSetRBMtEph.rs | join_pair | external_body | assume(feq_clone) | Sequential insert loops |
| 10 | 37 | BSTSetRBMtEph.rs | join_m | external_body | assume(feq_clone) | Sequential insert loops |
| 11 | 37 | BSTSetRBMtEph.rs | filter | external_body | external_body + assume(feq_clone) | FnMut — same as AVL |
| 12 | 37 | BSTSetRBMtEph.rs | reduce | external_body | external_body + assume(feq_clone) | FnMut — same as AVL |
| 13 | 37 | BSTSetRBMtEph.rs | iter_in_order | external_body | assume(feq_clone) | Direct delegation |
| 14 | 37 | BSTSetRBMtEph.rs | iter | external_body | clean | Uses values_vec |
| 15 | 37 | BSTSetRBMtEph.rs | IntoIterator &ref | external | clean | Verified with requires/ensures |
| 16 | 37 | BSTSetRBMtEph.rs | IntoIterator owned | external | clean | Verified with requires/ensures |

## Remaining 13 holes

- 8 × `assume(obeys_feq_clone)` — established workaround pattern, same as AVL
- 3 × `external_body` on union/intersection/difference — ParaPair recursive closures
- 2 × `external_body` on filter/reduce — FnMut closure requires not provable in Verus
- 1 × `accept()` in Iterator::next — Clone preserves value (pre-existing)

## Technique

Exact replication of Agent 5's R75 BSTSetAVLMtEph pattern: removed BTreeSet, replaced
`from_sorted_iter` with `rebuild_from_vec`/`build_from_vec`, wrote verified while-loops
with explicit `nth()` + `clone()`, removed ParaPair from join_pair/join_m.
