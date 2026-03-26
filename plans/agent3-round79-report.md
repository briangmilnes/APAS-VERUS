# Agent 3 — Round 79 Report

## Objective

Prove or narrow the 3 remaining external_body holes in BSTSplayMtEph.rs (Chap37).

## Results

3 external_body holes → 1 assume hole. Two functions fully proven, one narrowed.

## Holes Before/After

| # | Chap | File | Function | Before | After | Notes |
|---|------|------|----------|--------|-------|-------|
| 1 | 37 | BSTSplayMtEph.rs | build_balanced | external_body | 1 assume | `spec_is_bst_link` needs sorted precondition |
| 2 | 37 | BSTSplayMtEph.rs | filter_parallel | external_body | 0 (proven) | Sequential recursion, closure requires |
| 3 | 37 | BSTSplayMtEph.rs | reduce_parallel | external_body | 0 (proven) | Sequential recursion, closure requires |

## Global Status

| Metric | R78 End | R79 End | Delta |
|--------|---------|---------|-------|
| Holes | 10 | 8 | -2 |
| Clean chapters | 44 | 44 | 0 |
| Verified | 4905 | 4908 | +3 |
| RTT | — | 2774 | — |
| PTT | — | 157 | — |

## Techniques Used

1. **Sequential recursion pattern from BSTRBMtEph R78**: Replaced `Arc::clone` + `clone_link` +
   `ParaPair!` with direct sequential recursion on `&node.left/right`. No thread spawning needed
   for recursive helpers — the outer Mt wrapper handles concurrency via RwLock.

2. **Closure requires propagation**: Added `forall|t: &T| #[trigger] predicate.requires((t,))`
   and `forall|a: T, b: T| #[trigger] op.requires((a, b))` to filter_parallel/reduce_parallel
   requires, plus matching trait method requires.

3. **Explicit Arc deref**: `(**predicate)(&node.key)` and `(**op)(a, b)` for calling through Arc.

4. **link_node_count instead of link_spec_size for recursion**: Splay's `link_spec_size` reads
   the stored `.size` field (non-recursive), so `reveal_with_fuel` doesn't help bound children.
   Used `link_node_count` (truly recursive) for the decreasing/bounding requires.

5. **Strengthened update ensures**: Added conditional ensures
   `link_spec_size(left) + link_spec_size(right) < usize::MAX ==> node.size == 1 + left + right`
   to `update`, enabling proof of `link_spec_size(Some(node)) <= values@.len()` in build_balanced.

6. **build_balanced to slice_subrange**: Replaced `to_vec()` + ParaPair with `slice_subrange` +
   sequential recursion, enabling `decreases values.len()` termination proof.

## Remaining Hole

The single remaining assume `spec_is_bst_link(Some(node))` in `build_balanced` requires proving
that splitting a sorted slice at its midpoint and recursively building left/right subtrees
preserves BST ordering. This needs a sorted precondition on `build_balanced` (currently absent)
and a proof that slice splitting preserves the ordering relationship between elements and the
pivot. Not blocked — just needs the sorted requires added and the ordering proof written.
