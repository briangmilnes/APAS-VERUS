# Agent 1 — Round 14 Report

## Summary

OrderedTableMtEph fully proved: 11 external_body holes eliminated (11 → 0).
AugOrderedTableMtEph trait signatures updated with closure requires for map/filter/reduce.
AugOrderedTableStPer assumes deferred (requires cascading trait changes across ~14 methods).

## Verification

- **4049 verified, 0 errors** (up from 4012 in Round 13)
- **184 total project holes** (down from 217 at Round 13 end)
- Agent 1 contribution: **-11 holes**

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableMtEph.rs | 11 | 0 | -11 |
| 2 | 43 | AugOrderedTableMtEph.rs | 2 | 2 | 0 |
| 3 | 43 | AugOrderedTableStPer.rs | 2 | 2 | 0 |

## Functions Proved (OrderedTableMtEph.rs)

All 11 external_body stubs removed:

| # | Function | Technique |
|---|----------|-----------|
| 1 | collect | collect+while loop + from_vec |
| 2 | from_sorted_entries | while loop + Pair clone decomposition |
| 3 | previous_key | collect + reverse while + cmp pattern |
| 4 | next_key | collect + forward while + cmp pattern |
| 5 | rank_key | collect + while + cmp counter |
| 6 | get_key_range | collect + while + cmp range filter |
| 7 | split_key | collect + while + cmp partition |
| 8 | split_rank_key | collect + while + index partition |
| 9 | map | collect + while + f(k,v) + from_sorted_entries |
| 10 | filter | collect + while + conditional push + from_sorted_entries |
| 11 | reduce | collect + while + fold accumulator |

## Key Techniques

- **collect+while loop pattern**: Replace `for i in 0..len` with `while i < len` + invariants for Verus
- **Pair clone decomposition**: `Pair(pair.0.clone(), pair.1.clone())` instead of `pair.clone()` (Pair::clone is external)
- **cmp pattern**: `pair.0.cmp(k)` with `std::cmp::Ordering` match (Verus doesn't support `<`/`>` on references)
- **Closure requires in invariants**: `forall|k: &K, v: &V| f.requires((k, v))` must be in while loop invariants (trait requires erased by loop)
- **Postcondition in loop invariant**: `self@.dom().finite()` needed in while loop invariants for early returns

## AugOrderedTableMtEph Changes

Updated trait signatures to propagate closure requires from OrderedTableMtEph:
- `map`: added `requires forall|k: &K, v: &V| f.requires((k, v))`
- `filter`: added `requires forall|k: &K, v: &V| f.requires((k, v))`
- `reduce`: added `requires forall|r: R, k: &K, v: &V| f.requires((r, k, v))`

## Deferred

- **AugOrderedTableStPer assumes** (calculate_reduction, join_key): Fixing requires adding `forall|v1: &V, v2: &V| reducer.requires((v1, v2))` to ~14 trait method signatures and propagating through all callers. Deferred to future round.

## Commit

Pending user approval.
