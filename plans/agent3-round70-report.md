# Agent 3 — Round 70 Report

## Assignment

BSTTreapStEph.rs (Chap39): Eliminate 1 proof hole, fix 27 fn_missing_wf style warnings.

## Results

| # | Chap | File | Holes Before | Holes After | Warnings Before | Warnings After |
|---|------|------|-------------|-------------|-----------------|----------------|
| 1 | 39 | BSTTreapStEph.rs | 1 | 1 | 27 | 0 |

- **Holes**: 1 → 1 (eq/clone workaround assume — not eliminable, see below)
- **Warnings**: 27 → 0 (all fn_missing_wf_requires, fn_missing_wf_ensures, fn_missing_requires fixed)

## Techniques Used

### 1. Bridge Lemma: `lemma_param_wf_implies_size_wf`

Wrote a new proof function establishing that `spec_param_wf_link` (the parametric wf predicate)
implies `spec_link_size_wf` (the module-level wf predicate, i.e., `spec_bsttreapsteph_wf`).
The proof recurses on the link, calling the existing `lemma_wf_size_eq_view_len` at each node
to bridge view-set-length size checks to tree-node-count size checks.

### 2. Bottom-Up WF Propagation on _st Helpers

Added `spec_bsttreapsteph_wf()` to requires and ensures of all 15 _st helper functions:

- `clone_with_view`, `make_node_treap_st`, `tree_priority_st`, `expose_to_parts_st`
- `join_with_priority_st`, `split_inner_st`, `join_pair_inner_st`
- `union_inner_st`, `intersect_inner_st`, `difference_inner_st`
- `filter_inner_st`, `reduce_inner_st`, `collect_in_order_st`

Functions returning `BSTTreapStEph<T>` got both requires and ensures; functions returning
other types (reduce → T, collect → void) got requires only.

### 3. Parametric Trait Caller Bridge

All 13 parametric trait impl methods now call `lemma_param_wf_implies_size_wf::<T>(&self.root)`
(and `&other.root` for binary operations) before delegating to _st helpers. This bridges the
parametric `spec_parambsttreapsteph_wf` (which equals `spec_param_wf_link`) to the module-level
`spec_bsttreapsteph_wf` that the _st functions require.

## Hole Not Eliminated

The assume at line 2693 (`assume(left_base == identity)` in `reduce_inner_st`) is an eq/clone
workaround. `reduce` needs two copies of `identity` for left/right recursive calls. After
`identity.clone()`, Verus cannot prove `left_base == identity` because `Clone::clone` has no
`ensures result == *self` spec for generic `T: StT`. Eliminating this would require either:

1. Adding `ClonePreservesView` to `StT` (project-wide cross-cutting change)
2. A bridge from view equality (`@`) to value equality (`==`)
3. Verus adding clone ensures to its trait model

None feasible in this round.

## Verification

- **validate.sh**: 4436 verified, 0 errors
- **rtt.sh**: 2528 tests passed, 0 failed
- **ptt.sh**: 145 tests passed, 0 failed
