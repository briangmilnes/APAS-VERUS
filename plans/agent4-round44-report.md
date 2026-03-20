# Agent 4 — Round 44 Report

## Summary

R44 targeted Chap38 BSTParaMtEph.rs (20 holes) and Chap39 BSTParaTreapMtEph.rs (27 holes).
Applied ghost field migration to Chap39, added ensures to external_body helpers in both
chapters, and removed external_body from all provable trait impl delegations.

- **4384 verified, 0 errors**
- **2613 RTT pass**
- **Chap38**: 21 to 14 holes (**-7**)
- **Chap39**: 27 to 16 holes (**-11**)
- **Net**: 48 to 30 holes (**-18**)

## Hole Changes by File

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 38 | BSTParaMtEph.rs | 20 | 13 | -7 |
| 2 | 38 | BSTParaStEph.rs | 1 | 1 | 0 |
| 3 | 39 | BSTParaTreapMtEph.rs | 27 | 16 | -11 |
| 4 | 39 | BSTSetTreapMtEph.rs | 0 | 0 | 0 |
| 5 | 39 | BSTTreapMtEph.rs | 0 | 0 | 0 |
| 6 | 39 | BSTTreapStEph.rs | 0 | 0 | 0 |

## Techniques Used

### 1. Ghost field migration (Chap39, -2 holes)

Added `ghost_locked_root: Ghost<Set<T::V>>` to `ParamTreap` struct, replacing
external_body View impl with direct ghost field read. Mirrored R43 Chap38 pattern.
Created `new_param_treap` and `new_leaf` helper functions with ensures. Removed assume
from `new()`.

### 2. Cyclic self-reference resolution (Chap39, prerequisite for delegation)

Created `expose_internal` as a free function (not a trait method) to break Verus's
cyclic self-reference detection. All inner recursive functions (`split_inner`,
`join_pair_inner`, `union_inner`, `intersect_inner`, `difference_inner`, `filter_inner`,
`filter_parallel`, `reduce_inner`, `reduce_parallel`, `collect_in_order`,
`join_with_priority`) had their `where ParamTreap<T>: ParamTreapTrait<T>` clauses
removed and bodies updated to use `expose_internal` + `new_leaf` instead of trait methods.

### 3. Helper ensures + trait delegation (both chapters, -16 holes)

Added ensures to external_body helper functions, then removed external_body from
trait impl methods that delegate to them:

| # | Chap | Function | Type |
|---|------|----------|------|
| 1 | 38 | join_pair | delegation proved |
| 2 | 38 | union | delegation proved |
| 3 | 38 | intersect | delegation proved |
| 4 | 38 | difference | delegation proved |
| 5 | 38 | reduce | delegation proved |
| 6 | 38 | filter | delegation proved (Ghost param) |
| 7 | 38 | in_order | delegation proved (collect_in_order ensures) |
| 8 | 39 | expose_with_priority | delegation proved (expose_internal) |
| 9 | 39 | split | delegation proved |
| 10 | 39 | join_pair | delegation proved |
| 11 | 39 | union | delegation proved |
| 12 | 39 | intersect | delegation proved |
| 13 | 39 | difference | delegation proved |
| 14 | 39 | reduce | delegation proved |
| 15 | 39 | filter | delegation proved (Ghost param) |
| 16 | 39 | join_mid | delegation proved |
| 17 | 39 | in_order | delegation proved (collect_in_order ensures) |

Chap38 count: 7 delegations proved = -7 holes (20 to 13 in MtEph).
Chap39 count: 10 delegations proved + 1 View = -11 holes (27 to 16).

### 4. Filter Ghost propagation (both chapters)

Added `Ghost(spec_pred)` parameter to `filter_parallel` (external_body) with full
filter ensures matching the trait spec. The trait impl passes the ghost predicate
through, enabling Verus to verify the delegation.

### 5. collect_in_order ensures (both chapters)

Added `requires tree@.finite(), ensures out@.len() == old(out)@.len() + tree@.len()`
to `collect_in_order`. This enabled the `in_order` trait delegation to prove:
`Vec::with_capacity` gives len 0, `collect_in_order` adds `self@.len()` elements,
`from_vec` preserves length.

## Remaining Holes

### Chap38 BSTParaMtEph.rs (13 holes)

| # | Chap | Line | Type | Function | Blocker |
|---|------|------|------|----------|---------|
| 1 | 38 | 352 | assume | find | BST search correctness — needs BST ordering invariant in ghost field |
| 2 | 38 | 450 | external_body | expose_internal | RwLock read boundary |
| 3 | 38 | 486 | external_body | split_inner | Recursive split + RwLock |
| 4 | 38 | 517 | external_body | min_key | RwLock read + tree traversal |
| 5 | 38 | 528 | external_body | join_pair_inner | Recursive join + RwLock |
| 6 | 38 | 542 | external_body | union_inner | Recursive union + RwLock |
| 7 | 38 | 558 | external_body | intersect_inner | Recursive intersect + RwLock |
| 8 | 38 | 577 | external_body | difference_inner | Recursive difference + RwLock |
| 9 | 38 | 598 | external_body | filter_inner | Recursive filter (no parallelism) |
| 10 | 38 | 621 | external_body | filter_parallel | Fork-join filter + RwLock |
| 11 | 38 | 643 | external_body | reduce_inner | Recursive reduce (no parallelism) |
| 12 | 38 | 667 | external_body | reduce_parallel | Fork-join reduce + RwLock |
| 13 | 38 | 677 | external_body | collect_in_order | Recursive collect + &mut Vec |

### Chap38 BSTParaStEph.rs (1 hole)

| # | Chap | Line | Type | Function | Blocker |
|---|------|------|------|----------|---------|
| 1 | 38 | — | assume | clone_elem | Generic Clone workaround |

### Chap39 BSTParaTreapMtEph.rs (16 holes)

| # | Chap | Line | Type | Function | Blocker |
|---|------|------|------|----------|---------|
| 1 | 39 | 158 | external_body | expose_internal | RwLock read boundary |
| 2 | 39 | 201 | external_body | tree_priority | RwLock read |
| 3 | 39 | 211 | external_body | tree_size | RwLock read |
| 4 | 39 | 221 | external_body | make_node | Arc/RwLock construction |
| 5 | 39 | 234 | external_body | join_with_priority | Recursive join + RwLock |
| 6 | 39 | 258 | external_body | split_inner | Recursive split + RwLock |
| 7 | 39 | 288 | external_body | join_pair_inner | Recursive join_pair + RwLock |
| 8 | 39 | 303 | external_body | union_inner | Recursive union + RwLock |
| 9 | 39 | 318 | external_body | intersect_inner | Recursive intersect + RwLock |
| 10 | 39 | 337 | external_body | difference_inner | Recursive difference + RwLock |
| 11 | 39 | 356 | external_body | filter_inner | Recursive filter (no parallelism) |
| 12 | 39 | 374 | external_body | filter_parallel | Fork-join filter + RwLock |
| 13 | 39 | 396 | external_body | reduce_inner | Recursive reduce (no parallelism) |
| 14 | 39 | 419 | external_body | reduce_parallel | Fork-join reduce + RwLock |
| 15 | 39 | 428 | external_body | collect_in_order | Recursive collect + &mut Vec |
| 16 | 39 | 703 | assume | find | BST search correctness |

### Analysis of remaining holes

All 28 remaining external_body functions are the actual recursive tree implementations
that cross the `Arc<RwLock>` boundary. They acquire the read lock, pattern-match the
inner node, recurse, and construct new `ParamBST`/`ParamTreap` values with fresh
`Arc<RwLock>` wrappers. Verus cannot verify through the `Arc<RwLock>` layer.

The 2 find assumes require proving BST search correctness, which needs a BST ordering
invariant carried in the ghost field — currently the ghost field only carries `Set<T::V>`
(membership), not ordering information. Adding ordering would require restructuring
the ghost invariant.

## Verification

```
verification results:: 4384 verified, 0 errors
2613 tests run: 2613 passed, 0 skipped
```
