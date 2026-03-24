# Agent 3 — Round 69 Report

## Assignment

Eliminate 6 constructor axiom assumes in `tabulate` in `src/Chap43/OrderedTableStEph.rs`.

## Result: 6 assumes → 0

All 6 tabulate axiom assumes removed by lifting to `requires` clauses across
the call chain.

## Technique

The 6 assumes were type-level axiom predicates needed for `Pair<K, V>` ordering
and key uniqueness:

```
vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>()
view_ord_consistent::<Pair<K, V>>()
spec_pair_key_determines_order::<K, V>()
vstd::laws_cmp::obeys_cmp_spec::<K>()
view_ord_consistent::<K>()
obeys_feq_fulls::<K, V>()
```

These couldn't initially go in the trait requires because the trait had `V: StT`
while the predicates need `V: Ord` (since `Pair<K, V>: Ord` requires `V: Ord`).
The struct `OrderedTableStEph<K, V>` already required `V: Ord` and there is only
one impl, so changing the trait bound from `V: StT` to `V: StT + Ord` was safe.

With the bound change, all 6 predicates were added to `tabulate` requires in
the trait, and the 6 assumes deleted from the impl body. The 3 caller traits
(AugOrderedTableStEph, OrderedTableMtEph, AugOrderedTableMtEph) were updated
with matching requires so the axioms propagate through the call chain.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedTableStEph.rs | Trait `V: StT` → `V: StT + Ord`; +6 requires on `tabulate`; -6 assumes |
| 2 | 43 | AugOrderedTableStEph.rs | +import `view_ord_consistent`; +6 requires on `tabulate` |
| 3 | 43 | OrderedTableMtEph.rs | +6 requires on `tabulate` |
| 4 | 43 | AugOrderedTableMtEph.rs | +imports `view_ord_consistent`, `spec_pair_key_determines_order`; +6 requires on `tabulate` |

## Holes Before/After — OrderedTableStEph.rs

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStEph.rs | 9 | 3 | -6 |

## Remaining Holes (3)

| # | Chap | File | Type | Location | Blocker |
|---|------|------|------|----------|---------|
| 1 | 43 | OrderedTableStEph.rs | external_body | rank_key_iter | Agent 3 owns, merge-conflict placeholder |
| 2 | 43 | OrderedTableStEph.rs | external_body | select_key | Agent 3 owns, depends on rank_key |
| 3 | 43 | OrderedTableStEph.rs | assume | iter next() | iter_invariant pattern |

## Remaining Warnings (1)

`fn_missing_wf_ensures` on `from_sorted_entries` — needs `spec_key_unique_pairs_set`
precondition and 4 additional axiom requires to prove `spec_orderedtablesteph_wf()`.
Deferred: changes `OrderedTableStEphLit!` macro contract.

## Verification

- validate: 4435 verified, 0 errors
- RTT: 2528 passed, 0 skipped
- PTT: 145 passed, 0 skipped
