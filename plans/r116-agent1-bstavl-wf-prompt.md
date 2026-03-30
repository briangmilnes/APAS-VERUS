# R116 Agent 1 — Strengthen BSTAVLMtEph wf to include tree_is_avl. AFK. DOT.

## Problem

BSTAVLMtEph's `spec_bstavlmteph_wf` is a stub — it only claims `tree_is_bst`,
not `tree_is_avl`. The StEph variant's wf IS `tree_is_avl` (which implies
`tree_is_bst`). The MtEph wrapper delegates to the same AVL insert/rebalance
logic, so it maintains AVL — it just doesn't say so.

This causes 3 real warnings in veracity-compare-par-mut:
- `new` ensures `tree_is_avl` in StEph, no match in MtEph
- `insert` requires `tree_is_avl` in StEph, no match in MtEph
- `insert` ensures `tree_is_avl` in StEph, no match in MtEph

## Current state

```rust
// BSTAVLStEph.rs:1065
open spec fn spec_bstavlsteph_wf(&self) -> bool { tree_is_avl::<T>(self.spec_root()) }

// BSTAVLMtEph.rs:497
open spec fn spec_bstavlmteph_wf(&self) -> bool {
    self@.tree_is_bst()
    && self@.spec_size() <= usize::MAX
    && self@.spec_height() <= usize::MAX
}
```

## Fix

1. Strengthen `spec_bstavlmteph_wf` to include `tree_is_avl`:
   ```rust
   open spec fn spec_bstavlmteph_wf(&self) -> bool {
       tree_is_avl::<T>(self@)
       && self@.spec_size() <= usize::MAX
       && self@.spec_height() <= usize::MAX
   }
   ```
   Since `tree_is_avl` implies `tree_is_bst`, the `tree_is_bst` conjunct
   can be dropped.

2. Update the RwLockPredicate inv (`BSTAVLMtEphInv`) to include `tree_is_avl`
   in its invariant. Find the inv struct and strengthen it to match the new wf.

3. Verify that `new` and `insert` can prove the strengthened ensures.
   The underlying AVL insert/rebalance functions already maintain AVL —
   the proofs should go through since the logic is there, it was just
   not claimed.

4. Check BSTSetAVLMtEph.rs — it uses BSTAVLMtEph and references
   `spec_bstavlmteph_wf`. Make sure the stronger wf doesn't break
   its callers (it shouldn't — stronger postconditions only help callers).

## Read first

- `src/Chap37/BSTAVLMtEph.rs` — full file
- `src/Chap37/BSTAVLStEph.rs` — the StEph wf and trait for reference
- `src/Chap37/BSTSetAVLMtEph.rs` — caller of BSTAVLMtEph

## Validate

Use `scripts/validate.sh isolate Chap37`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept.
- If `tree_is_avl` can't propagate through the RwLock pattern, stop and
  report why. Do NOT fall back to the stub wf.
- No subagents.

## STEP 20

## Report

Write `plans/agent1-r116-bstavl-wf-report.md`.
