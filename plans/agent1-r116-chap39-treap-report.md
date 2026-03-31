# R116 Agent 1 — BSTTreapMtEph Trait Lifting Report

## Summary

Lifted internal algorithms, specs, and proof lemmas into `BSTTreapMtEphTrait`
and `LinkTrait` for Chap39/BSTTreapMtEph.rs. Added 4 link-level spec functions,
2 self-level spec functions, 8 proof lemma declarations, and 5 read-only exec
function declarations to the traits.

## Verus Limitation Discovered

Moving `&mut`-taking functions that call `Option::take()` into trait methods
triggers a Verus internal error: "generated ill-typed AIR code" in the ensures
of `core::option::Option::take()`. This affects `rotate_left`, `rotate_right`,
`insert_link`, and `delete_link` — all use `link.take()` or `node.left.take()`.

**Workaround**: Keep the 4 take()-using functions as free functions. All other
functions (specs, proof lemmas, read-only exec fns) are in the trait with
delegating implementations.

## Changes

### LinkTrait — 4 new spec functions

| # | Chap | File | Spec Function |
|---|------|------|--------------|
| 1 | 39 | BSTTreapMtEph.rs | `spec_in_order_link` |
| 2 | 39 | BSTTreapMtEph.rs | `spec_pre_order_link` |
| 3 | 39 | BSTTreapMtEph.rs | `spec_min_link` |
| 4 | 39 | BSTTreapMtEph.rs | `spec_max_link` |

### BSTTreapMtEphTrait — new declarations

| # | Chap | File | Kind | Function |
|---|------|------|------|----------|
| 1 | 39 | BSTTreapMtEph.rs | spec | `spec_size` |
| 2 | 39 | BSTTreapMtEph.rs | spec | `spec_contains` |
| 3 | 39 | BSTTreapMtEph.rs | proof | `lemma_bst_decompose` |
| 4 | 39 | BSTTreapMtEph.rs | proof | `lemma_contains_left` |
| 5 | 39 | BSTTreapMtEph.rs | proof | `lemma_contains_right` |
| 6 | 39 | BSTTreapMtEph.rs | proof | `lemma_contains_root` |
| 7 | 39 | BSTTreapMtEph.rs | proof | `lemma_height_le_size` |
| 8 | 39 | BSTTreapMtEph.rs | proof | `lemma_size_wf_child_bounded` |
| 9 | 39 | BSTTreapMtEph.rs | proof | `lemma_wf_decompose` (new) |
| 10 | 39 | BSTTreapMtEph.rs | proof | `lemma_wf_assemble_node` |
| 11 | 39 | BSTTreapMtEph.rs | exec | `size_link` |
| 12 | 39 | BSTTreapMtEph.rs | exec | `find_link` |
| 13 | 39 | BSTTreapMtEph.rs | exec | `min_link` |
| 14 | 39 | BSTTreapMtEph.rs | exec | `max_link` |
| 15 | 39 | BSTTreapMtEph.rs | exec | `height_link` |

### Spec Strengthening

| # | Chap | File | Function | Change |
|---|------|------|----------|--------|
| 1 | 39 | BSTTreapMtEph.rs | `min_link` | Added `match (min_val, Lnk::spec_min_link(link))` ensures |
| 2 | 39 | BSTTreapMtEph.rs | `max_link` | Added `match (max_val, Lnk::spec_max_link(link))` ensures |

### Functions NOT moved (Verus take() bug)

| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 1 | 39 | BSTTreapMtEph.rs | `update` | No take(), but kept free for consistency with callers |
| 2 | 39 | BSTTreapMtEph.rs | `rotate_left` | Uses `link.take()`, `x.right.take()`, `y.left.take()` |
| 3 | 39 | BSTTreapMtEph.rs | `rotate_right` | Uses `link.take()`, `x.left.take()`, `y.right.take()` |
| 4 | 39 | BSTTreapMtEph.rs | `insert_link` | Uses `link.take()` |
| 5 | 39 | BSTTreapMtEph.rs | `delete_link` | Uses `link.take()`, `node.left.take()`, `node.right.take()` |
| 6 | 39 | BSTTreapMtEph.rs | `in_order_collect` | Kept free for consistency |
| 7 | 39 | BSTTreapMtEph.rs | `pre_order_collect` | Kept free for consistency |

## Verification

| Metric | Before | After |
|--------|--------|-------|
| Verified | 1186 | 1203 |
| Errors | 0 | 0 |
| Proof holes | 0 | 0 |
| RTTs | 3529 | 3529 |
| veracity-compare-par-mut warnings | 24 | 24 |
| veracity-compare-par-mut info | 19 | 45 |

The warning count is unchanged (24). Info messages increased because more trait
methods are now being compared between StEph and MtEph. The 2 warnings from
newly-added min_link/max_link were eliminated by strengthening their ensures to
match StEph's pattern.

## Why public specs weren't further strengthened

Many of StEph's public ensures reference `spec_bst()`, `spec_height()`,
`spec_min()`, `spec_max()`, `spec_in_order()`, `spec_pre_order()`. These are
structural properties of the tree that cannot be defined on the MtEph type
because the tree is behind an `RwLock`. The BST property is enforced by the
`BSTTreapMtEphInv` RwLock predicate, but it's not expressible as a spec on
`&self` (which only has access to the ghost set view).

The MtEph ensures `self@ =~= old(self)@.insert(value@)` for `insert` (and
similarly for `delete`) are actually STRONGER than StEph's individual ensures
clauses, since set extensionality subsumes containment preservation, size
bounds, etc. The veracity tool counts clause-by-clause mismatches and doesn't
recognize this subsumption.
