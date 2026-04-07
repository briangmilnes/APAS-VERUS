# R156 Agent 4 — Comprehensive OrdKeyMap RTTs. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read `src/Chap38/OrdKeyMap.rs` — know all available methods.
Read `tests/Chap38/TestOrdKeyMap.rs` — existing tests.

Report file: `plans/r156-agent4-ordkeymap-rtts-report.md`

## Problem

OrdKeyMap now has ~18 methods but only ~46 RTTs. The new R155 methods
(union_with, intersect_with, first_key, last_key, get_key_range,
split_rank_key) have no RTTs yet.

## What to add

### union_with / intersect_with tests
- union_with: collision uses combiner (e.g., add values)
- union_with: no collision (same as plain union)
- intersect_with: collision uses combiner
- intersect_with: disjoint maps (empty result)

### first_key / last_key tests
- first_key on populated map
- first_key on empty map (None)
- last_key on populated map
- last_key on empty map (None)
- first_key == last_key on singleton

### get_key_range tests
- range covering all keys
- range covering subset
- range with no keys in bounds
- range with exact boundary keys
- empty map

### split_rank_key tests
- split at 0 (empty left)
- split at size (empty right)
- split at middle
- left.size + right.size == original.size
- all keys in left < all keys in right

### split disjointness tests
- verify left.dom().disjoint(right.dom()) after split
- round-trip: union(left, right) + insert(k) reconstructs original

### Stress tests
- Build map with 200 entries, run all operations, verify invariants
- Sequential insert/delete/find cycle

## Validation

`scripts/rtt.sh`. Report total test count.

## Rules

- Do NOT modify OrdKeyMap.rs or any Chap43 source file.
- Tests go in `tests/Chap38/TestOrdKeyMap.rs`.

## When done

RCP.
