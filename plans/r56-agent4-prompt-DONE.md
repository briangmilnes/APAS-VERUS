<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 4 — Round 56 Prompt

## Branch

Work on `agent4/ready`. Base: `fd7f38bb4`.

## DO NOT TOUCH

- Chap47 (any file)
- Chap41 (any file — Agent 3)
- Chap43/OrderedTableStPer.rs (Agent 2)

## Assignment: Chap45 capacity holes + Chap43 select holes

### Task 1: BalancedTreePQ.rs — 2 capacity holes

**File:** `src/Chap45/BalancedTreePQ.rs`

**Hole 1 (line 253):** `assume(vals@.len() < usize::MAX)` in `insert`
**Hole 2 (line 363):** `assume(values@.len() < usize::MAX)` in `meld`

Both holes guard calls to `AVLTreeSeqStPerS::from_vec`. The wf chain from R55 gives
`self.elements@.len() < usize::MAX` via the broadcast group.

For `insert`: the new vec has `self.elements@.len() + 1` elements. Need
`self.elements@.len() + 1 < usize::MAX`. Add this as a requires.

For `meld`: the merged vec has `self.elements@.len() + other.elements@.len()` elements.
Add `self@.len() + other@.len() < usize::MAX` to requires.

Check callers (HeapsortExample.rs) and propagate requires if needed.

### Task 2: OrderedSetStEph.rs + OrderedSetStPer.rs — select holes

**File:** `src/Chap43/OrderedSetStEph.rs` (line 1134)
**File:** `src/Chap43/OrderedSetStPer.rs` (line 1031)

Both have `assume(self@.filter(...).len() == i as int)` in `select`. The proof needs
sortedness of the backing AVL tree sequence.

In R55, Agent 2 added `filter_sorted`, `intersection_sorted`, `difference_sorted`, and
`union_sorted` to `AVLTreeSetStEphTotalOrderTrait`. These ensure `spec_elements_sorted()`
on results.

**Strategy:**

1. Read the sorted variants in `src/Chap41/AVLTreeSetStEph.rs` to understand their API.

2. Add `spec_elements_sorted()` to `spec_orderedsetsteph_wf()` / `spec_orderedsetstper_wf()`.

3. Switch OrderedSet operations that need sortedness to call the `_sorted` variants:
   - `filter` calls `filter_sorted`
   - `intersection` calls `intersection_sorted`
   - `union` calls `union_sorted`
   - `difference` calls `difference_sorted`

4. Prove that `empty()`, `singleton()`, `insert()`, `delete()` also maintain sortedness.
   For `insert` and `delete`: use `insert_sorted` and `delete_sorted` from the AVL trait.

5. Once sortedness is in wf, prove the `select` filter-count lemma:
   For a sorted sequence with no duplicates, element at position i has exactly i elements
   less than it. This connects `rank` to `filter(...).len()`.

**If this is too much, complete Task 1 first and report progress on Task 2.**

## Validation

Run `scripts/validate.sh` after each change. Show full output. Fix all warnings.

## Report

Write `plans/agent4-round56-report.md` with holes before/after table including Chap column.
