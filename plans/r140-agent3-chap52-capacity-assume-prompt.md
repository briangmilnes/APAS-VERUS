# R140 Agent 3 — Eliminate capacity assume in AdjTableGraphMtPer. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `capacity_bounds_standard.rs`

Read `src/Chap52/AdjTableGraphMtPer.rs` — the file with the assume.
Read `src/Chap41/AVLTreeSetMtPer.rs` — the type stored as neighbor sets.
Read `src/Chap38/BSTParaMtEph.rs` — the backing BST with type_invariant.

Report file: `plans/r140-agent3-chap52-capacity-report.md`

## Problem

`src/Chap52/AdjTableGraphMtPer.rs:472` has:
```rust
assume(neighbors@.len() < usize::MAX as nat);
```

This is inside a `map` closure in `delete_vertex`. The closure calls
`neighbors.delete(&v_clone)` which requires `self@.len() < usize::MAX`.

In R139, you proved `neighbors@.len() <= usize::MAX` from ParamBST's `size()`
returning usize, but couldn't close the gap to strict `<`.

## Investigation: Why does delete need capacity?

First, check whether `AVLTreeSetMtPer::delete` actually NEEDS `self@.len() < usize::MAX`.
Agent2 (R138) added this requires when converting assumes to requires. But `delete`
REMOVES an element — it should make the set smaller, not larger.

Read the `delete` implementation in `src/Chap41/AVLTreeSetMtPer.rs`. Check:
1. Does the BST `delete` operation call any function that needs capacity?
2. Does the rebalancing (AVL rotations) create intermediate nodes that could overflow?
3. Is the capacity requires actually necessary, or was it added defensively?

If `delete` doesn't actually need `self@.len() < usize::MAX`, the fix is to
REMOVE the requires from delete (not add it to the caller). This would eliminate
the assume entirely.

## If delete genuinely needs capacity

If after reading the code you confirm delete needs the capacity bound, try these
approaches:

**Approach A**: The graph's wf could guarantee all stored neighbor sets have
`@.len() < usize::MAX`. If `add_edge` checks capacity before inserting, then
all stored sets are bounded. Propagate through the graph's spec_wf predicate.

**Approach B**: If ParamBST's type_invariant can be strengthened to prove
`@.len() < usize::MAX` (not just `<=`), that would close the gap. Check whether
the BST's `NodeInner.size: usize` field is always < usize::MAX (it starts at 0
for empty, increments by 1 per insert — it can reach usize::MAX but only after
usize::MAX inserts, at which point we'd have OOMed long ago).

**Approach C**: If the set is non-empty (it contains at least vertex v which is
being deleted), then `@.len() >= 1`, so `@.len() <= usize::MAX` implies
`@.len() - 1 < usize::MAX`. But delete needs the PRE-delete size < usize::MAX,
not post-delete.

## Validation

Run `scripts/validate.sh isolate Chap52`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Prefer removing unnecessary requires over adding workarounds.
- If the requires is unnecessary, remove it from the trait AND impl.

## When done

RCP.
