# Agent 1 Round 69 Report

## Summary

Proved `rank_key_iter` and `select_key` in `OrderedTableStEph.rs` by adapting
the ghost proof technique from `OrderedTableStPer.rs` (Agent 3 R68). Both
`external_body` marks removed. Project-wide holes: 34 to 32.

## Verification

- 4437 verified, 0 errors
- 2528 RTT passed, 0 skipped
- 145 PTT passed, 0 skipped

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStEph.rs | 9 | 7 | -2 |

**Project total: 34 to 32 holes (-2)**

Per-chapter breakdown: Chap39: 1, Chap43: 31.

## Techniques Used

### rank_key_iter
Ghost `counted_keys: Set<K::V>` tracks keys counted as strictly less than `k`.
Three-way comparison proof:
- `Less`: insert key into counted_keys, prove filter_pred via witness
- `Equal`: prove `!filter_pred` via view equality contradiction (`t@ == pair.0@ == k@` contradicts `t@ != k@`)
- `Greater`: prove `!filter_pred` via `TotalOrder::antisymmetric` contradiction
Post-loop: set extensionality connects `counted_keys` to `self@.dom().filter(pred)`.

### select_key
Loop invariant on `result_key matches Some(rk)` with conditional rank ensures.
Uses `clone_plus()` with `lemma_cloned_view_eq` bridge for candidate key.
Calls `rank_key` per candidate; postconditions are vacuously true when `result_key` is `None`.

## Remaining Holes in OrderedTableStEph.rs (7)

| # | Chap | File | Line | Hole | What Blocks It |
|---|------|------|------|------|----------------|
| 1 | 43 | OrderedTableStEph.rs | 1326 | assume(obeys_cmp_spec) | Trait `V: StT` lacks `Ord` bound; can't derive from trait requires |
| 2 | 43 | OrderedTableStEph.rs | 1327 | assume(view_ord_consistent) | Same trait bound limitation as above |
| 3 | 43 | OrderedTableStEph.rs | 1453 | assume(spec_pair_key_determines_order) | tabulate wf proof; trait bound gap |
| 4 | 43 | OrderedTableStEph.rs | 1454 | assume(obeys_cmp_spec::\<K\>) | tabulate wf proof; trait bound gap |
| 5 | 43 | OrderedTableStEph.rs | 1455 | assume(view_ord_consistent::\<K\>) | tabulate wf proof; trait bound gap |
| 6 | 43 | OrderedTableStEph.rs | 1456 | assume(obeys_feq_fulls) | tabulate wf proof; trait bound gap |
| 7 | 43 | OrderedTableStEph.rs | 3842 | assume(iter_invariant(self)) | Iterator::next can't have requires |

Holes 1-6 are all in `tabulate` and stem from the trait signature declaring `V: StT`
without `Ord`, so the impl can't prove `obeys_cmp_spec`/`view_ord_consistent` for
`Pair<K, V>` from the trait's requires alone. Hole 7 is the standard iterator limitation.

## Also Attempted

- `from_sorted_entries` wf ensures (`fn_missing_wf_ensures` warning): needs
  `obeys_feq_fulls::<K, V>()` added to requires and `spec_key_unique_pairs_set`
  maintained in loop invariant. Deferred — more invasive than a one-line ensures addition.

## Commit

`d8a457ccf` on `agent1/ready`
