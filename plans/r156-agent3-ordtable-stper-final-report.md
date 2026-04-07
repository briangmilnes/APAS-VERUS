# R156 Agent 3 Report: Final OrderedTableStPer Delegation

## Summary

Replaced 8 loop-based implementations in `src/Chap43/OrderedTableStPer.rs` with simple
delegations to the new `OrdKeyMap` methods added in R155. File reduced from 3,442 lines
to 2,295 lines (−1,147 lines, −33%).

## Holes Before / After

No new proof holes introduced. No assumes, accepts, or external_body added.

## File Changes

| # | Chap | File | Lines Before | Lines After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 43 | OrderedTableStPer.rs | 3,442 | 2,295 | −1,147 |

## Delegations Performed

| # | Method | Old Lines | Delegation | Notes |
|---|--------|-----------|-----------|-------|
| 1 | `intersection` | ~162 | `self.tree.intersect_with(&other.tree, &f)` | Verified cleanly |
| 2 | `union` | ~371 | `self.tree.union_with(&other.tree, &f)` | Required proof bridge |
| 3 | `difference` | ~108 | `self.tree.difference(&other.tree)` | Verified cleanly |
| 4 | `first_key_iter` | ~40 | `self.tree.first_key()` | Verified cleanly |
| 5 | `last_key_iter` | ~40 | `self.tree.last_key()` | Verified cleanly |
| 6 | `split_key_iter` | ~218 | `self.tree.split(k)` + wrap | Added `lemma_pair_set_to_map_dom_finite` |
| 7 | `get_key_range_iter` | ~91 | `self.tree.get_key_range(k1, k2)` | Verified cleanly |
| 8 | `split_rank_key_iter` | ~178 | clone inner BST + `split_rank_key` | Clone needed for `&mut` |

## Proof Techniques

**`union` trigger bridge**: OrdKeyMap `union_with` triggers its combine ensures on
`combined@[k] == r@` (inside the existential), while the StPer trait triggers on
`self@.contains_key(k) && other@.contains_key(k)` (the hypothesis). Z3 cannot fire the
OrdKeyMap forall from the hypothesis alone. Fix: in a `proof { assert forall|k| hyp implies
exists ... by { let vk = result@[k]; assert(result@[k] == vk); } }` block, binding
`result@[k]` to a fresh spec variable materializes it as a ground term in Z3's context.
This matches the trigger pattern `combined@[k] == r@` (with `r@ = vk`) and fires the forall.

**`split_key_iter` finiteness**: The trait ensures `self@.dom().finite()`. Added
`proof { lemma_pair_set_to_map_dom_finite(self.tree.inner@); }` before the split call.

**`split_rank_key_iter` clone pattern**: OrdKeyMap's `split_rank_key` takes `&mut self`, so
persistent semantics require cloning:
```rust
let inner_copy = self.tree.inner.clone();
let mut tree_copy = OrdKeyMap { inner: inner_copy };
proof { assert(tree_copy.spec_ordkeymap_wf()); }
let (left, right) = tree_copy.split_rank_key(i);
```
The `spec_ordkeymap_wf` assertion on `tree_copy` holds because all conditions are either
type-level predicates or depend only on `inner@`, and `inner_copy@ == self.tree.inner@`
from `Clone::clone`'s ensures.

## Verification Results

| # | Step | Result |
|---|------|--------|
| 1 | `validate isolate Chap43` | 2,814 verified, 0 errors |
| 2 | `validate` (full) | 5,758 verified, 0 errors |
| 3 | `rtt` | 3,727 passed, 0 skipped |

## Constraints Observed

- `OrdKeyMap.rs` not modified
- `OrderedTableStEph.rs` not modified
- No assumes, accepts, or external_body added
- No ensures weakened
- All original proof lemmas and spec functions preserved
