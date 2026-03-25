# R74 Agent 1 — Prove Chap43 OrderedTableStPer type-axiom assumes (20 holes)

## Objective

Prove or eliminate the 20 `assume` holes in `src/Chap43/OrderedTableStPer.rs`. All 20 are
type-axiom assumptions in `empty`, `singleton`, and `tabulate` — the same 7 predicates
repeated across 3 functions:

1. `obeys_feq_fulls::<K, V>()`
2. `obeys_feq_full::<Pair<K, V>>()`
3. `obeys_cmp_spec::<Pair<K, V>>()`
4. `view_ord_consistent::<Pair<K, V>>()`
5. `spec_pair_key_determines_order::<K, V>()`
6. `obeys_cmp_spec::<K>()`
7. `view_ord_consistent::<K>()`

## Strategy

These assumes exist because the generic types `K` and `V` don't have trait bounds that
guarantee these properties. The fix is one of:

1. **Add requires clauses** that demand these properties from callers (best if callers can
   prove them via broadcast lemmas or their own bounds).
2. **Add trait bounds** to `K`/`V` that imply these properties (e.g., `K: TotalOrder` implies
   `obeys_cmp_spec::<K>()`).
3. **Write broadcast lemmas** that prove these properties from existing bounds.

### Approach

1. Read `src/Chap43/OrderedTableStPer.rs` — focus on `empty` (line ~823), `singleton`
   (line ~842), `tabulate` (line ~1006, ~1124).
2. Read the trait definition for `OrderedTableStPerTrait` to understand the current bounds
   on `K` and `V`.
3. Search vstd for `obeys_cmp_spec` broadcast lemmas: `veracity-search 'obeys_cmp_spec'`.
4. Search for `view_ord_consistent`: `veracity-search 'view_ord_consistent'`.
5. Search for `obeys_feq_fulls`: `veracity-search 'obeys_feq_fulls'`.
6. Check how `src/Chap43/OrderedTableStEph.rs` handles the same assumes (it has 13 of the
   same pattern — coordinate if possible).
7. Read `src/Chap43/AugOrderedTableStPer.rs` (0 holes) — it may show how to avoid these
   assumes.

### Key insight

The StPer file wraps StEph. If StEph's functions require these axioms, StPer must either
propagate them as requires or prove them from its own bounds. Check whether the trait bounds
on `OrderedTableStPerTrait` already imply these properties.

## Also fix

- Warning: `fn_missing_wf_ensures` on `from_sorted_entries` (line ~3382). Add the real
  `ensures result.spec_orderedtablestper_wf()` clause.

## Assigned files

| # | File | Holes |
|---|------|-------|
| 1 | src/Chap43/OrderedTableStPer.rs | 20 assume |

## Validation

```bash
scripts/validate.sh    # must pass: 4735+ verified, 0 errors
scripts/rtt.sh         # must pass: 2619+ tests
```

Fix all warnings in your assigned file before committing.

## Rules

- Read `CLAUDE.md` first.
- Do NOT weaken ensures to make proofs easier.
- Do NOT add `accept()` or convert `assume` to `accept`.
- Do NOT add `requires true` or tautological requires.
- Propagate real requires if needed — callers must prove the obligation.
- Commit to your branch, push to `origin/agent1/ready`.
- Write report to `plans/agent1-round74-report.md`.
