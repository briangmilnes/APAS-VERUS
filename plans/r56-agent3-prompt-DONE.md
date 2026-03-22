<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 3 — Round 56 Prompt

## Branch

Work on `agent3/ready`. Base: `fd7f38bb4`.

## DO NOT TOUCH

- Chap47 (any file)
- Chap43/OrderedTableStPer.rs (Agent 2)
- Chap45 (any file — Agent 4)

## Assignment: Close 3 capacity holes in Chap41

### Hole 1: AVLTreeSetStEph.rs — union capacity (line 797)

**File:** `src/Chap41/AVLTreeSetStEph.rs`
**Hole:** `assume(combined@.len() + 1 < usize::MAX as nat)` in `union`

The `union` method iterates over `other` and inserts each element into `combined`.

**Fix:** Add a `requires` to `union` in the trait:
```
self@.len() + other@.len() < usize::MAX as nat,
```

Then add loop invariant: `combined@.len() <= self@.len() + j as nat` (union can only
deduplicate, never grow beyond `self@.len() + other@.len()`). With the requires,
`combined@.len() + 1 <= self@.len() + j + 1 <= self@.len() + other@.len() < usize::MAX`.

Check all callers (MtEph, OrderedSetStEph, OrderedSetMtEph, etc.) and propagate the
requires clause. Some callers may need their own capacity requires added.

### Hole 2: AVLTreeSetStPer.rs — union capacity (line 631)

**File:** `src/Chap41/AVLTreeSetStPer.rs`
**Hole:** `assume(combined@.len() + 1 < usize::MAX as nat)` in `union`

Same pattern as StEph. Add `self@.len() + other@.len() < usize::MAX as nat` to requires.
Same loop invariant approach.

### Hole 3: AVLTreeSetStPer.rs — insert capacity (line 1018)

**File:** `src/Chap41/AVLTreeSetStPer.rs`
**Hole:** `assume(new_vec@.len() < usize::MAX)` in `insert`

The `insert` method builds a new vec from the existing elements plus the new one.
The wf chain guarantees `self.elements@.len() < usize::MAX` (from R55). After inserting
one element: `new_vec@.len() <= self.elements@.len() + 1`. We need
`new_vec@.len() < usize::MAX`, which requires `self.elements@.len() + 1 < usize::MAX`,
i.e., `self.elements@.len() < usize::MAX - 1`.

This is slightly tighter than what wf gives (`< usize::MAX`). Options:
(a) Add `self@.len() + 1 < usize::MAX as nat` to insert's requires
(b) Tighten the wf bound to `< usize::MAX - 1`

Option (a) is simpler. Add the requires and propagate to callers.

## Validation

Run `scripts/validate.sh` after each fix. Show full output. Fix all warnings.

## Report

Write `plans/agent3-round56-report.md` with holes before/after table including Chap column.
