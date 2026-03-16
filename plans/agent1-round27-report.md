# Agent 1 — Round 27 Report

## Mission

Add real `requires`/`ensures` with proof annotations to all BST helper functions
in `BSTSplayMtEph.rs` (6 functions) and `BSTRBMtEph.rs` (8 functions). Strengthen
`spec_bstsplaymteph_wf` and `spec_bstrbmteph_wf` to include BST ordering invariant.

## R26 Feedback Applied

R26 defined `spec_is_bst_link` but left helper functions with no specs. This round
added full BST-preservation specs and proof bodies to all 14 helper functions. No
`assume`, `accept`, or `external_body` added.

## Results

- **Verification**: 4109 verified, 0 errors (up from 4103 in R26; +6 newly verified functions)
- **RTT**: 2613 passed
- **PTT**: 147 passed
- **Holes**: 217 total (unchanged — R27 added specs to already-body-verified functions)
- **Clean chapters**: 34 (unchanged)

## Per-File Changes

| # | Chap | File | Holes Before | Holes After | Functions Proved |
|---|------|------|-------------|-------------|-----------------|
| 1 | 37 | BSTSplayMtEph.rs | 0 | 0 | splay, bst_insert, insert_link, find_link, min_link, max_link |
| 2 | 37 | BSTRBMtEph.rs | 0 | 0 | rotate_left, rotate_right, flip_colors, fix_up, insert_link, find_link, min_link, max_link |

## Techniques

### Part 1: BSTSplayMtEph.rs (6 functions)

Mechanically translated proof annotations from `BSTSplayStEph.rs` using sed scripts
to adapt naming conventions:
- `spec_contains_link(&x)` → `link_contains(x)`
- `spec_is_bst_link(&x)` → `spec_is_bst_link(x)`
- `&mut` old-state: `spec_contains_link(old(link))` → `link_contains(*old(link))`

The 976-line `splay` function was translated and spliced in via bash head/cat/tail.
All 6 functions verified on first attempt.

### Part 2: BSTRBMtEph.rs (8 functions)

`BSTRBStEph.rs` uses `BalBinTree<T>` (discriminated enum Leaf/Node) — completely
different type from MtEph's `Link<T> = Option<Box<Node<T>>>`. Mechanical translation
was impossible. Wrote new proofs from scratch:

- **rotate_left/rotate_right**: Ghost captures of all subtree pieces before rotation.
  Two proof blocks: (1) extract BST ordering facts from original tree using
  `reveal_with_fuel`, (2) prove rotated tree satisfies BST ordering and containment
  preservation using `T::transitive` and `T::antisymmetric`.
- **flip_colors**: Trivial — only modifies color fields; BST ordering and containment
  structurally unaffected. Added `reveal_with_fuel(spec_is_bst_link, 2)` and
  `reveal_with_fuel(link_contains, 2)`.
- **fix_up**: Chains rotate_left, rotate_right, flip_colors. Verified from their
  postconditions without additional proof annotations.
- **insert_link**: Take/put-back pattern with `TotalOrder::cmp`. Same proof structure
  as BSTSplayMtEph's `bst_insert`. Calls `fix_up` at end.
- **find_link, min_link, max_link**: Identical pattern to BSTSplayMtEph versions.

### Part 3: Strengthened wf predicates

Both `spec_bstsplaymteph_wf` and `spec_bstrbmteph_wf` now include
`&& spec_is_bst_link(self@)`, ensuring BST ordering is part of the well-formedness
contract.

## Remaining Chap37 Holes (3)

| # | Chap | File | Hole | Type | Blocker |
|---|------|------|------|------|---------|
| 1 | 37 | AVLTreeSeq.rs | Iterator::next external_body | external_body | Iterator::next can't have requires in Verus |
| 2 | 37 | AVLTreeSeqStEph.rs | fn_missing_requires (cached_height) | fn_missing_requires | Needs real requires clause |
| 3 | 37 | AVLTreeSeqStEph.rs | fn_missing_requires (value) | fn_missing_requires | Needs real requires clause |

These are pre-existing AVLTreeSeq holes unrelated to BST Splay/RB work.
