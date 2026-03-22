<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 2 — Round 57 Prompt

## Branch

Work on `agent2/ready`. Push when done.

## DO NOT TOUCH

- Chap47 (any file)
- Chap43 (any file — Agent 1)
- Chap05 (any file — Agent 3)
- Chap42 (any file — Agent 3)
- Chap62, Chap63, Chap64 (any file — Agent 4)
- Any file in any other chapter not listed in your assignment

## Assignment: Close 5 capacity assumes in Chap41 + Chap45

### Task 1: AVLTreeSetStEph.rs — union capacity (1 hole)

**File:** `src/Chap41/AVLTreeSetStEph.rs`
**Line ~797:** `assume(combined@.len() + 1 < usize::MAX as nat)`

The `union` method iterates over `other` and inserts each element into `combined`.

**Fix:**
1. Add `requires self@.len() + other@.len() < usize::MAX as nat` to `union`
   in the trait.
2. Add loop invariant: `combined@.len() <= self@.len() + j as nat` (union
   deduplicates, never grows beyond `self@.len() + other@.len()`).
3. With the requires: `combined@.len() + 1 <= self@.len() + j + 1
   <= self@.len() + other@.len() < usize::MAX`.
4. Delete the `assume`.

Check all callers of `union` and propagate the new requires if needed.

### Task 2: AVLTreeSetStPer.rs — union capacity (1 hole)

**File:** `src/Chap41/AVLTreeSetStPer.rs`
**Line ~631:** `assume(combined@.len() + 1 < usize::MAX as nat)`

Same pattern as Task 1. Same fix. Add requires + loop invariant.

### Task 3: AVLTreeSetStPer.rs — insert capacity (1 hole)

**File:** `src/Chap41/AVLTreeSetStPer.rs`
**Line ~1018:** `assume(new_vec@.len() < usize::MAX)`

The `insert` method builds a new vec from existing elements + the new one.
The wf chain gives `self.elements@.len() < usize::MAX`. After inserting one
element: `new_vec@.len() <= self.elements@.len() + 1`. Need
`new_vec@.len() < usize::MAX`, so add
`requires self@.len() + 1 < usize::MAX as nat` to `insert` in the trait.
Then delete the assume.

### Task 4: BalancedTreePQ.rs — insert capacity (1 hole)

**File:** `src/Chap45/BalancedTreePQ.rs`
**Line ~253:** `assume(vals@.len() < usize::MAX)`

The new vec has `self.elements@.len() + 1` elements. Add
`requires self@.len() + 1 < usize::MAX as nat` to `insert`.

### Task 5: BalancedTreePQ.rs — meld capacity (1 hole)

**File:** `src/Chap45/BalancedTreePQ.rs`
**Line ~363:** `assume(values@.len() < usize::MAX)`

The merged vec has `self@.len() + other@.len()` elements. Add
`requires self@.len() + other@.len() < usize::MAX as nat` to `meld`.

Check callers of each modified function (especially `HeapsortExample.rs` for
BalancedTreePQ callers) and propagate requires.

## Validation

Run `scripts/validate.sh` after each fix. Show full output. Fix all warnings and errors.

## Report

Write `plans/agent2-round57-report.md` with holes before/after table including Chap column.
