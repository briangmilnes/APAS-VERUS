# Agent 1 Round 166 Report

## Summary

Lifted within-module proof patterns from BSTPlainStEph and BSTPlainMtEph into
shared lemmas in BSTSpecsAndLemmas. Net -193 lines across 3 files.

Splay files were in scope but not touched this round -- the Plain files had the
most extractable duplication.

## Lemmas extracted to BSTSpecsAndLemmas.rs

| # | Lemma | Purpose |
|---|-------|---------|
| 1 | `lemma_node_contains` | Unfolds `tree_contains` one level for Node(left, val, right) |
| 2 | `lemma_bst_left` | From BST + left contains x, derives le(x, val) and x != val |
| 3 | `lemma_bst_right` | From BST + right contains x, derives le(val, x) and x != val |
| 4 | `lemma_bst_left_subtree_ordering` | Left subtree ordering preserved through replacement |
| 5 | `lemma_bst_right_subtree_ordering` | Right subtree ordering preserved through replacement |

Lemmas 1-3 replaced ~40 repeated inline proof blocks across insert, delete, and
delete_min in both St and Mt files. Lemmas 4-5 replaced repeated subtree ordering
reestablishment proofs after rotations/restructuring.

## Files changed

| # | Chap | File | Lines before | Lines after | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 37 | BSTPlainStEph.rs | ~870 | ~663 | -207 |
| 2 | 37 | BSTPlainMtEph.rs | ~610 | ~480 | -130 |
| 3 | 37 | BSTSpecsAndLemmas.rs | ~320 | ~389 | +69 |

Net: -193 lines (107 added, 300 removed).

## Verification

- Full validate: 5586 verified, 0 errors
- RTT: 3776 passed
- PTT: 221 passed
- Trigger notes: 0 (fixed 2 in BSTSpecsAndLemmas.rs)

## Additional fix

Added `#[cfg(verus_keep_ghost)]` gate on the `pub use` re-export of proof lemmas
from BSTPlainStEph (needed for cargo RTT compilation since proof fns are invisible
to cargo).
