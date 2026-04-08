# Agent 2 Round 163 Report — Compress OrdKeyMap proof functions

## Summary

Compressed `src/Chap41/OrdKeyMap.rs` by extracting 3 shared proof lemmas and
replacing repeated inline proof patterns across 11 functions. Net reduction:
**206 lines** (5105 → 4899).

## Lemmas created

| # | Chap | Lemma | Requires/Ensures Summary | Callers | Lines Saved |
|---|------|-------|--------------------------|---------|-------------|
| 1 | 41 | `lemma_view_gen_empty` | Empty set is View-generated | 6 sites | ~24 |
| 2 | 41 | `lemma_freshness_from_sorted` | Key at index i not yet in accumulated domain | 6 sites | ~36 |
| 3 | 41 | `lemma_map_dom_preserved_by_superset` | sub ⊆ sup implies dom(sub) ⊆ dom(sup) | 8 sites | ~56 |

Total lemma overhead: ~45 lines (3 lemmas with signatures, requires, ensures, bodies).

## Compression techniques

1. **ordkeymap_find inline→lemma**: Replaced ~50 lines of inline
   `spec_key_unique_pairs_set` and `spec_set_pair_view_generated` proofs
   for BST subtrees with calls to existing `lemma_key_unique_subset` and
   `lemma_view_gen_subset` (which every other BST-descent function already used).

2. **Empty view-gen**: 6 sites had identical 5-line `assert(false)` blocks
   proving `spec_set_pair_view_generated(Set::empty())`. Replaced with
   `lemma_view_gen_empty::<K, V>()`.

3. **Freshness from sorted**: 6 loop bodies had identical 8-line
   proof-by-contradiction blocks proving the next sorted key is not in
   the accumulated domain. Replaced with `lemma_freshness_from_sorted(...)`.

4. **Domain monotonicity**: 8 loop-body sites had 5-7 line blocks proving
   old domain keys survive BST insert (choose + map_contains pattern).
   Replaced with `lemma_map_dom_preserved_by_superset(old, new)`.

## Per-function proof line changes

| # | Chap | Function | Before | After | Delta |
|---|------|----------|--------|-------|-------|
| 1 | 41 | ordkeymap_select | 281 | 281 | 0 |
| 2 | 41 | union_with | 237 | 198 | −39 |
| 3 | 41 | ordkeymap_prev | 205 | 205 | 0 |
| 4 | 41 | ordkeymap_next | 201 | 201 | 0 |
| 5 | 41 | ordkeymap_rank | 195 | 195 | 0 |
| 6 | 41 | union | 183 | 148 | −35 |
| 7 | 41 | intersect_with | 166 | 131 | −35 |
| 8 | 41 | split_rank_key | 124 | 121 | −3 |
| 9 | 41 | intersect | 119 | 96 | −23 |
| 10 | 41 | ordkeymap_split | 116 | 116 | 0 |
| 11 | 41 | tabulate | 114 | 100 | −14 |
| — | 41 | ordkeymap_find | 88 | 46 | −42 |
| — | 41 | difference | 72 | 65 | −7 |
| — | 41 | map_values | 62 | 44 | −18 |
| — | 41 | restrict | 65 | 58 | −7 |
| — | 41 | subtract | 65 | 58 | −7 |

Note: ordkeymap_find was not in the original >100-line list but had the
largest single compression opportunity (inline proofs → existing lemma calls).

## Total

- Lines before: 5105
- Lines after: 4899
- Net removed: **206 lines**
- 3 new lemmas added (45 lines)
- Gross proof lines removed: ~251

## Validation

`scripts/validate.sh isolate Chap41`: 2282 verified, 0 errors in Chap41
(pre-existing flaky conjunction failure in BSTSplayMtEph.rs Chap37, unchanged).

## Patterns found but not extracted

- **ordkeymap_next/prev symmetry**: These mirror functions have structurally
  similar proofs but differ in direction (Less↔Greater, min↔max). A shared
  lemma would need direction parameterization, adding complexity without
  clear savings.

- **rank/select three-way domain decomposition**: The Greater case in rank
  and the i > left_size case in select prove similar `left ∪ {root} ∪ right.filter`
  decompositions, but the rank predicate is parameterized differently in each.
  Extracting this would require a higher-order lemma taking the predicate
  as a parameter, which Verus handles awkwardly.

- **Post-loop domain equality proofs**: Each set operation's post-loop proof
  (forward + backward direction of `dom =~= expected`) follows the same
  structure but with operation-specific conditions (intersection vs difference
  vs union). Not extractable without losing clarity.

- **WF axiom re-assertions**: 7 sites end with 4-line blocks asserting
  `spec_pair_key_determines_order`, `obeys_cmp_spec`, `view_ord_consistent`,
  `obeys_feq_fulls`. These are Z3 trigger hints, not derivable from a lemma
  call — Z3 needs them in-scope explicitly.
