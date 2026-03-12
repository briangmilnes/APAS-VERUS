# Agent 1 Plan — Foundation Sequences + Near-Clean

## Current State (actual, differs from PBOGH.md)

| # | Chap | Holes | Type | Status |
|---|------|-------|------|--------|
| 1 | 18 | 6 | trivial spec_wf { true } | 6 files, all Vec-backed |
| 2 | 19 | 3 | trivial spec_wf { true } | 3 files, all Vec-backed |
| 3 | 12 | 1 | trivial spec_wf { true } | Exercise12_5.rs (ConcurrentStackMt) |
| 4 | 52 | 1 | external_body | EdgeSetGraphMtPer.rs:out_neighbors |
| 5 | 55 | 0 | — | Already clean |
| 6 | 57 | 0 | — | Already clean |

**Total: 11 holes** (not 17 — Chap55/57 went clean since PBOGH was written).

## Analysis

### Trivial spec_wf { true } (10 holes across 10 files)

All Chap18/19 types are Vec-backed: `struct FooS<T> { seq: Vec<T> }`.
The correct wf body is `true` because `Vec@.len() <= usize::MAX` is not
axiomatically provable in Verus. This is a known Verus limitation documented
in MEMORY.md. The hole checker says each "needs // accept hole".

Chap12/Exercise12_5.rs has `ConcurrentStackMt` wrapping `Mutex<Vec<T>>` —
same situation.

### External_body (1 hole in Chap52)

`EdgeSetGraphMtPer.rs:out_neighbors` uses `filter` with a runtime `Fn` closure.
The spec (trait ensures) does not need closures — it can express the result as a
set comprehension. The body's closure can be bridged to spec via a `Ghost(spec_fn)`
companion per Pattern C in `using_closures_standard.rs`. This is provable work,
not an accept situation.

## Proposed Work (3 phases)

### Phase 1: Add `// accept hole` to 10 trivial spec_wf holes

Add `// accept hole: Vec-backed, true is correct` comment to each trivial
spec_wf line in these files:

| # | Chap | File | Line |
|---|------|------|------|
| 1 | 18 | ArraySeqMtEph.rs | 529 |
| 2 | 18 | ArraySeqMtPer.rs | 323 |
| 3 | 18 | ArraySeqStEph.rs | 354 |
| 4 | 18 | ArraySeqStPer.rs | 348 |
| 5 | 18 | LinkedListStEph.rs | 297 |
| 6 | 18 | LinkedListStPer.rs | 289 |
| 7 | 19 | ArraySeqMtEph.rs | 455 |
| 8 | 19 | ArraySeqStEph.rs | 422 |
| 9 | 19 | ArraySeqStPer.rs | varies |
| 10 | 12 | Exercise12_5.rs | 53 |

### Phase 2: Prove EdgeSetGraphMtPer.rs:out_neighbors

Remove `#[verifier::external_body]` and prove the function:
1. Write spec-level ensures on the trait (set comprehension, no closures).
2. Check whether `AVLTreeSetMtPer::filter` accepts a `Ghost(spec_fn)` companion.
3. If yes: bridge the runtime closure to spec via Pattern C, prove the body.
4. If no: investigate what filter's spec provides and whether the proof can
   be built from available ensures (e.g., subset_of + membership reasoning).

### Phase 3: Validate + regenerate holes

1. Run `scripts/validate.sh` — Phase 1 is comment-only; Phase 2 changes proof.
2. Run `scripts/holes.sh` on each chapter — confirm hole counts.
3. Regenerate per-chapter analysis logs.

## Expected Outcome

| # | Chap | Before | After | New Clean? |
|---|------|--------|-------|------------|
| 1 | 18 | 6 | 0 | Yes (+1) |
| 2 | 19 | 3 | 0 | Yes (+1) |
| 3 | 12 | 1 | 0 | Yes (+1) |
| 4 | 52 | 1 | 0 or 1 | Yes if proved |
| 5 | 55 | 0 | 0 | Already clean |
| 6 | 57 | 0 | 0 | Already clean |

**Phase 1: +3 clean chapters** (18, 19, 12). 10 holes closed via `// accept hole`.
**Phase 2: +1 clean chapter** (52) if `out_neighbors` proves. 1 hole closed via proof.
**Worst case: +3 clean chapters, 10 holes closed. Best case: +4, 11 closed.**

## Risk

Phase 1: Low. Comment-only changes, no verification impact.
Phase 2: Medium. Depends on `AVLTreeSetMtPer::filter`'s spec and whether the
proof obligation is tractable. If blocked, document the blocker and move on.
