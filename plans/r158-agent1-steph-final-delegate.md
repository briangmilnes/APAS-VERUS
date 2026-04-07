# R158 Agent 1 — Final StEph Delegation to OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap41/OrdKeyMap.rs` — check ALL available methods. OrdKeyMap now has
27 methods including domain, tabulate, restrict, subtract, filter, map_values,
reduce, collect, Clone.
Read `src/Chap43/OrderedTableStEph.rs` — your file.

Report file: `plans/r158-agent1-steph-final-delegate-report.md`

## Problem

OrdKeyMap now has all operations that OrderedTableStEph needs. Several methods
still use `self.tree.inner` because OrdKeyMap didn't have equivalents when they
were last touched. Now it does.

## What to delegate

Check every method in OrderedTableStEph that still accesses `self.tree.inner`.
For each one, check if OrdKeyMap has a matching method and delegate.

Methods that should now be delegable:
- `domain` → `self.tree.domain()`
- `tabulate` → `OrdKeyMap::tabulate(keys, f)`
- `restrict` → `self.tree.restrict(keys)`
- `subtract` → `self.tree.subtract(keys)`
- `filter` → `self.tree.filter(pred)`
- `collect` → `self.tree.collect()` (may need type adaptation if return types differ)
- `reduce` → `self.tree.reduce(f, id)` (may need signature adaptation)
- `map` → `self.tree.map_values(f)` (may need signature adaptation)

After each delegation, check if bridge lemmas become dead. Delete dead lemmas.

## Return type mismatches

Some OrdKeyMap methods return different types than OrderedTable:
- OrdKeyMap::collect returns `Vec<Pair<K,V>>`, OrderedTable may return `AVLTreeSeqStPerS`
- OrdKeyMap::reduce takes `(&V, &V) -> V`, OrderedTable may take `(R, &K, &V) -> R`

For mismatches: if the conversion is trivial (wrap/unwrap), delegate and convert.
If the semantics are different, leave the existing implementation.

## After all delegation

Find all remaining `self.tree.inner` references. These are the methods that
genuinely can't delegate. Report them and why.

Delete all bridge lemmas and spec fns with zero callers.

## Validation

`scripts/validate.sh isolate Chap43` after each method.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- All existing RTTs must pass.

## When done

RCP. Report: line count before/after, methods delegated, methods that can't delegate and why.
