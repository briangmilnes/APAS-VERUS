# R158 Agent 2 — Final StPer Delegation to OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap41/OrdKeyMap.rs` — all 27 methods available.
Read `src/Chap43/OrderedTableStPer.rs` — your file.
Read `src/Chap43/OrderedTableStEph.rs` — agent 2 (R156) got union/intersect
working using a trigger bridge technique. If StPer union/intersect aren't
delegated yet, apply the same technique.

Report file: `plans/r158-agent2-stper-final-delegate-report.md`

## Problem

Same as Agent 1 but for StPer. OrdKeyMap now has domain, tabulate, restrict,
subtract, filter, collect, map_values, reduce, Clone. Delegate everything
possible.

## What to delegate

Check every method that still accesses `self.tree.inner`. For each:
- If OrdKeyMap has a matching method, delegate
- For persistent semantics: clone `self.tree` first if the OrdKeyMap method
  is `&mut self`, then wrap result

Methods that should now be delegable:
- `domain` → `self.tree.domain()`
- `tabulate` → `OrdKeyMap::tabulate(keys, f)`
- `restrict` → `self.tree.restrict(keys)`
- `subtract` → `self.tree.subtract(keys)`
- `filter` → `self.tree.filter(pred)`
- `collect` → `self.tree.collect()`
- `insert` / `delete` → clone + `self.tree.insert()`/`delete()` (OrdKeyMap has Clone now)

After delegation, delete dead bridge lemmas and spec fns.

## Validation

`scripts/validate.sh isolate Chap43` after each method.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs or OrderedTableStEph.rs.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- All existing RTTs must pass.

## When done

RCP. Report line count before/after.
