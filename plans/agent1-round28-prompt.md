# Agent 1 — R28: Chap37 fn_missing_requires Sweep

## State

Main at latest commit. 4114 verified, 0 errors. You are Agent 1.

## Assignment

Fix all 53 `fn_missing_requires` warnings in Chap37. These are exec functions that have
`ensures` but no `requires`. Add the **real** precondition — not `requires true`.

## Rules

- **DO NOT add `requires true`.** That is a vacuous precondition and is worse than the warning.
- Read the function body to understand what it actually needs.
- Run `scripts/validate.sh` after every batch of changes. 0 errors required.
- Skip Example files.
- Do NOT touch files outside Chap37.

## File-by-File Targets

### BSTRBMtEph.rs (11 warnings)

| Line | Function | Likely requires |
|------|----------|----------------|
| 117 | new_node | None needed — constructor. But it has ensures, so add a simple `requires` like the value bound if any, or just the trait bounds suffice. If truly no precondition, the function should not have `ensures` without `requires`. Check if the ensures is on a trait method — if so, the trait dictates. |
| 134 | is_red | None needed — structural accessor on Option. |
| 145 | size_link | None needed — structural accessor on Option. |
| 156 | update | None needed — structural accessor. |
| 659 | in_order_collect | Needs `spec_is_bst_link(link)` or just well-formedness on the link. |
| 671 | pre_order_collect | Same as in_order_collect. |
| 683 | in_order_parallel | Same. |
| 704 | pre_order_parallel | Same. |
| 725 | build_balanced | Needs input vec constraints. |
| 754 | filter_parallel | Needs `spec_is_bst_link` on input + closure specs. |
| 783 | reduce_parallel | Needs `spec_is_bst_link` on input + closure specs. |

**Pattern**: For structural accessors (new_node, is_red, size_link, update), look at what
the function actually does. If it genuinely has no precondition and the ensures is trivially
true from the body, the real fix may be that these functions shouldn't have ensures either.
But veracity flags `fn_missing_requires` only when ensures exists without requires. So either:
(a) add a real requires, or (b) if the ensures is vacuous too, remove it. Prefer (a).

For traversal/parallel functions, the real precondition is typically the BST ordering
invariant on the input link.

### BSTSplayMtEph.rs (10 warnings)

| Line | Function | Likely requires |
|------|----------|----------------|
| 110 | new_node | Same pattern as BSTRBMtEph. |
| 126 | size_link | Structural accessor. |
| 137 | update | Structural accessor. |
| 1415 | in_order_collect | Needs BST invariant on link. |
| 1427 | pre_order_collect | Same. |
| 1439 | in_order_parallel | Same. |
| 1460 | pre_order_parallel | Same. |

Plus 3 more from the grep — check all fn_missing_requires in the file.

### BSTSplayStEph.rs — not listed in current holes. Double-check with `scripts/holes.sh src/Chap37/BSTSplayStEph.rs`.

### AVLTreeSeqStPer.rs (5 warnings)

| Line | Function | Likely requires |
|------|----------|----------------|
| 230 | height_fn | Structural accessor on node. |
| 239 | size_fn | Structural accessor on node. |
| 484 | inorder_collect | Needs well-formedness on link/tree. |
| 495 | build_balanced_from_slice | Needs slice bounds. |
| 727 | push_left_iter_stper | Iterator helper — needs valid tree state. |

### AVLTreeSeqMtPer.rs (4 warnings)

| Line | Function | Likely requires |
|------|----------|----------------|
| 259 | height_fn | Structural accessor. |
| 268 | size_fn | Structural accessor. |
| 495 | inorder_collect | Needs well-formedness. |
| 512 | rec (fn_missing_requires_ensures) | Needs both requires and ensures. |

### BSTSetSplayMtEph.rs (3), BSTSetPlainMtEph.rs (3), BSTSetBBAlphaMtEph.rs (3), BSTSetAVLMtEph.rs (3), BSTSetRBMtEph.rs (2)

All follow the same pattern — `values_vec`, `rebuild_from_vec`, `from_sorted_iter`:

- `values_vec`: requires `self.spec_<module>_wf()` (the module's wf predicate).
- `rebuild_from_vec`: requires input vec is sorted / valid.
- `from_sorted_iter`: requires iterator yields sorted values.

Look at the StEph counterpart for each to see what requires it has, and copy that pattern.

### AVLTreeSeqStEph.rs (3 warnings)

| Line | Function | Likely requires |
|------|----------|----------------|
| 327 | h_fn | Structural accessor. |
| 674 | clone_link | Structural accessor on Option<Box<Node>>. |
| 1039 | push_left_iter | Iterator helper — needs valid tree. |

### AVLTreeSeq.rs (1 warning)

| Line | Function | Likely requires |
|------|----------|----------------|
| 345 | cached_height | Structural accessor. |

## How To Fix

For each function:
1. Read the function body and its `ensures`.
2. Determine what the function actually needs from its inputs.
3. Add `requires` with the real precondition.
4. If it's a structural accessor that truly has no precondition, the minimum real
   requires is typically just the type bounds — but check if the ensures mentions
   `self@` or similar spec functions that need wf to be meaningful.

## Deliverable

- `scripts/validate.sh` passes with 0 errors.
- All 53 fn_missing_requires in Chap37 resolved.
- Write report to `plans/agent1-round28-report.md`.
- `git add -A && git commit` with descriptive message.
- `git push origin agent1/ready`.
