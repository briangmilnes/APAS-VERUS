# R166 Agent 6 Report: Lift repeated proof patterns in Chap39 + Chap47 + Chap55

## Summary

Extracted duplicated proof patterns across 7 files in two chapters, reducing net
line count by 279 lines while maintaining full verification (5578 verified, 0 errors),
RTT (3776 passed), and PTT (221 passed).

## Changes

### Chap39: BST Treap — 3 new shared lemmas (−174 / +62 = −112 net)

Added 3 proof lemmas to `BSTTreapSpecsAndLemmas.rs`:

| # | Chap | Lemma | Purpose | Instances replaced |
|---|------|-------|---------|--------------------|
| 1 | 39 | `lemma_split_result_subset` | After split, both result halves are subsets of original | 7 (3 St + 4 Mt) |
| 2 | 39 | `lemma_union_part_subset` | Both halves of a union are subsets of the whole | — (used internally) |
| 3 | 39 | `lemma_halves_cross_ordered` | Two sets separated by key are cross-ordered | 5 (3 St + 2 Mt) |

Files modified:
| # | Chap | File | Lines before | Lines after | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 39 | BSTTreapSpecsAndLemmas.rs | 186 | 242 | +56 |
| 2 | 39 | BSTTreapStEph.rs | 3125 | 3036 | −89 |
| 3 | 39 | BSTParaTreapMtEph.rs | 1784 | 1699 | −85 |

Functions simplified in BSTTreapStEph.rs: `split_inner_st` (Less, Greater, Equal
branches), `union_inner_st`, `intersect_inner_st`, `difference_inner_st`, `filter_inner_st`.

Functions simplified in BSTParaTreapMtEph.rs: `split_inner` (Less, Greater branches),
`union_inner`, `intersect_inner`, `difference_inner`.

### Chap47: Flat Hash Tables — shared empties infrastructure (−249 / +91 = −158 net)

Moved 5 duplicated functions from 3 flat hash table files into `FlatHashTable.rs`:

| # | Chap | Function | Type | Was duplicated in |
|---|------|----------|------|-------------------|
| 1 | 47 | `spec_count_empties` | spec fn | QuadProb, LinProb, DoubleHash |
| 2 | 47 | `lemma_all_empties_count` | proof fn | QuadProb, LinProb, DoubleHash |
| 3 | 47 | `lemma_empties_positive_implies_exists_empty` | proof fn | QuadProb, LinProb, DoubleHash |
| 4 | 47 | `lemma_one_slot_change_empties` | proof fn | QuadProb, LinProb, DoubleHash |
| 5 | 47 | `lemma_probe_mod_identity` | proof fn | LinProb, DoubleHash |

Files modified:
| # | Chap | File | Lines before | Lines after | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 47 | FlatHashTable.rs | 200 | 287 | +87 |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | 1167 | 1097 | −70 |
| 3 | 47 | LinProbFlatHashTableStEph.rs | 858 | 770 | −88 |
| 4 | 47 | DoubleHashFlatHashTableStEph.rs | 901 | 810 | −91 |

### Chap55: DFS — analysis only, no changes

The DFS files (CycleDetect, TopoSort, DFS, SCC) share bridge lemmas via
`DFSSpecsAndLemmas.rs` (already extracted in R165). The remaining duplication is
between Eph/Per variants using different graph types (`ArraySeqStEphS` vs
`ArraySeqStPerS`). Unifying these would require a graph abstraction trait — a
larger refactoring than within-function pattern lifting.

## Verification

| Step | Result |
|------|--------|
| validate isolate Chap39 | 1295 verified, 0 errors |
| validate isolate Chap47 | 1238 verified, 0 errors |
| validate (full) | 5578 verified, 0 errors |
| RTT | 3776 passed |
| PTT | 221 passed |

## Net line change

−279 lines (−451 removed, +172 added).
