# R127 Agent 2 — Parallelize Chap18 ArraySeqMtEph trait methods. AFK.

## Background

`src/Chap18/ArraySeqMtEph.rs` has 11 trait methods (in `ArraySeqMtEphRedefinableTrait`)
that are sequential but APAS expects parallel span. The file already contains verified
parallel helpers: `map_par`, `filter_par`, `reduce_par` (lines ~1106-1340) that use
`join()` with divide-and-conquer. These prove the pattern works.

Your job: make the 11 trait methods parallel.

## Read these standards FIRST

1. `src/standards/using_closures_standard.rs` — closure requires/ensures propagation
2. `src/standards/hfscheduler_standard.rs` — join() with named closures, ParaPair!
3. `src/standards/arc_usage_standard.rs` — Arc deref pattern

## The 11 DIFFERS functions

| # | Function | Current Span | APAS Span | Notes |
|---|----------|-------------|-----------|-------|
| 1 | subseq | O(j) | O(1) | Clone loop → parallel tabulate |
| 2 | append | O(\|a\|+\|b\|) | O(1) | Sequential loops → parallel tabulate |
| 3 | filter | O(n) | O(lg n) | Sequential loop → divide-and-conquer |
| 4 | update | O(n) | O(1) | Clone loop → parallel tabulate |
| 5 | inject | O(n+m) | O(lg degree) | Sequential loops |
| 6 | ninject | O(n+m) | O(1) | Delegates to inject |
| 7 | reduce | O(n) | O(lg n) | Sequential fold → divide-and-conquer |
| 8 | scan | O(n) | O(lg n) | Sequential loop → divide-and-conquer |
| 9 | map | O(n) | O(1) | Sequential loop → parallel tabulate |
| 10 | tabulate | O(n) | O(1) | Sequential loop → parallel generation |
| 11 | flatten | O(Σ\|a_i\|) | O(lg\|a\|) | Nested loops → parallel D&C |

## Approach

### Step 1: Update trait signatures

The trait methods take `f: &F` where `F: Fn(...)` with no `Clone` bound. For `join()`,
each closure arm needs to own a copy of `f`. Add `Clone + Send + Sync + 'static` bounds
to the closure type parameter `F` in both the trait declaration AND the impl.

The traits `ArraySeqMtEphBaseTrait` and `ArraySeqMtEphRedefinableTrait` are only used
within this file — no external callers use the trait names (verified by grep).

Also add these bounds to `T` where needed: the `_par` functions require
`T: Clone + Send + Sync + Eq + 'static`. Check what the existing `_par` functions require.

### Step 2: Rewrite trait method bodies

For each function, replace the sequential loop with divide-and-conquer:
- Base case: length 0 or 1
- Recursive case: split at midpoint, `join()` both halves, combine results

**Use the existing `_par` helpers as your template.** `map_par` (line ~1106), `filter_par`
(line ~1158), `reduce_par` (line ~1240) show exactly how to:
- Split with `subseq_copy`
- Clone closures with `clone_fn` / `clone_fn2` / `clone_pred` from `use crate::vstdplus::clone_plus::clone_plus::*`
- Set up ghost state for the proof
- Write join closures with ensures
- Combine results with `append`

For functions without existing `_par` helpers (scan, tabulate, subseq, append, update,
inject, ninject, flatten), follow the same pattern.

### Step 3: Update annotations

After parallelizing, update the Code review annotation Span to match the new code.
If the new Span matches APAS, change `— DIFFERS: ...` to `— matches APAS`.
If it still differs (e.g., append is O(n) due to Vec concat not O(1)), keep DIFFERS
with accurate reason.

### Step 4: Consider removing `_par` duplicates

After the trait methods are parallel, the separate `_par` functions may be redundant.
If a `_par` function does exactly what the trait method now does, consider whether it
can be removed. But DO NOT remove them if they have different type bounds or are called
from outside. If in doubt, leave them.

## Validation

Run `scripts/validate.sh isolate Chap18` after changes. Fix verification errors.
Then run `scripts/rtt.sh` to confirm runtime tests pass.

Do NOT run full validate — isolate Chap18 only.

## Rules

- Read the existing `_par` functions thoroughly before writing code.
- Use `clone_fn`, `clone_fn2`, `clone_pred` from `vstdplus::clone_plus` — do not write your own.
- Named closures with explicit `ensures` for every `join()` call.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT weaken `ensures` clauses.
- Do NOT change the St files.
- Preserve all existing RTTs.

## When done

Commit all changes with `git add -A && git commit` and push.

## Report

Write `plans/agent2-r127-report.md` with:
- Table of functions parallelized (# | Function | Old Span | New Span | Status)
- Verification count from isolate validate
- RTT pass count
- Any functions that could not be parallelized and why
