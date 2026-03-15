# Agent 1 — Round 19: Strengthen Chap43 Table Operation Specs

## Mission

The 12 table operations in OrderedTableStEph, OrderedTableStPer, AugOrderedTableStEph,
and AugOrderedTableStPer all have `ensures self@.dom().finite()` only — pure weak specs.
Strengthen them to match what you did for Chap42 TableStEph/TableStPer in R17.

## Required Reading

- `src/standards/using_closures_standard.rs` — Pattern C for filter, Pattern A/B for
  tabulate/map.
- `src/Chap42/TableStEph.rs` — your R17 work. The ensures patterns you wrote there are
  the template for Chap43.
- `src/standards/total_order_standard.rs` — for ordering-aware specs.

## The 12 Operations

Each of these appears in 4 files (OrderedTableStEph, OrderedTableStPer,
AugOrderedTableStEph, AugOrderedTableStPer) = **48 functions total**.

| # | Function | Correct Ensures (from Chap42 R17 pattern) |
|---|----------|------------------------------------------|
| 1 | insert | `self@ == old(self)@.insert(key@, value@)` or 3-case value spec |
| 2 | domain | `result@ == self@.dom()` |
| 3 | tabulate | `result@.dom() == keys`, `forall\|k\| f.ensures((&k,), result@[k@])` |
| 4 | map | `result@.dom() == self@.dom()`, `forall\|k\| f.ensures((&k, &self@[k]),result@[k])` |
| 5 | filter | Already has Ghost(spec_fn) from R18. Add backward completeness if missing |
| 6 | reduce | `self@.dom().len() == 0 ==> result@ == base@` |
| 7 | intersection | `result@.dom() == self@.dom().intersect(other@.dom())`, combine.ensures values |
| 8 | union | `result@.dom() == self@.dom().union(other@.dom())`, 3-case value spec |
| 9 | difference | `result@.dom() == self@.dom().difference(other@.dom())`, value preservation |
| 10 | restrict | `result@.dom() == self@.dom().intersect(keys@)`, value preservation |
| 11 | subtract | `result@.dom() == self@.dom().difference(keys@)`, value preservation |
| 12 | join_key | `result@.dom() == left@.dom().union(right@.dom())`, value preservation |

## Procedure

1. **Read** `src/Chap42/TableStEph.rs` — copy the ensures patterns from your R17 work.
2. For each of the 4 Chap43 files:
   a. Read the current trait.
   b. Replace `ensures self@.dom().finite()` with the correct ensures.
   c. Keep `finite()` as an additional ensures (don't remove it).
   d. Add `external_body` to impl fns where proof breaks.
3. `scripts/validate.sh` — 0 errors.

## Important

- The Chap42 TableStEph specs you wrote in R17 are the template. Copy them.
- Do NOT leave any function with only `ensures finite()`.
- Add `external_body` freely. Strong spec + external_body > weak spec.
- Do NOT modify Chap42 files or any non-Chap43 files.
- Filter already has Ghost(spec_fn) from R18 — just verify backward completeness is present.

## Deliverables

- 48 functions strengthened across 4 files.
- `plans/agent1-round19-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
