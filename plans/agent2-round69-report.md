# Agent 2 — Round 69 Report

## Goal

Eliminate constructor axiom assumes in `src/Chap43/OrderedTableStPer.rs`.

## Results

**OrderedTableStPer.rs holes: 21 → 1 (-20)**

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 43 | OrderedTableStPer.rs | 21 | 1 | -20 |
| 2 | 43 | AugOrderedTableStPer.rs | 0 | 0 | 0 |

## Technique

Lifted 7 type-level axiom predicates from `assume()` in constructor bodies to
`requires` in the trait signatures. The axiom set matches what
`spec_orderedtablestper_wf()` includes:

1. `obeys_feq_fulls::<K, V>()`
2. `obeys_feq_full::<Pair<K, V>>()`
3. `vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>()`
4. `view_ord_consistent::<Pair<K, V>>()`
5. `spec_pair_key_determines_order::<K, V>()`
6. `vstd::laws_cmp::obeys_cmp_spec::<K>()`
7. `view_ord_consistent::<K>()`

Methods that take `&self` already get these axioms from `self.spec_orderedtablestper_wf()`
in their requires. Constructors (`empty`, `singleton`, `tabulate`) have no `self`, so
callers must provide them explicitly. Callers that already have wf in scope satisfy the
new requires automatically.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedTableStPer.rs | Added axiom requires to `empty()`, `singleton()`, `tabulate()` trait; deleted 20 assumes from impl bodies |
| 2 | 43 | AugOrderedTableStPer.rs | Cascaded axiom requires to `empty()`, `singleton()`, `tabulate()` trait (calls OrderedTableStPer constructors); added cfg-gated `view_ord_consistent` import |
| 3 | 43 | ProveOrderedTableStPer.rs (PTT) | Added axiom requires to both iterator proof test functions |

## Remaining Hole

- 1 `assume(iter_invariant(self))` in iterator `next()` — standard iterator pattern, stays.

## Warnings (unchanged)

- 1 `fn_missing_wf_ensures` on `from_sorted_entries` — needs `spec_key_unique_pairs_set` requires or proof to ensure wf, not attempted this round.
- 2 `assume_eq_clone_workaround` on `eq()` and `clone()` — standard pattern.

## Validation

- validate.sh: 4435 verified, 0 errors
- rtt.sh: 2528 passed, 0 skipped
- ptt.sh: 145 passed, 0 skipped
