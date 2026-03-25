# Agent 1 — Round 74 Report

## Objective

Prove/eliminate 20 `assume` holes in `src/Chap43/OrderedTableStPer.rs`.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 43 | OrderedTableStPer.rs | 20 | 0 | −20 |
| 2 | 43 | OrderedTableMtPer.rs | 0 | 0 | 0 |
| 3 | 52 | AdjTableGraphStPer.rs | 0 | 0 | 0 |
| 4 | 52 | AdjTableGraphMtPer.rs | 0 | 0 | 0 |

**Net: −20 holes. OrderedTableStPer now clean (0 actionable holes).**

## Verification

- 4735 verified, 0 errors
- 2619 RTTs passed
- Project total: 149 holes (on agent1 branch)

## Technique

**Requires propagation** — lifted 5 non-broadcastable type-axiom predicates into
`requires` clauses on `empty()`, `singleton()`, and `tabulate()`:

1. `vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>()`
2. `view_ord_consistent::<Pair<K, V>>()`
3. `spec_pair_key_determines_order::<K, V>()`
4. `vstd::laws_cmp::obeys_cmp_spec::<K>()`
5. `view_ord_consistent::<K>()`

**Broadcast triggers** — replaced 2 broadcastable feq assumes per function with
`assert(obeys_feq_full_trigger::<K/V/Pair>())` which activate `axiom_obeys_feq_full`
from `group_feq_axioms`.

**Propagation chain** (all pass-through, no new assumes):
- StPer trait → MtPer trait → AdjTableGraphMtPer trait
- StPer trait → AdjTableGraphStPer trait
- StPer trait → AugOrderedTableStPer (already had requires)
- MtPer::map()/filter() — type axioms auto-extracted from lock invariant (`inner.spec_orderedtablestper_wf()`)
- PTT test functions — added requires (test harness, not algorithmic code)
- RTTs — outside verus!, requires not checked

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedTableStPer.rs | Removed 20 assumes, added requires + broadcast triggers |
| 2 | 43 | OrderedTableMtPer.rs | Added requires to empty/singleton trait, imports |
| 3 | 52 | AdjTableGraphStPer.rs | Added requires to empty trait, import |
| 4 | 52 | AdjTableGraphMtPer.rs | Added requires to empty trait, imports |
| 5 | 43 | ProveOrderedTableStPer.rs (PTT) | Added requires to 4 test functions |

## Remaining Warnings (not holes)

- 2 × `assume_eq_clone_workaround` in PartialEq::eq and Clone::clone (allowed pattern)
- 1 × `fn_missing_wf_ensures` on `from_sorted_entries` (stretch goal, deferred — requires key-uniqueness loop invariant proof)

## Stretch Goal Status

`from_sorted_entries` needs `ensures result.spec_orderedtablestper_wf()`. This requires
proving `spec_key_unique_pairs_set(tree@)` across the insertion loop, which needs a
key-uniqueness precondition on the input entries and a non-trivial loop invariant.
Deferred to a future round.
