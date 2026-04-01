# R127 Agent 4 — Parallelize Chap19 ArraySeqMtEph trait methods. AFK.

## Background

`src/Chap19/ArraySeqMtEph.rs` has 11 trait methods that are sequential but APAS expects
parallel span. The file already has verified parallel helpers: `map_par`, `filter_par`,
`reduce_par` (starting around line 1114) using `join()`. These prove the pattern works.

Chap19 is the "Sequences (Array-Based, Ephemeral, Delayed Copy)" variant. The backing
structure differs from Chap18 (which is "Sequences (Array-Based)"), but the parallelization
pattern is the same: split, join, combine.

## Read these standards FIRST

1. `src/standards/using_closures_standard.rs` — closure requires/ensures propagation
2. `src/standards/hfscheduler_standard.rs` — join() with named closures, ParaPair!
3. `src/standards/arc_usage_standard.rs` — Arc deref pattern

## The 11 DIFFERS functions

| # | Function | Current Span | APAS Span | Notes |
|---|----------|-------------|-----------|-------|
| 1 | subseq | O(j) | O(1) | Vec-backed, copies sequentially |
| 2 | append | O(\|a\|+\|b\|) | O(1) | Sequential tabulate |
| 3 | filter | O(n) | O(lg n) | Sequential map+flatten |
| 4 | update | O(n) | O(1) | Clone entire Vec + set |
| 5 | inject | O(n+m) | O(lg degree) | Sequential clone + loop |
| 6 | ninject | O(n+m) | O(1) | Delegates to inject |
| 7 | reduce | O(n) | O(lg n) | Delegates to sequential reduce_iter |
| 8 | scan | O(n) | O(lg n) | Sequential loop |
| 9 | map | O(n) | O(1) | Sequential tabulate |
| 10 | tabulate | O(n) | O(1) | Sequential loop |
| 11 | flatten | O(Σ\|a_i\|) | O(lg\|a\|) | Nested sequential loops |

## Approach

### Step 1: Update trait signatures

Add `Clone + Send + Sync + 'static` bounds to the closure type parameter `F` in both the
trait declaration AND the impl. The Chap19 trait is `ArraySeqMtEphTrait` (single trait,
not split like Chap18). Verify it is only used within this file before changing.

Also check what `T` bounds the existing `_par` functions require and match them.

### Step 2: Rewrite trait method bodies

Use the existing `_par` helpers in this file (`map_par`, `filter_par`, `reduce_par`) as
templates. Replace sequential loops with divide-and-conquer:
- Base case: length 0 or 1
- Recursive: split at midpoint, `join()` both halves, combine

Use `clone_fn`, `clone_fn2`, `clone_pred` from `crate::vstdplus::clone_plus::clone_plus::*`.

**Important**: Chap19's backing structure may differ from Chap18. Read the type definition
and the existing `_par` helpers to understand how splitting works in this module. Don't
blindly copy Chap18 patterns — adapt to Chap19's data representation.

### Step 3: Update annotations

Update Code review Span to match new code. Change `— DIFFERS` to `— matches APAS`
where applicable.

### Step 4: Consider `_par` duplicates

If trait methods now do the same as `_par` helpers, the helpers may be redundant.
Leave them if unsure.

## Reference

Read `src/Chap18/ArraySeqMtEph.rs` lines 1106-1340 for a verified `_par` pattern in a
sibling module. But prioritize your own file's `_par` helpers — they handle Chap19's
specific type structure.

## Validation

Run `scripts/validate.sh isolate Chap19` after changes. Fix verification errors.
Then run `scripts/rtt.sh` to confirm runtime tests pass.

Do NOT run full validate — isolate Chap19 only.

## Rules

- Read the existing `_par` functions thoroughly before writing code.
- Named closures with explicit `ensures` for every `join()` call.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT weaken `ensures` clauses.
- Do NOT change the St files or Chap18 files.
- Preserve all existing RTTs.

## When done

Commit all changes with `git add -A && git commit` and push.

## Report

Write `plans/agent4-r127-report.md` with:
- Table of functions parallelized (# | Function | Old Span | New Span | Status)
- Verification count from isolate validate
- RTT pass count
- Any functions that could not be parallelized and why
