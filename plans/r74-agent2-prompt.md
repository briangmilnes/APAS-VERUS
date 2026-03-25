# R74 Agent 2 — Prove Chap43 OrderedTableStEph type-axiom assumes + iterator holes (15 holes)

## Objective

Prove or eliminate the 15 holes in `src/Chap43/OrderedTableStEph.rs`:
- 13 `assume` holes in `singleton` and `tabulate` (same type-axiom pattern as StPer)
- 2 `external_body` holes: `rank_key_iter` (line ~3417) and `select_key` (line ~3444)

## Strategy

### Type-axiom assumes (13 holes)

Same 7 predicates as OrderedTableStPer: `obeys_feq_fulls`, `obeys_feq_full`,
`obeys_cmp_spec`, `view_ord_consistent`, `spec_pair_key_determines_order` on `Pair<K,V>`
and `K`.

Approach:
1. Read `src/Chap43/OrderedTableStEph.rs` — focus on `singleton` (line ~1107) and
   `tabulate` (lines ~1335 and ~1462).
2. Read trait bounds. Search vstd for broadcast lemmas.
3. Check `src/Chap43/AugOrderedTableStEph.rs` (0 holes) for patterns.
4. Coordinate approach with Agent 1 (OrderedTableStPer has 20 of the same pattern).

### rank_key_iter and select_key (2 external_body)

These are real algorithmic proofs:

- **`rank_key_iter`** (line ~3417): Count elements with key strictly less than given key.
  Uses iterator loop. Need loop invariant connecting traversal position to filter count.
- **`select_key`** (line ~3444): Return the i-th smallest key. Depends on `rank_key_iter`
  being proved. Need invariant showing sorted sequence contains all domain elements.

Approach:
1. Read the existing `external_body` implementations carefully.
2. Read the `ensures` clauses — these define what must be proved.
3. Write loop invariants connecting the iterator position to the spec.
4. Search for similar proved iterator loops in the codebase (e.g., `src/Chap18/ArraySeqStEph.rs`).

## Bonus (if time permits)

Fix the 4 iterator `external_body` holes in Chap43 Mt files:
- `src/Chap43/OrderedTableMtEph.rs`: `iter` (line ~665), `into_iter` (line ~801)
- `src/Chap43/AugOrderedTableMtEph.rs`: `iter` (line ~752), `into_iter` (line ~765)

## Also fix

- Warning: `fn_missing_wf_ensures` on `from_sorted_entries` (line ~3811). Add the real
  ensures clause.

## Assigned files

| # | File | Holes |
|---|------|-------|
| 1 | src/Chap43/OrderedTableStEph.rs | 13 assume + 2 external_body |

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
- Commit to your branch, push to `origin/agent2/ready`.
- Write report to `plans/agent2-round74-report.md`.
