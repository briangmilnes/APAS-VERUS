# Agent 1 — Round 23 Report

## Mission

Build spec infrastructure for BSTSplayStEph.rs and prove all 7 holes (6 `external_body` + 1 `trivial_spec_wf`). Chap37 BST is the root blocker for 9 downstream chapters.

## Results Summary

- **BSTSplayStEph.rs holes**: 7 → 0 (−7, file clean)
- **Total project holes**: 234 → 227 (−7)
- **Verified functions**: 3958 → 3983 (+25)
- **Errors**: 0
- **RTT**: 2600 passed
- **PTT**: 147 passed

## Holes Eliminated

| # | Chap | File | Function | Technique |
|---|------|------|----------|-----------|
| 1 | 37 | BSTSplayStEph.rs | `find` (trait) | BST-guided search with `T::antisymmetric` |
| 2 | 37 | BSTSplayStEph.rs | `minimum` (trait) | Minimality proof: per-arm `forall` with `T::transitive` |
| 3 | 37 | BSTSplayStEph.rs | `maximum` (trait) | Maximality proof: symmetric to minimum |
| 4 | 37 | BSTSplayStEph.rs | `in_order` (trait) | Length ensures from recursive structure |
| 5 | 37 | BSTSplayStEph.rs | `pre_order` (trait) | Length ensures from recursive structure |
| 6 | 37 | BSTSplayStEph.rs | `insert` (trait) | Full splay + bst_insert spec infrastructure |
| 7 | 37 | BSTSplayStEph.rs | `spec_bstsplaysteph_wf` | Changed from `{ true }` to `{ spec_is_bst_link(&self.root) }` |

## Techniques Used

### 1. Pre-mutation ghost capture

BST ordering quantifiers reference live variables. After `.take()` mutates a field, the quantifier becomes vacuously true. Solution: capture ghost copies of node fields BEFORE the take, assert BST ordering facts while the live variables still hold the original values, then use the ghost copies in post-mutation proofs.

```rust
let ghost root_key = root.key;
let ghost orig_root_left = root.left;
proof {
    assert forall|x: T| spec_contains_link(&orig_root_left, x) implies
        (T::le(x, root_key) && x != root_key) by {};
}
let Some(mut left) = root.left.take() else { return root };
// Now orig_root_left still carries the ordering fact.
```

### 2. Decomposition of `old(link)` containment

For `&mut` BST insert, the solver can't connect `old(link)` to the reconstructed tree directly. Solution: decompose `spec_contains_link(old(link), x)` into `(node_key == x || old_left contains x || old_right contains x)` via ghost variables, prove both directions, then use the decomposed form for preservation and only-adds-value proofs.

### 3. Splay rotation proofs (10 cases)

Each splay rotation case (Zig, Zig-zig, Zig-zag, Zag, Zag-zag, Zag-zig, plus 4 else/no-child variants) requires:
- BST ordering quantifiers for the result node and inner nodes (`T::transitive` + `T::antisymmetric` for cross-subtree ordering)
- Element preservation (`forall|x| contains(orig, x) <==> contains(result, x)`) with structural decomposition
- Explicit `assert(spec_is_bst_link(&Some(result))) by { reveal_with_fuel(spec_is_bst_link, N); }` to defeat flaky conjunction proofs

### 4. Splay BST ordering for recursive cases

After recursive splay call, capture the splay result's BST ordering facts BEFORE taking its subtrees:

```rust
proof {
    assert forall|x: T| spec_contains_link(&lr_left, x) implies
        (T::le(x, lr_key) && x != lr_key) by {};
}
left.right = lr.left.take(); // After this, lr.left = None, fact is lost without capture.
```

### 5. Anti-flaky BST decomposition

When Verus reports "all sub-assertions succeeded but conjunction failed", decompose the proof bottom-up: assert BST for leaf subtrees, then inner nodes, then the result node, each with `reveal_with_fuel(spec_is_bst_link, 2)`.

## What Blocks Further Progress in Chap37

| # | Chap | File | Holes | Category | Blocker |
|---|------|------|------:|----------|---------|
| 1 | 37 | AVLTreeSeq.rs | 1 | external_body | Iterator::next has no `requires` slot (structural) |
| 2 | 37 | AVLTreeSeqMtPer.rs | 2 | external_body | Thread boundary: `&[T]` slices can't be `'static` (structural) |

BSTSplayStEph.rs is now clean. The remaining 3 Chap37 holes are structural limitations.
