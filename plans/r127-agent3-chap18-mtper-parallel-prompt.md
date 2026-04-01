# R127 Agent 3 — Parallelize Chap18 ArraySeqMtPer trait methods. AFK.

## Background

`src/Chap18/ArraySeqMtPer.rs` has 10 trait methods that are sequential but APAS expects
parallel span. The file already has verified parallel helpers: `map_par`, `filter_par`,
`reduce_par` (starting around line 899) using `join()`. These prove the pattern works.

Your sibling agent (Agent 2) is doing the same work on `ArraySeqMtEph.rs`. The MtPer
file is structurally similar but uses persistent (cloning) semantics.

## Read these standards FIRST

1. `src/standards/using_closures_standard.rs` — closure requires/ensures propagation
2. `src/standards/hfscheduler_standard.rs` — join() with named closures, ParaPair!
3. `src/standards/arc_usage_standard.rs` — Arc deref pattern

## The 10 DIFFERS functions

| # | Function | Current Span | APAS Span | Notes |
|---|----------|-------------|-----------|-------|
| 1 | subseq | O(j) | O(1) | Clone loop |
| 2 | append | O(\|a\|+\|b\|) | O(1) | Sequential loops |
| 3 | filter | O(n) | O(lg n) | Sequential loop |
| 4 | update | O(n) | O(1) | Clone loop |
| 5 | inject | O(n+m) | O(lg degree) | Sequential loops |
| 6 | reduce | O(n) | O(lg n) | Sequential fold |
| 7 | scan | O(n) | O(lg n) | Sequential loop |
| 8 | map | O(n) | O(1) | Sequential loop |
| 9 | tabulate | O(n) | O(1) | Sequential loop |
| 10 | flatten | O(Σ\|a_i\|) | O(lg\|a\|) | Nested loops |

Note: MtPer has no `ninject` (unlike MtEph).

## Approach

### Step 1: Update trait signatures

Add `Clone + Send + Sync + 'static` bounds to the closure type parameter `F` in both the
trait declaration AND the impl. The MtPer traits (`ArraySeqMtPerBaseTrait`,
`ArraySeqMtPerRedefinableTrait`) are only used within this file.

Also check what `T` bounds the existing `_par` functions require and match them.

### Step 2: Rewrite trait method bodies

Use the existing `_par` helpers (`map_par`, `filter_par`, `reduce_par`) as templates.
Replace sequential loops with divide-and-conquer:
- Base case: length 0 or 1
- Recursive: split at midpoint, `join()` both halves, combine

Use `clone_fn`, `clone_fn2`, `clone_pred` from `crate::vstdplus::clone_plus::clone_plus::*`.

### Step 3: Update annotations

Update Code review Span to match new code. Change `— DIFFERS` to `— matches APAS`
where applicable.

### Step 4: Consider `_par` duplicates

If trait methods now do the same as `_par` helpers, the helpers may be redundant.
Leave them if unsure.

## Reference

Read `src/Chap18/ArraySeqMtEph.rs` lines 1106-1340 for the verified `_par` pattern
(Agent 2's file — same pattern, ephemeral variant). Also read the MtPer `_par` helpers
in your own file.

## Validation

Run `scripts/validate.sh isolate Chap18` after changes. Fix verification errors.
Then run `scripts/rtt.sh` to confirm runtime tests pass.

Do NOT run full validate — isolate Chap18 only.

## Rules

- Read the existing `_par` functions thoroughly before writing code.
- Named closures with explicit `ensures` for every `join()` call.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT weaken `ensures` clauses.
- Do NOT change the St files or the MtEph file.
- Preserve all existing RTTs.

## When done

Commit all changes with `git add -A && git commit` and push.

## Report

Write `plans/agent3-r127-report.md` with:
- Table of functions parallelized (# | Function | Old Span | New Span | Status)
- Verification count from isolate validate
- RTT pass count
- Any functions that could not be parallelized and why
