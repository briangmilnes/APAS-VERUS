# R48 Agent 1: Chap41 (2 real holes) + Chap43 (4 holes)

## Assignment

Clean Chap41's 2 real holes (Example41_3 is skipped per CLAUDE.md). If Chap41
cleans, Chap43 may benefit since it depends on Chap41::AVLTreeSetStEph.

## Baseline

38 holes total. 4419 verified. Your targets: Chap41 (2 real) + Chap43 (4) = 6 holes.

## REQUIRED READING

1. `src/standards/arc_usage_standard.rs`
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`
3. `src/standards/using_closures_standard.rs`

## Current Holes

Run `scripts/holes.sh src/Chap41/` and `scripts/holes.sh src/Chap43/` to verify.

### Chap41 — 2 real holes (ignore Example41_3.rs per CLAUDE.md)

| # | Chap | File | Line | Function | Type | Notes |
|---|------|------|------|----------|------|-------|
| 1 | 41 | AVLTreeSetStEph.rs | 1085 | insert | assume | `assume(new_vec@.len() < usize::MAX)` |
| 2 | 41 | AVLTreeSetStEph.rs | 1352 | insert_sorted | assume | `assume(new_vec@.len() < usize::MAX)` |

Both are capacity bounds. The AVL tree stores elements in a Vec. After insert,
the Vec length must be < usize::MAX. This should follow from spec_wf requiring
the tree size is bounded, or from the tree's finite size. Options:

- Add `self@.len() < usize::MAX - 1` to the insert requires clause (callers
  prove the set isn't at max capacity).
- Prove from existing spec_wf that the tree size is bounded.
- If the tree tracks `self@.len()` in spec, assert that `self@.len() + 1 < usize::MAX`
  follows from the requires.

Read the function, understand what `new_vec` is, and find the right requires.

### Chap43 — 4 holes

| # | Chap | File | Line | Function | Type | Notes |
|---|------|------|------|----------|------|-------|
| 3 | 43 | AugOrderedTableMtEph.rs | 672 | reduce_range_parallel | external_body | Parallel reduce |
| 4 | 43 | AugOrderedTableStPer.rs | 124 | lemma_reducer_clone_total | assume | Closure requires assume |
| 5 | 43 | OrderedSetStEph.rs | 1117 | select | assume | Filter cardinality |
| 6 | 43 | OrderedSetStPer.rs | 1031 | select | assume | Filter cardinality |

**#3 reduce_range_parallel**: This is a parallel version of reduce_range. Read the
St version to understand the spec. The Mt version needs fork-join with closure
propagation. Read `using_closures_standard.rs`.

**#4 lemma_reducer_clone_total**: The assume is on closure requires after clone.
Read `using_closures_standard.rs` — the fix is likely lifting the requires to the
function's own requires clause.

**#5-6 select**: Filter cardinality assume. `select(k)` returns the k-th element.
The assume is probably that filtering produces the right count. May need a vstd
lemma about filter + cardinality.

## What NOT to do
- Do NOT modify Example41_3.rs (skip per CLAUDE.md).
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT use Arc<RwLock> as struct field — read arc_usage_standard.rs.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap41/` + `src/Chap43/`.
Write your report to `plans/agent1-round48-report.md`.
