# R161 Agent 3 — Minimize Chap37 BSTSplayStEph Proofs

## Summary

Removed 60 unnecessary proof assertions from `src/Chap37/BSTSplayStEph.rs`.

- Baseline (HEAD before edits): **250 asserts**
- After minimization: **190 asserts**
- Delta: **-60 asserts (-24%)**
- Verification: **5748 verified, 0 errors** (full validate)

---

## Per-Function Results

| # | Chap | File | Function | Before | After | Delta |
|---|------|------|----------|--------|-------|-------|
| 1 | 37 | BSTSplayStEph.rs | `splay` (all 6 rotation cases) | ~215 | ~158 | -57 |
| 2 | 37 | BSTSplayStEph.rs | `bst_insert` (Less + Greater) | ~35 | ~32 | -3 |

---

## What Was Removed

### Assignment-Tracking Proof Blocks (~40 asserts)

Removed standalone `proof { assert(x.key == x_key); assert(x.left == y); ... }` blocks
that appear between mutation steps. After `root.left = left.right.take()` where ghost
variables captured `root_key = root.key` and `orig_left_right = left.right` before the
mutation, Verus's mutation tracking + ghost capture semantics mean Z3 already knows these
equalities. The explicit assertions were redundant.

Affected cases: Zig, Zig-zig, Zig-zig None, Zig-zag, Zig-zag None, Zag, Zag-zag,
Zag-zag None, Zag-zig, Zag-zig None.

### `spec_is_bst_link` Helper Scaffolding (~10 asserts)

In the Zig-zag full case and Zag-zig full case, removed intermediate scaffolding assertions
(`spec_is_bst_link(&lr_left)`, `spec_is_bst_link(&lr_right)`, `spec_is_bst_link(&orig_left_left)`,
`spec_is_bst_link(&orig_root_right)`, `spec_is_bst_link(&Some(left))`, `spec_is_bst_link(&Some(root))`).
Replaced the terminal assertion from `reveal_with_fuel(..., 2)` to `reveal_with_fuel(..., 4)`,
which gives Z3 enough unfolding depth to verify the goal directly.

### `bst_insert` Decompose Foralls (~6 asserts)

In both Less and Greater branches of `bst_insert`, removed redundant bidirectional
decomposition foralls that broke `spec_contains_link(old(link), x)` into three components.
These were redundant because the preservation and only-adds forall bodies use
`reveal_with_fuel(spec_contains_link, 2)` which lets Z3 unfold directly.

---

## What Was NOT Removed

### Empty-Body Pre-Mutation Foralls (lr_left/lr_right, rl_left/rl_right)

`assert forall|x: T| spec_contains_link(&lr_left, x) implies (T::le(x, lr_key) && x != lr_key) by {};`

These look trivially removable (empty body) but are essential. They force Z3 to instantiate
the BST ordering fact from the splay result BEFORE the tree mutation takes `lr.left`.
After `left.right = lr.left.take()`, the field `lr.left` becomes `None` — Z3 can no longer
recover the pre-mutation BST ordering for `lr_left`. Removing them caused a verification
failure in the Zig-zag case. They remain.

### Empty-Body Root/Left/Right Pre-Mutation Foralls

`assert forall|x: T| spec_contains_link(&orig_root_left, x) implies (...) by {};`

These capture BST ordering of root subtrees before the root is mutated. Same pattern —
essential for the post-mutation proof to access pre-mutation ordering facts.

### Substantive `spec_contains_link` Chain Assertions

Assertions like `assert(spec_contains_link(&orig_left_left, x)); assert(spec_contains_link(&orig_root_left, x));`
inside forall bodies. These are genuine intermediate reasoning steps for chaining membership
through multiple tree levels. Z3 cannot close these goals without them given the quantifier
instantiation depth available.

---

## Techniques Used

1. **Ghost capture analysis**: Identified which post-mutation assertions were trivially
   provable from Verus's own mutation tracking, eliminating the need for explicit proofs.
2. **Fuel escalation**: In cases where intermediate helper assertions were removed, raised
   `reveal_with_fuel` level from 2 to 4 to compensate for removed scaffolding.
3. **Targeted stripping with validation**: Removed candidate groups, validated, restored on
   failure with explanation of why the assertions are load-bearing.

---

## Verification

Full codebase: `scripts/validate.sh` → **5748 verified, 0 errors**.
