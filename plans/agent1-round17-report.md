# Agent 1 — Round 17 Report

## Mission

Spec audit of Chap41 (Sets ADT 41.1) and Chap42 (Tables ADT 42.1) against APAS prose.
Fix weak/missing requires/ensures. Strong spec + external_body > weak spec + proved body.

## Results

| # | Chap | File | Function | Before | After | Holes Δ |
|---|------|------|----------|--------|-------|---------|
| 1 | 41 | ArraySetStEph.rs | from_seq | wf only | constructed@ =~= seq@.to_set() | 0 |
| 2 | 41 | AVLTreeSetStEph.rs | from_seq | wf only | constructed@ =~= seq@.to_set() | 0 |
| 3 | 41 | AVLTreeSetStPer.rs | from_seq | wf only | constructed@ =~= seq@.to_set() | 0 |
| 4 | 42 | TableStEph.rs | tabulate | domain only | domain + f.ensures values | +1 |
| 5 | 42 | TableStEph.rs | map | domain only | domain + f.ensures values | +1 |
| 6 | 42 | TableStEph.rs | intersection | domain only | domain + combine.ensures values | +1 |
| 7 | 42 | TableStEph.rs | union | domain only | domain + 3-case value spec | +1 |
| 8 | 42 | TableStEph.rs | insert | domain only | domain + 3-case value spec | +1 |
| 9 | 42 | TableStPer.rs | tabulate | domain only | domain + f.ensures values | +1 |
| 10 | 42 | TableStPer.rs | map | domain only | domain + f.ensures values | +1 |
| 11 | 42 | TableStPer.rs | intersection | domain only | domain + combine.ensures values | +1 |
| 12 | 42 | TableStPer.rs | union | domain + partial values | domain + 3-case value spec | +1 |

## Hole Count

| Metric | Before | After |
|--------|--------|-------|
| Total holes | 103 | 112 |
| Verified fns | 4150 | 4128 |
| Errors | 0 | 0 |

Hole increase is intentional: 9 new external_body holes from strengthening specs that
previously had weak ensures with proved bodies. Each new hole preserves a strong spec
that captures APAS prose semantics (value formulas, not just domain correctness).

## Techniques Used

- **from_seq proof:** Loop invariant tracking forward containment (`seq@[j] in constructed@`)
  and backward containment (`constructed@ subset_of seq@.to_set()`), with `lemma_cloned_view_eq`
  bridging clone view preservation. Proved in all 3 Chap41 files with zero new holes.

- **Closure value specs:** Used `f.ensures((&key_arg,), result)` existential pattern from
  the closure standard (see `src/standards/using_closures_standard.rs`) and from
  TableStPer's existing `insert` spec. Applied consistently to tabulate, map, intersection,
  union, and insert.

## Partial Specs (No Fix)

| # | Chap | Function | Gap | Reason |
|---|------|----------|-----|--------|
| 1 | 41 | filter (all 3 files) | Missing predicate completeness | No Ghost(spec_fn) companion |
| 2 | 42 | filter (both files) | Missing predicate completeness | No Ghost(spec_fn) companion |

Filter completeness requires a `Ghost<spec_fn(T) -> bool>` parameter to mirror the exec
closure at spec level. Adding this would change the trait signature and all callers. The
current `subset_of` + value preservation spec is the strongest expressible without signature
changes.

## Deliverables

- `src/Chap41/analyses/spec-audit.md` — per-function classification
- `src/Chap42/analyses/spec-audit.md` — per-function classification
- Strengthened ensures in 5 source files (3 Chap41, 2 Chap42)
- `plans/agent1-round17-report.md` — this report

## Commit

Commit hash: 36fc1308
