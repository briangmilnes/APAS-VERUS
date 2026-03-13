# Agent 1 Report — Round 7

## Assignment

Prove holes in Chap37 AVLTreeSeq files. Five files:

| # | Chap | File | Before | Type |
|---|------|------|--------|------|
| 1 | 37 | AVLTreeSeq.rs | 3 | 1 assume, 2 external_body |
| 2 | 37 | AVLTreeSeqStEph.rs | 7 | 2 assume, 5 external_body |
| 3 | 37 | AVLTreeSeqStPer.rs | 13 | 1 assume, 12 external_body |
| 4 | 37 | AVLTreeSeqMtPer.rs | 12 | 2 assume, 10 external_body |
| 5 | 37 | BSTSplayStEph.rs | 2 | 1 trivial_spec_wf, 1 external_body |

## Approach

Arc-based persistent AVL trees (StPer, MtPer) had 12 external_body functions each
hiding the entire algorithm body from verification. Six core functions — mk, rotate_right,
rotate_left, rebalance, nth_ref, set_rec — are pure algorithmic logic that can be proven.

Key proof techniques developed:

1. **lemma_height_le_size**: For any wf tree, cached_height <= cached_size. This bridges
   mk's height precondition (`1 + max(h_left, h_right) <= N::MAX`) from the size bound
   already in wf. Added to AVLTreeSeq.rs, AVLTreeSeqStEph.rs, AVLTreeSeqStPer.rs,
   AVLTreeSeqMtPer.rs.

2. **mk field ensures**: Added `node.left == left, node.right == right` to mk's ensures.
   Callers need this to prove `rebuilt.left.is_some()` for the outer rotation in double
   rotation cases.

3. **Size preservation ensures**: Added `spec_cached_size(&Some(result)) == spec_cached_size(&Some(input))`
   to rotate_right, rotate_left, and rebalance. Required by set_rec to prove mk preconditions
   in recursive cases (new subtree has same size as original).

4. **Ghost size capture before moves**: After `let left = n.left.as_ref().unwrap().clone()`,
   Verus can relate `left` to `n.left`. But after `rotate_left(left)` consumes `left`,
   Verus loses the connection. Fix: `let ghost left_size = spec_cached_size(&Some(left))`
   before the move, then use `left_size` in later proofs.

5. **clone_plus + assume for value clones**: Persistent trees need `n.value.clone()` to
   create new nodes. `clone_plus()` gives `ensures cloned(*self, res)` but Verus can't
   derive `res@ == self@` without `obeys_feq_full`. Bridge: `assume(val@ == original@)`.
   These are narrow single-property assumes replacing external_body on entire functions.

6. **Recursive nth_ref**: Converted iterative loop to recursive with `decreases *cur`.
   Recursive form needs no loop invariants and Verus handles it natively.

## Results

| # | Chap | File | Before | After | ext_body removed | assumes added |
|---|------|------|--------|-------|------------------|---------------|
| 1 | 37 | AVLTreeSeq.rs | 3 | 2 | 0 | 0 (-1 assume) |
| 2 | 37 | AVLTreeSeqStEph.rs | 7 | 5 | 1 (PartialEq::eq) | 0 (-1 assume) |
| 3 | 37 | AVLTreeSeqStPer.rs | 13 | 15 | 6 | 8 (clone_plus) |
| 4 | 37 | AVLTreeSeqMtPer.rs | 12 | 14 | 6 (same 6) | 8 (clone_plus) |
| 5 | 37 | BSTSplayStEph.rs | 2 | 2 | 0 | 0 |
| | | **Total** | **37** | **38** | **13** | **16** |

Hole count increased by 1 net, but composition changed dramatically:
- **12 external_body eliminated**: Each hid an entire function body from verification.
  Now mk, rotate_right, rotate_left, rebalance, nth_ref, set_rec are fully proven
  in both StPer and MtPer.
- **16 clone_plus assumes added**: Each is a single property (`val@ == original@`).
  These are the standard persistent-tree bridge for value cloning through Arc.
- **2 algorithmic assumes eliminated**: insert_at_link height bound (AVLTreeSeq.rs),
  update_meta height bound (AVLTreeSeqStEph.rs).

## Remaining Holes

Functions NOT proven (staying as external_body):

| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 1 | 37 | AVLTreeSeqStPer.rs | inorder_collect | Vec content tracking through recursion |
| 2 | 37 | AVLTreeSeqStPer.rs | build_balanced_from_slice | Nested fn (Verus limitation) |
| 3 | 37 | AVLTreeSeqStPer.rs | compare_trees | PartialEq ensures for T needed |
| 4 | 37 | AVLTreeSeqStPer.rs | iter, push_left_iter, next | Iterator infrastructure |
| 5 | 37 | AVLTreeSeqMtPer.rs | inorder_collect | Same as StPer |
| 6 | 37 | AVLTreeSeqMtPer.rs | build_balanced_from_slice | Parallel (ParaPair!) — must stay |
| 7 | 37 | AVLTreeSeqMtPer.rs | compare_trees | Same as StPer |
| 8 | 37 | AVLTreeSeqMtPer.rs | subseq_copy | Parallel (spawn/wait) — must stay |

## Verification

- **Validated**: 3780 verified, 0 errors
- **RTT**: 2600 passed, 0 skipped
- **Commit**: `f891e2e4` on `agent1/ready`
