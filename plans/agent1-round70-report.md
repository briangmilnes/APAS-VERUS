# Agent 1 — Round 70 Report

## Summary

Eliminated 6 proof holes in `src/Chap43/OrderedTableStEph.rs` by lifting
constructor axiom assumes to trait requires. Fixed the root cause: the trait
had `V: StT` while all other OrderedTable traits had `V: StT + Ord`.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStEph.rs | 7 | 1 | −6 |

## Technique

The `tabulate` impl had 6 `assume(...)` calls for type-level axiom predicates:
- `obeys_cmp_spec::<Pair<K,V>>()`, `view_ord_consistent::<Pair<K,V>>()`
- `spec_pair_key_determines_order::<K,V>()`, `obeys_cmp_spec::<K>()`
- `view_ord_consistent::<K>()`, `obeys_feq_fulls::<K,V>()`

These couldn't be derived from the trait because the trait had `V: StT` (no `Ord`),
while the axiom functions require `Pair<K,V>: Ord`, which needs `V: Ord`.

**Root cause fix**: Changed `OrderedTableStEphTrait<K: StT + Ord, V: StT>` to
`OrderedTableStEphTrait<K: StT + Ord, V: StT + Ord>`, matching all other
OrderedTable traits (StPer, MtEph, MtPer) which already had `V: ... + Ord`.
The struct `OrderedTableStEph` itself already required `V: StT + Ord`.

With `V: Ord` on the trait, the 6 axiom predicates could be added to the trait's
`tabulate` requires and removed as assumes from the impl body.

**Callers updated** (same axiom requires propagated):
- `OrderedTableMtEph::tabulate` (trait, line 157)
- `AugOrderedTableStEph::tabulate` (trait, line 190)
- `AugOrderedTableMtEph::tabulate` (trait, line 191)

New imports added with `#[cfg(verus_keep_ghost)]` to avoid non-Verus build failures:
- `view_ord_consistent` from BSTParaStEph
- `spec_pair_key_determines_order` from OrderedTableStEph

## Remaining Holes (1)

| # | Chap | File | Line | Type | Reason |
|---|------|------|------|------|--------|
| 1 | 43 | OrderedTableStEph.rs | 3843 | assume | `assume(iter_invariant(self))` in `Iterator::next` — irreducible (Rust's Iterator trait can't have requires) |

## Remaining Warning (1)

| # | Chap | File | Line | Type | Reason |
|---|------|------|------|------|--------|
| 1 | 43 | OrderedTableStEph.rs | 3867 | fn_missing_wf_ensures | `from_sorted_entries` missing `result.spec_orderedtablesteph_wf()` — requires adding `spec_key_unique_pairs_set(entries@.to_set())` precondition, tracking key uniqueness through loop invariant, and cascading requires to MtEph caller |

## Verification

- 4437 verified, 0 errors
- 2528 RTT passed
- 145 PTT passed
