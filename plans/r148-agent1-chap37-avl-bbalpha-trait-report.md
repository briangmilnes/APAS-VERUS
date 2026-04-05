# R148 Agent 1 Report: Traitify BST Node Functions in Chap37

## Summary

Moved ALL free functions operating on `BalBinTree<T>` into trait methods, following
the `BSTPlainNodeFns` pattern from `BSTPlainStEph.rs`.

## Changes by file

| # | Chap | File | Fns moved to trait | Free fns remaining | Trait name |
|---|------|------|--------------------|-------------------|------------|
| 1 | 37 | BSTBBAlphaStEph.rs | 7 | 0 | BSTBBAlphaNodeFns |
| 2 | 37 | BSTBBAlphaMtEph.rs | 7 | 0 | BSTBBAlphaMtNodeFns |
| 3 | 37 | BSTAVLStEph.rs | 8 | 0 | BSTAVLNodeFns |
| 4 | 37 | BSTAVLMtEph.rs | 8 | 0 | BSTAVLMtNodeFns |
| 5 | 37 | BSTSetPlainMtEph.rs | 1 | 3 | (added to existing trait) |
| 6 | 37 | BSTSetBBAlphaMtEph.rs | 1 | 3 | (added to existing trait) |
| 7 | 37 | BSTSetAVLMtEph.rs | 1 | 3 | (added to existing trait) |

**Total: 33 functions moved to traits. 0 free functions with tree-type first param remain.**

The 9 remaining free functions in BSTSet files take `&BSTPlainMtEph<T>`, `Vec<T>`, etc.
as first param — not the tree type.

## AVL trait technique

AVL rotation/rebalance ensures reference `avl_balanced()` and pattern match on
`BalBinTree::Node(...)`, which can't appear in an abstract trait `Self`. Solution:
add spec accessor methods to the trait:

```rust
spec fn avl_balanced_spec(self) -> bool;
spec fn tree_is_avl_spec(self) -> bool;
spec fn spec_left(self) -> Self;
spec fn spec_right(self) -> Self;
```

The impl defines them concretely (e.g., `open spec fn spec_left(self) -> Self { match self { ... } }`).
Trait ensures use `self.spec_left().spec_height()` instead of `match self { Node(n) => n.left.spec_height() }`.
Since the spec methods are `open`, Verus unfolds them in the impl and the proofs go through unchanged.

## Conjunction flakiness fix

`lemma_bst_deep` in BSTAVLMtEph.rs hit Z3 conjunction flakiness after restructuring.
Fixed by adding intermediate assertions for `tree_is_bst()` on children and grandchildren.

## Verification

- Full validate: 5702 verified, 0 errors
- RTT: 3690 passed
- PTT: 221 passed (from R148 run)
