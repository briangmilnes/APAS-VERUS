# R148 Agent 1 Report: Traitify BST Node Functions in Chap37

## Summary

Moved free functions operating on `BalBinTree<T>` into trait methods, following
the `BSTPlainNodeFns` pattern from `BSTPlainStEph.rs`.

## Changes by file

| # | Chap | File | Fns moved to trait | Fns kept as free | Trait name |
|---|------|------|--------------------|------------------|------------|
| 1 | 37 | BSTBBAlphaStEph.rs | 7 | 0 | BSTBBAlphaNodeFns |
| 2 | 37 | BSTBBAlphaMtEph.rs | 7 | 0 | BSTBBAlphaMtNodeFns |
| 3 | 37 | BSTAVLStEph.rs | 4 | 4 | BSTAVLNodeFns |
| 4 | 37 | BSTAVLMtEph.rs | 4 | 4 | BSTAVLMtNodeFns |
| 5 | 37 | BSTSetPlainMtEph.rs | 1 | 3 | (added to existing trait) |
| 6 | 37 | BSTSetBBAlphaMtEph.rs | 1 | 3 | (added to existing trait) |
| 7 | 37 | BSTSetAVLMtEph.rs | 1 | 3 | (added to existing trait) |

## BBAlpha files (full traitification)

All 7 node-level functions moved to trait: `insert_node`, `contains_node`,
`find_node`, `min_node`, `max_node`, `delete_min_node`, `delete_node`.

Their requires/ensures only reference spec functions from `BSTSpecFns` and
`BalBinTreeTrait`, so the trait abstraction is clean.

## AVL files (partial traitification)

4 of 8 functions moved to trait: `contains_node`, `find_node`, `min_node`,
`max_node`.

4 functions remain as free functions: `rotate_right`, `rotate_left`,
`rebalance`, `insert_node`.

**Why partial**: The AVL `insert_node` requires `tree_is_avl(self)` and ensures
`tree_is_avl(inserted)`, where `tree_is_avl` and `avl_balanced` are module-level
spec functions taking `BalBinTree<T>` by value. These cannot appear in a trait's
requires/ensures because the trait's `Self` is abstract — it doesn't know it's
`BalBinTree<T>`. The rotation functions have `match self { BalBinTree::Node(...) }`
in their ensures, which requires concrete type knowledge.

The search functions (contains, find, min, max) only use `tree_is_bst()` and
`tree_contains()` from `BSTSpecFns`, so they traitify cleanly.

## BSTSet files

Moved `copy_set` from free function to trait method (`&self` → `self.copy_set()`).
The other 3 free functions (`values_vec`, `rebuild_from_vec`, `from_vec`) don't
take the set type as first arg, so they remain as helpers.

## Pattern applied

For each function:
- `node: BalBinTree<T>` (consuming) → `self`, add `let ghost node = self;`
- `node: &BalBinTree<T>` (borrowing) → `&self`
- Recursive calls: `insert_node(left, value)` → `left.insert_node(value)`
- Wrapper calls: `contains_node(&self.root, target)` → `self.root.contains_node(target)`
- Trait impls only declare `decreases` (not requires/ensures, which come from trait)

## Import fix

`BSTSpecFns` imports in BSTBBAlphaStEph.rs and BSTAVLStEph.rs were gated behind
`#[cfg(verus_keep_ghost)]`. Since the trait bounds reference `BSTSpecFns`, the
import must be unconditional. Removed the cfg gate.

## Verification

- Full validate: 5702 verified, 0 errors
- RTT: 3690 passed
- PTT: 221 passed
- 4 pre-existing `==>` vs `implies` warnings (not from this change)
