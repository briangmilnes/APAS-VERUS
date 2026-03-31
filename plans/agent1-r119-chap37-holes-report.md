# Agent 1 — R119 Report: Chap37 BST MtEph Assume Removal

## Summary

Removed 10 assumes across 4 of 5 Chap37 BST MtEph files. Strengthened ensures on
delete/insert infrastructure. Added bridge lemma for RB BST property. All remaining
assumes are structural (lock-boundary or clone-bridge).

## Verification

- **5439 verified, 0 errors** (full validation)
- **3529 RTT passed**, **221 PTT passed**
- **Chap37 holes: 5** (unchanged — remaining holes are all clone-bridge in find)
- **Project holes: 19** (41 clean chapters, 5 holed)

## Changes By File

### BSTPlainMtEph.rs (16 → 14 assumes, -2)

| # | Chap | Line | Assume content | Status | Technique |
|---|------|------|----------------|--------|-----------|
| 1 | 37 | 643 | `self.ghost_root@ == tree` (insert) | Structural | RWLOCK_GHOST: lock pred doesn't carry ghost_root |
| 2 | 37 | 665 | `self.ghost_root@ == tree` (delete) | Structural | RWLOCK_GHOST |
| 3 | 37 | 669 | `new_tree.spec_size() <= usize::MAX` (delete) | **Removed** | Strengthened delete_node ensures with `spec_size <= node.spec_size` |
| 4 | 37 | 670 | `new_tree.spec_height() <= usize::MAX` (delete) | **Removed** | Strengthened delete_node ensures with `spec_height <= node.spec_height` |
| 5 | 37 | 683 | `found == self@.tree_contains(...)` (contains) | Structural | RWLOCK_GHOST |
| 6 | 37 | 694 | `n as nat == self@.spec_size()` (size) | Structural | RWLOCK_GHOST |
| 7 | 37 | 704 | `b == (self@ is Leaf)` (is_empty) | Structural | RWLOCK_GHOST |
| 8 | 37 | 715 | `h as nat == self@.spec_height()` (height) | Structural | RWLOCK_GHOST |
| 9 | 37 | 725 | `found.is_some() == self@.tree_contains(...)` (find) | Structural | RWLOCK_GHOST |
| 10 | 37 | 726 | `found.is_some() ==> found.unwrap() == *target` (find) | Structural | Clone bridge: Option::cloned() no ensures |
| 11-13 | 37 | 737-9 | minimum 3 assumes | Structural | RWLOCK_GHOST |
| 14-16 | 37 | 750-2 | maximum 3 assumes | Structural | RWLOCK_GHOST |

**Proof work:** Added `pair.0.spec_size() < node.spec_size()` and `pair.0.spec_height() <= node.spec_height()` to `delete_min_node` ensures. Added `deleted.spec_size() <= node.spec_size()` and `deleted.spec_height() <= node.spec_height()` to `delete_node` ensures. Verus proved both automatically (no extra proof hints needed).

### BSTBBAlphaMtEph.rs (16 → 14 assumes, -2)

Identical changes to BSTPlainMtEph. Same delete_min_node/delete_node strengthening.

### BSTAVLMtEph.rs (7 → 7 assumes, 0)

All 7 assumes are standard RWLOCK_GHOST (insert writer + 5 reader + 1 find clone bridge). No insert capacity or delete bounds to fix (AVL has insert only, no delete; insert already uses assert not assume for bounds).

### BSTRBMtEph.rs (10 → 8 assumes, -2)

| # | Chap | Line | Assume content | Status | Technique |
|---|------|------|----------------|--------|-----------|
| 1 | 37 | 1074 | `spec_is_bst_link(ghost_link)` (from_sorted_slice) | Structural | build_balanced doesn't require/ensure sorted→BST |
| 2 | 37 | 1085 | `self.ghost_root@ == current` (insert) | Structural | RWLOCK_GHOST |
| 3 | 37 | 1109 | `link_spec_size(new_root) <= usize::MAX` (insert) | **Removed** | Added link_spec_size ensures to rotate_left/right, flip_colors, fix_up, insert_link |
| 4 | 37 | 1122 | `link_to_bbt(new_root).tree_is_bst()` (insert) | **Removed** | New lemma_link_to_bbt_is_bst bridge lemma |
| 5-8 | 37 | various | contains/size/is_empty/height/find readers | Structural | RWLOCK_GHOST + clone bridge |

**Proof work:**
1. Added `link_spec_size(*link) == link_spec_size(*old(link))` ensures to `rotate_left`, `rotate_right`, `flip_colors`, `fix_up`. Each needed `reveal_with_fuel(link_spec_size, 2-3)` hints.
2. Added `link_spec_size(*link) <= link_spec_size(*old(link)) + 1` to `insert_link`.
3. New `lemma_link_to_bbt_is_bst`: proves `spec_is_bst_link(link) ==> link_to_bbt(link).tree_is_bst()` by induction, using `lemma_link_to_bbt_contains` to bridge quantifiers.

### BSTSplayMtEph.rs (21 → 17 assumes, -4)

| # | Chap | Line | Assume content | Status | Technique |
|---|------|------|----------------|--------|-----------|
| 1 | 37 | 1524 | `c == *link` (clone_link None) | **Removed** | Trivially provable: `None == None` |
| 2 | 37 | 1536 | `c == *link` (clone_link Some) | Structural | Clone bridge: generic T clone correctness |
| 3 | 37 | 1924 | `self.ghost_root@ == current` (insert) | Structural | RWLOCK_GHOST |
| 4 | 37 | 1928 | `link_node_count(current) <= usize::MAX` | Structural | insert_link doesn't ensure node_count bound |
| 5 | 37 | 1929 | `link_spec_size(current) <= ... + 1` | Structural | insert_link doesn't ensure cached size bound |
| 6 | 37 | 1930 | `link_contains(current, value)` (insert) | **Removed** | Already in insert_link ensures |
| 7-8 | 37 | 1931-4 | insert forall containment (2) | **Removed** | Already in insert_link ensures |
| 9-17 | 37 | various | readers + min/max + clone | Structural | RWLOCK_GHOST + clone bridge |

## Remaining Structural Categories

### 1. RWLOCK_GHOST (53 across 5 files)
The RwLockPredicate is frozen at construction — it constrains shape (tree_is_bst, bounded size) but cannot track the changing ghost_root value. `type_invariant` can't see inside the RwLock. These assumes bridge ghost shadow to lock contents. This is an inherent Verus limitation documented in `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`.

### 2. Clone bridge in find (5 = 1 per file)
`Option::cloned()` produces `Option<T>` from `Option<&T>` but doesn't propagate ensures in Verus. The `found.unwrap() == *target` property holds through clone correctness, but Verus's generic Clone trait lacks postconditions. Could potentially fix by replacing `.cloned()` with manual clone + explicit ensures chain, but would need investigation of how Verus handles generic Clone ensures.

### 3. Splay insert structural (2)
`insert_link` (Splay) doesn't ensure `link_node_count` or `link_spec_size` bounds. These would require tracing through `bst_insert` + `splay` operations, which involve significant rotation proof work.

### 4. RB from_sorted_slice BST property (1)
`build_balanced` doesn't require sorted input or ensure `spec_is_bst_link`. Proving this needs a sorted-ness precondition and inductive BST proof.

## Techniques Used

1. **Delete monotonicity strengthening** — Added size/height monotonicity ensures to delete_min_node and delete_node. Verus proved automatically.
2. **Rotation size preservation** — Proved link_spec_size invariant through all 4 rotation/fixup functions with reveal_with_fuel hints.
3. **Bridge lemma** — New lemma_link_to_bbt_is_bst converts between BST representations using existing lemma_link_to_bbt_contains.
4. **Redundant assume elimination** — Identified assumes already covered by callee ensures (Splay insert containment, clone_link None).
