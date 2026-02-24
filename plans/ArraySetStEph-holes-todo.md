# ArraySetStEph Proof Holes — Todo and Status

## Summary

- **Before:** 21 assumes
- **After:** 14 assumes
- **Removed:** 7 (finiteness via `seq_to_set_is_finite`, delete/insert via lemmas)

## Completed

| # | Task | Status |
|---|------|--------|
| 1 | find: add `self@.finite()` requires | Done |
| 2 | delete: use `lemma_filter_remove`, prove set equality | Done (2 assumes remain: seq equality, no_duplicates) |
| 3 | insert: use `lemma_push_*`, prove set equality | Done (1 assume remains: seq equality) |
| 4 | filter, intersection, difference, union: prove finiteness | Done (`seq_to_set_is_finite`) |

## Remaining Holes (14)

| Function | Assumes | Notes |
|----------|---------|-------|
| find | 2 | `elem@ == x@` when `*elem == *x`; `elements@[i] != x@` when `*elem != *x` — needs `obeys_eq_spec` |
| filter | 3 | `f.requires`, `result@.subset_of(self@)`, `result.spec_wf()` |
| intersection | 2 | `result@ == self@.intersect(other@)`, `result.spec_wf()` |
| difference | 2 | `result@ == self@.difference(other@)`, `result.spec_wf()` |
| union | 2 | `result@ == self@.union(other@)`, `result.spec_wf()` |
| delete | 2 | `elements@ =~= filter(...)`, `no_duplicates` |
| insert | 1 | `elements@ =~= push(x@)` |
| clone | 1 | `result@ == self@` (Verus workaround) |

## Trait Requires Added

- `find`: `self@.finite()`
- `filter`: `self@.finite()`
- `intersection`, `difference`, `union`: `self@.finite()`, `other@.finite()`
- `delete`, `insert`: `old(self)@.finite()`

## Future Work

1. **find**: Add `requires vstd::laws_eq::obeys_eq_spec::<T>()` and propagate to callers to remove 2 assumes.
2. **filter**: Loop invariant for `result_vec` to prove subset and `no_duplicates`.
3. **intersection/difference/union**: Loop invariants for set equality.
4. **delete**: Loop invariant for filtered seq equality; prove `no_duplicates` from subsequence.
5. **insert**: Prove `elements@ =~= push(x@)` via loop invariant.
6. **clone**: Use `lemma_seq_map_cloned_view_eq` with `obeys_feq_clone` (depends on ArraySeqStEph clone spec).
