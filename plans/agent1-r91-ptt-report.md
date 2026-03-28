# R91 Agent 1 Report — Fix PTT Infrastructure

## Summary

Fixed 2 Z3 flaky failures that prevented PTT crate compilation, unblocking all 157 PTT tests.

## Fixes

| # | Chap | File | Issue | Fix |
|---|------|------|-------|-----|
| 1 | 42 | TableMtEph.rs | Z3 conjunction flakiness in `subtract()` loop invariant (line 2086) | Split 4-conjunct forall into 4 separate forall invariants |
| 2 | 26 | ETSPStEph.rs | Z3 rlimit flakiness in `etsp_inner()` (line 341) | Added `#[verifier::rlimit(20)]` |

## Fix 1: TableMtEph.rs `subtract()` — Conjunction Split

The loop invariant had a single `forall` with 4 conjuncts joined by `&&`:
- `sources[j]` bounds check
- key correspondence (`old_view[sources[j]].0 == kept@[j].0@`)
- value correspondence (`old_view[sources[j]].1 == kept@[j].1@`)
- key exclusion (`!keys@.contains(kept@[j].0@)`)

Z3 could prove each conjunct individually but sometimes failed the conjunction.
Split into 4 independent `forall` invariants, each with its own trigger. This is
deterministically stable — verified twice in isolate mode, once in full mode.

## Fix 2: ETSPStEph.rs `etsp_inner()` — rlimit Bump

Full validation passed (5320/0) but PTT compilation occasionally hit rlimit on
`etsp_inner`. The function was already fixed for a matching loop in R89 (closed
spec fn pattern), but remained borderline on rlimit. Added `#[verifier::rlimit(20)]`
to give Z3 stable headroom.

## Validation Results

```
validate: 5320 verified, 0 errors
RTT:      3076 tests run, 3076 passed
PTT:       157 tests run,  157 passed
```

## Steps Used: 5 of 20
