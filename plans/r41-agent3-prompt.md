# R41 Agent 3: Chap41 AVLTreeSet Mt Delegation Wrappers

## Baseline
- Main at `29641a5e`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4281 verified, 192 holes, 30 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md, `src/standards/mod_standard.rs`, and
`src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` before starting.

## Context

Chap41 has 18 holes total (14 real — 4 are Example41_3, skip per CLAUDE.md).
The AVLTreeSetStEph and AVLTreeSetStPer are mature with strong specs. The Mt variants
(MtEph and MtPer) wrap the St variants with RwLock and delegate through the lock, but
many methods are still `external_body`.

## Assignment

### File 1: `src/Chap41/AVLTreeSetMtEph.rs` — 5 holes + 2 warnings

All 5 holes are `external_body` methods that should delegate through the RwLock to the
inner `AVLTreeSetStEph`.

| # | Method | Line | Notes |
|---|--------|------|-------|
| 1 | to_seq | 249 | Acquire read lock, call inner.to_seq(), release |
| 2 | filter | 293 | Acquire write lock, call inner.filter(), build new MtEph |
| 3 | intersection | 346 | Acquire read locks on both, call inner.intersection() |
| 4 | difference | 411 | Acquire read locks on both, call inner.difference() |
| 5 | union | 422 | Acquire read locks on both, call inner.union() |

**Warnings** (lines 311, 372): `fn_missing_requires` on `parallel_filter` and
`parallel_intersect`. These are internal helper functions called from filter/intersection.
Add real requires — likely the predicate/set well-formedness.

**Pattern**: Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`. The standard
pattern is:
1. Acquire read/write lock
2. Call the StEph method on the inner value
3. Connect the inner's ensures to the MtEph's ensures through the lock invariant
4. Release the lock
5. Return result (possibly wrapped in a new MtEph)

For methods returning a new MtEph (filter, intersection, difference, union):
- Compute the StEph result
- Wrap in a new RwLock with appropriate ghost state
- Use `new_arc_rwlock` from `vstdplus/arc_rwlock.rs`

For methods returning non-Self values (to_seq):
- Just acquire read, call inner method, release, return

The RWLOCK_GHOST assumes you see in the analysis are the standard pattern — the lock
invariant says `inner@ == ghost_view`, so after acquiring the lock you can bridge
`inner.method() ensures X` to `self.method() ensures X` via the ghost equality.

### File 2: `src/Chap41/AVLTreeSetMtPer.rs` — 7 holes + 1 warning

| # | Method | Line | Notes |
|---|--------|------|-------|
| 1 | from_seq | 224 | Construct from AVLTreeSeqMtPerS sequence |
| 2 | filter | 276 | Same pattern as MtEph |
| 3 | intersection | 339 | Same pattern |
| 4 | difference | 397 | Same pattern |
| 5 | union | 408 | Same pattern |
| 6 | delete | 498 | Acquire lock, call inner.delete(), wrap new MtPer |
| 7 | insert | 508 | Acquire lock, call inner.insert(), wrap new MtPer |

**Warning** (line 230): `fn_missing_requires` on `parallel_sort`. Add real requires.

MtPer is persistent — methods return new Self. The inner type is `AVLTreeSetStPer<T>`.
Same RwLock delegation pattern but the result is always a new MtPer wrapping a new StPer.

### File 3: `src/Chap41/AVLTreeSetStEph.rs` — 2 holes

| # | Method | Line | Notes |
|---|--------|------|-------|
| 1 | insert | 1085 | `assume(new_vec@.len() < usize::MAX)` |
| 2 | insert_sorted | 1352 | `assume(new_vec@.len() < usize::MAX)` |

Both assumes are about Vec length bounds after insertion. The tree's wf guarantees
`self.elements@.len() < usize::MAX` (line 1084 already asserts this). The new_vec is
built by collecting from the tree + possibly one new element, so its length is at most
`self.elements@.len() + 1`. You need to show `self.elements@.len() + 1 < usize::MAX`
which requires `self.elements@.len() < usize::MAX - 1` or a tighter bound from the
`lemma_wf_implies_len_bound` lemma.

Read the `lemma_wf_implies_len_bound` ensures clause — if it gives `len < usize::MAX`,
then `len + 1 <= usize::MAX` and the new_vec (which is len+1 in the worst case) fits
in usize. But you need `len + 1 < usize::MAX` for from_vec's requires, which means
`len < usize::MAX - 1`. Check if the bound is tight enough.

If the bound from the lemma is only `len < usize::MAX` (not `len < usize::MAX - 1`),
you may need to strengthen the lemma. Look at what the tree invariant actually guarantees
about maximum size.

### Priority

1. AVLTreeSetMtEph to_seq (1 method) — simplest, single read lock
2. AVLTreeSetMtPer insert + delete (2 methods) — core operations
3. AVLTreeSetMtEph filter, intersection, difference, union (4 methods)
4. AVLTreeSetMtPer from_seq, filter, intersection, difference, union (5 methods)
5. AVLTreeSetStEph usize::MAX assumes (2) — may need lemma strengthening
6. All fn_missing_requires warnings (3)

### Expected Results

Conservative: 5-7 holes closed + 3 warnings fixed.
Optimistic: 10-14 holes closed + 3 warnings fixed.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent3-r41-report.md`.
