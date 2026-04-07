# R155 Agent 3 — Delegate Remaining OrderedTableStEph Methods. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs` — check what methods are available NOW.
Read `src/Chap43/OrderedTableStEph.rs` — your file.

Report file: `plans/r155-agent3-ordtable-delegate-remaining-report.md`

## Problem

After R154, these OrderedTableStEph methods still use `self.tree.inner`:

- `union` (takes combiner fn)
- `intersection` (takes combiner fn)
- `split_key_iter` (needs disjointness)
- `get_key_range_iter`
- `split_rank_key_iter`
- `first_key_iter` / `last_key_iter`
- `domain` / `collect` / `tabulate` / `map` / `filter` / `reduce`
- `restrict` / `subtract` / `join_key`
- `empty` / `singleton` (need wf from new)

Agents 1+2 are adding combiner union/intersect, split disjointness, new()
wf, first/last, and other ops to OrdKeyMap in parallel.

## What to do

Check what OrdKeyMap methods are available on your branch (agents 1+2 may
have pushed by the time you read this, but you're on a stale branch).

For each OrderedTableStEph method:
1. If OrdKeyMap has the corresponding method, delegate and delete bridge proof
2. If OrdKeyMap doesn't have it yet, leave it using `self.tree.inner`
3. Track which methods you delegated vs couldn't

### Methods you CAN delegate regardless of agents 1+2

These use OrdKeyMap methods that already exist from R153:

- `empty` → `OrdKeyMap::new()` (if agent 1 fixes wf ensures)
- `singleton` → `OrdKeyMap::new()` + `OrdKeyMap::insert()`

### After each delegation

Delete or comment out the dead bridge proof. Validate with
`scripts/validate.sh isolate Chap43`.

### Target

Delete remaining bridge lemmas and dead spec fns once all methods delegate.
Report which bridge lemmas are still alive and why.

## Validation

`scripts/validate.sh isolate Chap43` after each method.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- All existing RTTs must pass.

## When done

RCP. Report: line count before/after, methods delegated, methods still pending.
