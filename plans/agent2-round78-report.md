# Agent 2 — Round 78 Report

## Objective

Prove or narrow 5 holes in BSTSplayMtEph.rs (Chap37).

## Results

- **Holes before**: 5 (BSTSplayMtEph.rs), 15 (project total)
- **Holes after**: 3 (BSTSplayMtEph.rs), 13 (project total)
- **Delta**: -2 holes
- **Verified**: 4902 (was 4898, +4)
- **RTT**: 2774 passed
- **PTT**: 157 passed

## Changes

| # | Chap | File | Hole | Action | Result |
|---|------|------|------|--------|--------|
| 1 | 37 | BSTSplayMtEph.rs | clone (external_body) | Wrote `clone_link` free fn with manual recursive clone | **Eliminated** |
| 2 | 37 | BSTSplayMtEph.rs | height (assume) | Added `link_node_count` spec fn, `lemma_height_le_node_count`, updated lock predicate | **Eliminated** |
| 3 | 37 | BSTSplayMtEph.rs | build_balanced (external_body) | Replaced `.clone()` with `clone_link()` inside body | Remains — blocked by to_vec specs + closure verification |
| 4 | 37 | BSTSplayMtEph.rs | filter_parallel (external_body) | Replaced `.clone()` with `clone_link()` inside body | Remains — blocked by closure + Arc + parallelism patterns |
| 5 | 37 | BSTSplayMtEph.rs | reduce_parallel (external_body) | Replaced `.clone()` with `clone_link()` inside body | Remains — blocked by closure + Arc + parallelism patterns |

## Techniques

### clone → clone_link (hole #1)

Wrote `clone_link<T>(link: &Link<T>) -> (c: Link<T>)` that manually recurses through the
tree structure, cloning each node field-by-field. Uses the standard `assume(c == *link)`
workaround for generic T clone (same pattern as BSTTreapMtEph). Removed `external_body`
from `Node::clone` and delegated to `clone_link`.

### height assume → lemma (hole #2)

The lock predicate bounded `link_spec_size` (cached `node.size`), but `link_height` is
structural. Added `link_node_count` (structural recursive node count), proved
`lemma_height_le_node_count` by induction. Changed lock predicate from
`link_spec_size(v) <= usize::MAX` to `link_node_count(v) <= usize::MAX`. Also changed
`height_rec` requires from `<` to `<=` (matching BSTRBMtEph pattern) with inline proof
assertions for child bounds.

### Parallel functions (holes #3-5)

Replaced `node.left.clone()` / `node.right.clone()` with `clone_link(&node.left)` /
`clone_link(&node.right)` inside all three external_body functions. Clone is no longer the
blocker. Remaining blockers:
- `build_balanced`: `to_vec()` on slices lacks Verus specs
- All three: closure verification with recursive calls + Arc captures + ParaPair macro
  requires proving `link_spec_size` bounds on recursive inputs, which needs `spec_size_consistent`

## Remaining Holes (BSTSplayMtEph.rs)

| # | Chap | File | Line | Function | Type | Blocked by |
|---|------|------|------|----------|------|------------|
| 1 | 37 | BSTSplayMtEph.rs | 1507 | build_balanced | external_body | to_vec specs, closure verification |
| 2 | 37 | BSTSplayMtEph.rs | 1534 | filter_parallel | external_body | closure + Arc + parallelism |
| 3 | 37 | BSTSplayMtEph.rs | 1567 | reduce_parallel | external_body | closure + Arc + parallelism |
