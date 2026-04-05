# R148 Agent 4 — Traitify AVLTreeSeq Link Functions in Chap37

## Summary

Moved free Link<T> functions into trait impls across all 4 AVLTreeSeq files
in Chap37. Node-level functions (rotations, rebalance, update_meta/mk) remain
as free functions because their contracts reference struct fields extensively,
making trait abstraction impractical.

## Approach

Following the BSTPlainNodeFns pattern from R147, but adapted for AVLTreeSeq's
type structure:

1. **Spec supertrait**: Each file gets a `*LinkSpec<T>` trait with 4 open spec
   methods (`link_wf`, `link_inorder`, `link_cached_size`, `link_cached_height`)
   that wrap the existing free spec functions. This enables abstract `Self` in
   trait contracts.

2. **Link trait**: Each file gets a `*LinkFns<T>` trait with `Sized + *LinkSpec<T>`
   as supertraits. Contracts use the spec methods.

3. **Link trait impl**: `impl<T: StT> *LinkFns<T> for Link<T>` moves the free
   function bodies, using `self` instead of the old parameter name.

4. **Node functions stay free**: update_meta, rotate_right/left, rebalance (and mk
   for StPer/MtPer) remain as free functions. Their contracts reference
   Box/Arc node fields directly, which an abstract `Self` cannot access.

## Key difference from BSTPlainNodeFns

BSTPlain used a custom enum (`BalBinTree<T>`) with spec traits already defined
(`BSTSpecFns`, `BalBinTreeTrait`). AVLTreeSeq uses `Link<T> = Option<Box/Arc<Node<T>>>`
(a type alias on stdlib types) with free spec functions. The spec supertrait bridges
this gap.

## Files Changed

| # | Chap | File | Link Fns | Node Fns (free) | Spec Trait |
|---|------|------|----------|------------------|------------|
| 1 | 37 | AVLTreeSeq.rs | 7 (cached_height_fn, cached_size_fn, insert_at_link, nth_link, set_link, push_inorder, compare_trees) | 4 (update_size_height, rotate_right, rotate_left, rebalance) | AVLTreeSeqLinkSpec + AVLTreeSeqLinkFns |
| 2 | 37 | AVLTreeSeqStEph.rs | 7 (h_fn, size_link_fn, insert_at_link, nth_link, set_link, compare_trees, clone_link) | 4 (update_meta, rotate_right_fn, rotate_left_fn, rebalance_fn) | AVLTreeSeqStEphLinkSpec + AVLTreeSeqStEphLinkFns |
| 3 | 37 | AVLTreeSeqStPer.rs | 6 (height_fn, size_fn, nth_ref, set_rec, inorder_collect, compare_trees) | 4 (mk, rotate_right, rotate_left, rebalance) + build_balanced_from_slice | AVLTreeSeqStPerLinkSpec + AVLTreeSeqStPerLinkFns |
| 4 | 37 | AVLTreeSeqMtPer.rs | 6 (height_fn, size_fn, nth_ref, set_rec, inorder_collect, compare_trees) | 4 (mk, rotate_right, rotate_left, rebalance) + build_balanced_from_slice | AVLTreeSeqMtPerLinkSpec + AVLTreeSeqMtPerLinkFns |

## R148b — Node functions moved to traits

All rotate/rebalance/update_meta functions now also live in trait impls
(`*NodeSpec` + `*NodeFns` traits on `Box<AVLTreeNode<T>>` or `Arc<Node<T>>`).

Remaining true free functions (first param is NOT a tree type):
- `mk` (StPer, MtPer) — constructor, first param is `value: T`
- `build_balanced_from_slice` (StPer, MtPer) — first param is `&[T]`
- `push_left_iter` / `push_left_iter_stper` (StEph, StPer) — first param is `&mut Iterator`

Free function counts per file (excluding trait/spec/proof/derive methods):
- AVLTreeSeq.rs: 0 free (11 trait methods)
- AVLTreeSeqStEph.rs: 1 free (push_left_iter — iterator helper, first param is &mut Iterator)
- AVLTreeSeqStPer.rs: 3 free (mk, build_balanced_from_slice, push_left_iter_stper)
- AVLTreeSeqMtPer.rs: 2 free (mk, build_balanced_from_slice)

## Verification

- **validate**: 5702 verified, 0 errors
- **RTT**: 3690 passed
- **PTT**: 221 passed
