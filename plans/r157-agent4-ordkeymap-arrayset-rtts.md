# R157 Agent 4 — RTTs for New OrdKeyMap Operations. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap41/OrdKeyMap.rs` — check all available methods.
Read `tests/Chap41/TestOrdKeyMap.rs` — existing tests.

Report file: `plans/r157-agent4-ordkeymap-rtts-report.md`

## What to test

### R156 agent 1 additions (filter/map/reduce/collect/Clone)

- collect: non-empty map, empty map, correct ordering
- filter: keep some, keep none, keep all
- map_values: transform all values, verify keys unchanged
- reduce: sum values, empty map
- Clone: cloned equals original, modify clone doesn't affect original

### R157 agent 1 additions (if available — domain/tabulate/restrict/subtract)

- domain: correct keys, correct size
- tabulate: from key set, verify values
- restrict: subset of keys, disjoint keys
- subtract: subset of keys, all keys, no keys

### Integration

- Build map, clone, filter, map_values, verify both independently
- tabulate + restrict + subtract round-trip
- domain(tabulate(keys, f)) == keys

## Validation

`scripts/rtt.sh`. Report total count.

## Rules

- Tests go in `tests/Chap41/TestOrdKeyMap.rs`.
- Do NOT modify OrdKeyMap.rs or any Chap43 file.

## When done

RCP.
