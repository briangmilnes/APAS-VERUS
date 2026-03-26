# R82b Agent 2 — Fix Chap43 OrderedSetMtEph + OrderedTableMtPer, STEP 15

## Objective

Fix `OrderedSetMtEph.rs` and `OrderedTableMtPer.rs` in Chap43. These were verified
before R82 but now have postcondition failures after the OrderedSetStPer rewrite.

## Isolation

Use isolated validation during development:
```bash
scripts/validate.sh isolate Chap43
```
This includes Chap43 + all transitive deps (Chap02, 18, 19, 23, 37, 38, 41, 42).
Before pushing, run a full `scripts/validate.sh` to confirm.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## What to fix

### 1. Uncomment both files in lib.rs

Change the commented-out entries in Chap43:
```rust
    pub mod OrderedSetMtEph;
    pub mod OrderedTableMtPer;
```

### 2. Fix postcondition/precondition failures

Agent4 rewrote `OrderedSetStPer` in R82 — the trait API changed:
- `spec_orderedsetstper_wf` now includes `obeys_cmp_spec` + `view_ord_consistent`
- `empty()` and `singleton()` now `requires obeys_cmp_spec, view_ord_consistent`
- Postconditions on first/last/next/previous changed from TotalOrder::le to cmp_spec style
- Iterator changed from borrowing to owning

`OrderedSetMtEph` wraps `OrderedSetStEph` (not StPer) in an RwLock, so it should
be less affected. `OrderedTableMtPer` wraps `OrderedTableStPer` — check if its
contracts still match.

Read the working files first:
- `src/Chap43/OrderedSetStPer.rs` — the rewritten version (reference)
- `src/Chap43/OrderedSetStEph.rs` — the StEph version OrderedSetMtEph wraps
- `src/Chap43/OrderedTableStPer.rs` — the StPer version OrderedTableMtPer wraps

## Important

- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.
- Do NOT modify OrderedSetStPer.rs or OrderedSetStEph.rs — only fix the MtEph/MtPer callers.

## STEP 15

## Validation

Before pushing: restore lib.rs, run full `scripts/validate.sh`, `scripts/rtt.sh`,
`scripts/ptt.sh`. Push to `agent2/ready`.

## Report

Write `plans/agent2-round82b-report.md`.
