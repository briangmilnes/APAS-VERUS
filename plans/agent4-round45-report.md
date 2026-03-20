# Agent 4 — Round 45 Report

## Summary

Proved 13 holes across Chap38 (BSTParaMtEph) and Chap39 (BSTParaTreapMtEph).
Restructured parallel recursive functions to use named closures with ParaPair!.
Strengthened `split_inner` and `expose_internal` ensures in both chapters.

- **Verification**: 4401 verified, 0 errors (was 4388)
- **RTT**: 2613 pass
- **Total holes**: 99 → 86 (-13)

## Holes Before/After by File

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 38 | BSTParaMtEph.rs | 13 | 8 | -5 |
| 2 | 38 | BSTParaStEph.rs | 1 | 1 | 0 |
| 3 | 39 | BSTParaTreapMtEph.rs | 16 | 8 | -8 |
| | | **Total** | **30** | **17** | **-13** |

## Functions Proved

| # | Chap | File | Function | Technique |
|---|------|------|----------|-----------|
| 1 | 38 | BSTParaMtEph.rs | `collect_in_order` | `decreases tree@.len()`, `lemma_subset_not_in_lt` |
| 2 | 38 | BSTParaMtEph.rs | `min_key` | `decreases tree@.len()`, `lemma_subset_not_in_lt` |
| 3 | 38 | BSTParaMtEph.rs | `reduce_parallel` | Trivial body (`Arc::new` + call) |
| 4 | 38 | BSTParaMtEph.rs | `join_pair_inner` | `size()` for finiteness, non-recursive |
| 5 | 38 | BSTParaMtEph.rs | `union_inner` | Named closures + ParaPair!, set algebra |
| 6 | 39 | BSTParaTreapMtEph.rs | `tree_priority` | `arc_deref` restructure |
| 7 | 39 | BSTParaTreapMtEph.rs | `tree_size` | `arc_deref` restructure |
| 8 | 39 | BSTParaTreapMtEph.rs | `make_node` | Fixed ghost field, overflow-safe size |
| 9 | 39 | BSTParaTreapMtEph.rs | `collect_in_order` | `decreases tree@.len()`, `lemma_subset_not_in_lt` |
| 10 | 39 | BSTParaTreapMtEph.rs | `join_with_priority` | Recursive, `decreases left@.len() + right@.len()` |
| 11 | 39 | BSTParaTreapMtEph.rs | `reduce_parallel` | Trivial body (`Arc::new` + call) |
| 12 | 39 | BSTParaTreapMtEph.rs | `join_pair_inner` | Recursive, `lemma_len_subset` for termination |
| 13 | 39 | BSTParaTreapMtEph.rs | `union_inner` | Named closures + ParaPair!, set algebra |

## Spec Strengthening (no hole reduction, enables future proofs)

| # | Chap | File | Function | Added ensures |
|---|------|------|----------|---------------|
| 1 | 38 | BSTParaMtEph.rs | `expose_internal` | `tree@.finite()`, subtree `subset_of`, `disjoint`, set decomposition |
| 2 | 38 | BSTParaMtEph.rs | `split_inner` | `!parts.0@.contains(key@)`, set decomposition |
| 3 | 39 | BSTParaTreapMtEph.rs | `expose_internal` | `!left@.contains(key@)`, `!right@.contains(key@)`, `disjoint` |

## Key Techniques

1. **`size()` trick for finiteness**: `ParamBST::size()` has `ensures self@.finite()` with no requires. Calling `let _ = x.size()` establishes finiteness without adding preconditions to the function signature. Used in `join_pair_inner` (Chap38) and several recursive functions.

2. **Named closures without ghost captures**: Ghost `Set<T::V>` values contain `FnSpec` which lacks `Send`. Instead of `let ghost view = x@`, reference `x@` directly in closure ensures (the exec variable is captured by move, its view is available in spec context). This avoids the `Send` bound violation while preserving the proof.

3. **`lemma_subset_not_in_lt` for decreases**: For recursive functions on persistent trees, `expose_internal` gives strict subsets (left/right don't contain root key). `s.lemma_subset_not_in_lt(parent, witness)` proves `s.len() < parent.len()`, establishing the decreases bound.

4. **Union set algebra auto-proves**: Z3 handles `union_inner`'s postcondition `result@ == a@.union(b@)` automatically because union's set algebra doesn't require BST ordering — the containment check `a@.union(b@).contains(v)` distributes cleanly. In contrast, `intersect` and `difference` need cross-disjointness between subtrees (BST ordering), which isn't tracked in the ghost sets.

## Remaining Holes (17)

### Chap38 BSTParaMtEph.rs (8 holes)

| # | Function | Type | Blocker |
|---|----------|------|---------|
| 1 | `find` | assume (rwlock:reader) | RwLock boundary |
| 2 | `expose_internal` | external_body (root cause) | RwLock acquire_read |
| 3 | `split_inner` | external_body | BST ordering not in ghost Set |
| 4 | `intersect_inner` | external_body | BST ordering (cross-disjointness) |
| 5 | `difference_inner` | external_body | BST ordering (cross-disjointness) |
| 6 | `filter_inner` | external_body | Arc::clone has no Verus spec |
| 7 | `filter_parallel` | external_body | Blocked by filter_inner (no ensures) |
| 8 | `reduce_inner` | external_body | Arc::clone has no Verus spec |

### Chap38 BSTParaStEph.rs (1 hole)

| # | Function | Type | Blocker |
|---|----------|------|---------|
| 1 | `clone_elem` | assume (algorithmic) | Clone bridge |

### Chap39 BSTParaTreapMtEph.rs (8 holes)

| # | Function | Type | Blocker |
|---|----------|------|---------|
| 1 | `expose_internal` | external_body (root cause) | RwLock acquire_read |
| 2 | `split_inner` | external_body | BST ordering not in ghost Set |
| 3 | `intersect_inner` | external_body | BST ordering (cross-disjointness) |
| 4 | `difference_inner` | external_body | BST ordering (cross-disjointness) |
| 5 | `filter_inner` | external_body | Arc::clone has no Verus spec |
| 6 | `filter_parallel` | external_body | Blocked by filter_inner (no ensures) |
| 7 | `reduce_inner` | external_body | Arc::clone has no Verus spec |
| 8 | `find` assume | assume (algorithmic) | Clone bridge in find loop |

## What Would Unblock More

1. **Verus spec for `Arc::clone`**: Would unblock `filter_inner` and `reduce_inner` in both chapters (4 holes). The spec should be `ensures cloned == *self` (same as Arc::new).

2. **BST ordering invariant in ghost state**: Would unblock `split_inner`, `intersect_inner`, and `difference_inner` in both chapters (6 holes). Requires adding a spec-level ordering predicate (e.g., `spec_all_lt(set, bound)`) to the ghost state and propagating it through expose/split.

3. **RwLock ghost shadow pattern**: Would unblock `expose_internal` and `find` assume in both chapters (4 holes). The current pattern uses `Arc<RwLock<...>>` for the tree root; migrating to plain `RwLock` with `Ghost` shadow field would eliminate the root cause.
