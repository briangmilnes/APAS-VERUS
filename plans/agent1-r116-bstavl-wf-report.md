# Agent 1 — R116 Report: Strengthen BSTAVLMtEph wf to include tree_is_avl

## Summary

Strengthened `spec_bstavlmteph_wf` from `tree_is_bst` to `tree_is_avl` (which
implies `tree_is_bst`). This required upgrading the entire insert path to
maintain AVL balance through rebalancing.

## What changed

**`src/Chap37/BSTAVLMtEph.rs`** — all changes inside this one file:

1. **`spec_bstavlmteph_wf`**: `tree_is_bst(self@)` → `tree_is_avl::<T>(self@)`.
   Applied to: trait impl (line 785), type_invariant `wf` (line 722).

2. **`BSTAVLMtEphInv::inv`**: Strengthened lock predicate from `tree_is_bst` to
   `tree_is_avl`.

3. **`rotate_right` / `rotate_left`**: Upgraded ensures to include `spec_size`
   preservation, `spec_height` structural facts, and `avl_balanced` conditional
   preservation. Added proof assertions for heights, avl_balanced equivalences,
   and size arithmetic. These detailed postconditions are required by `rebalance`.

4. **`rebalance`** (new function): AVL rebalance after insertion. Checks balance
   factor, performs single or double rotations. 4 cases: left-right, left-left,
   right-left, right-right. Ensures `avl_balanced`, `tree_is_bst`, `spec_size`
   and `spec_height` preservation, `tree_contains` preservation.

5. **`lemma_max_plus_one`** (new proof fn): Helper for height arithmetic in
   insert.

6. **`insert_node`**: Upgraded from plain BST insert to AVL insert:
   - Requires `tree_is_avl` (was `tree_is_bst`)
   - Requires `spec_height <= usize::MAX - 1`
   - Ensures `tree_is_avl` (was `tree_is_bst`)
   - Calls `rebalance(r)` in Less and Greater branches
   - Added avl_balanced and height proof assertions

## Verification

| Metric | Value |
|--------|-------|
| Verified (isolate Chap37) | 1798 |
| Errors | 0 |
| RTT | 3529 passed |
| PTT | Pre-existing failures (Chap65 staged changes) |

Full validation OOM'd due to pre-existing Chap65 issues in the worktree, not
related to this change.

## Holes

No new holes introduced. BSTAVLMtEph.rs holes unchanged:
- 6 RWLOCK_GHOST structural assumes (insert ghost sync, reader assumes)
- 2 algorithmic assumes in `find` (pre-existing)

## Impact on callers

`BSTSetAVLMtEph.rs` uses `spec_bstavlmteph_wf` — verified clean. Stronger wf
only helps callers (stronger postconditions, same preconditions).

## Technique

Copied proven rotation postconditions and rebalance function from
`BSTAVLStEph.rs` into `BSTAVLMtEph.rs` (Mt standalone rule requires local
definitions). The proof structures are identical since both operate on the same
`BalBinTree<T>` type.
