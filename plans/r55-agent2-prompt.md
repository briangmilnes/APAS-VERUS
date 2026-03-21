<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 2 — Round 55 Prompt

## Branch

Work on `agent2/ready`. Base: `045bf2ce9`.

## DO NOT TOUCH

- Chap47 (any file)
- Chap37 (any file)
- Chap41/AVLTreeSetStPer.rs (Agent 3 is working there)
- Chap41/AVLTreeSetMtEph.rs, AVLTreeSetMtPer.rs
- Chap43 (any file)

## Assignment: Chap41/AVLTreeSetStEph.rs — 2 tasks

### Task 1: Close capacity bound hole in `union`

**File:** `src/Chap41/AVLTreeSetStEph.rs`
**Line:** 722
**Hole:** `assume(combined@.len() + 1 < usize::MAX as nat)`

The `union` method iterates over `other` and inserts each element into `combined`.
The assume guards the `insert` call which requires `self@.len() + 1 < usize::MAX`.

**Strategy:** Add a `requires` clause to `union` in the trait bounding both input sets:

```
requires
    self.spec_avltreesetsteph_wf(),
    other.spec_avltreesetsteph_wf(),
    self@.len() + other@.len() < usize::MAX as nat,
```

Then prove the loop invariant: `combined@.len() <= self@.len() + other@.len()` (since
union can only shrink or stay same due to deduplication). This bound plus the requires
gives `combined@.len() + 1 < usize::MAX`.

Check all callers of `union` in the codebase to ensure they can satisfy the new requires.
Callers include `AVLTreeSetMtEph::union`, `OrderedSetStEph::union`, `OrderedSetMtEph::union`,
etc. These Mt callers delegate through RwLock and may need the same requires propagated
to their trait definitions.

**DO NOT add `accept()` or weaken the ensures. Prove the bound.**

### Task 2: Add sortedness ensures to trait methods

**File:** `src/Chap41/AVLTreeSetStEph.rs`

The `AVLTreeSetStEphTotalOrderTrait` has a `spec_elements_sorted` predicate, but the
main trait methods (`filter`, `intersection`, `union`, `difference`) do NOT ensure
that the result is sorted. This blocks Chap43 `select` proofs in a future round.

Add `ensures result.spec_elements_sorted()` (or the equivalent formulation) to the
ensures of these four methods in `AVLTreeSetStEphTotalOrderTrait`:

1. `filter` — result is a subset of a sorted set, so sorted
2. `intersection` — result is a subset of a sorted set, so sorted
3. `union` — result merges two sorted sets, sorted
4. `difference` — result is a subset of a sorted set, so sorted

Also ensure that `insert` and `delete` preserve sortedness (they likely already do
via the AVL tree structure, but make it explicit in ensures).

Read `spec_elements_sorted` to understand what it says, then prove each ensures.
This may require intermediate lemmas about how the AVL tree operations preserve
in-order traversal ordering.

**If this is too much work, complete Task 1 first and report progress on Task 2.**

## Validation

Run `scripts/validate.sh` after each change. Show full output. Fix all warnings.
Do not leave trigger warnings.

## Report

Write `plans/agent2-round55-report.md` with holes before/after table including
Chap column.
