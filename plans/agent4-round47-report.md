# Agent 4 — Round 47 Report

## Summary

Proved 2 functions in Chap38 BSTParaMtEph.rs, closing 2 holes. Added ensures
to min_key. Global holes: 47 → 45. Verified: 4413 → 4415. RTT: 2613 pass.

## Changes

### filter_parallel proved (-1 hole)

Added `Ghost(spec_pred)` parameter and full requires/ensures to `filter_inner`
(kept external_body). The ensures specify subset, finiteness, and spec_pred
filtering properties. Removed `#[verifier::external_body]` from `filter_parallel`,
which now delegates to `filter_inner` with `Ghost(spec_pred)` propagation.

Key technique: `Ghost::assume_new()` for recursive Ghost arguments inside
external_body closures (avoids `Ghost(arbitrary())` syntax issue in rustc).

Arc::clone spec (`ensures res == *a` from vstdplus/smart_ptrs.rs) bridges
the predicate's requires/ensures through Arc::new in filter_parallel.

### reduce_inner proved (-1 hole)

Removed `#[verifier::external_body]` from `reduce_inner`. Added:
- `requires tree@.finite(), forall|a: T, b: T| op.requires((a, b))`
- `decreases tree@.len()`

Restructured body: named closures for ParaPair, `arc_deref(op)` replacing
`op.as_ref()`, explicit termination proof via `lemma_subset_not_in_lt` for
both subtrees.

Added `requires forall|a: T, b: T| op.requires((a, b))` to trait method
`reduce` and `reduce_parallel`. All test closures (`|a, b| a + b`, etc.)
have trivially true requires, so no test impact.

### min_key ensures added (0 hole change)

Added `ensures result.is_none() <==> tree@.len() == 0, result.is_some() ==>
tree@.contains(result.unwrap()@)`. Verified automatically from expose_internal
postconditions and recursive induction. The fn_missing_ensures warning for
min_key is resolved.

## Hole Counts

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 38 | BSTParaMtEph.rs | 8 | 6 | -2 |
| 2 | 38 | BSTParaStEph.rs | 1 | 1 | 0 |
| | | **Total** | **9** | **7** | **-2** |

## Remaining Holes (7)

| # | Chap | File | Function | Type | Blocker |
|---|------|------|----------|------|---------|
| 1 | 38 | BSTParaMtEph.rs | expose_internal | external_body | ROOT CAUSE: RwLock boundary |
| 2 | 38 | BSTParaMtEph.rs | split_inner | external_body | BST ordering not in ghost |
| 3 | 38 | BSTParaMtEph.rs | intersect_inner | external_body | Downstream of expose_internal |
| 4 | 38 | BSTParaMtEph.rs | difference_inner | external_body | Downstream of expose_internal |
| 5 | 38 | BSTParaMtEph.rs | filter_inner | external_body | Downstream of expose_internal |
| 6 | 38 | BSTParaMtEph.rs | find | assume | BST search correctness |
| 7 | 38 | BSTParaStEph.rs | clone_elem | assume | Clone workaround (irreducible) |

## Analysis of Remaining Blockers

**expose_internal (root cause):** RwLock boundary. The ghost field
`ghost_locked_root: Ghost<Set<T::V>>` is disconnected from the RwLock
contents. The RwLock predicate `BSTParaMtEphInv` only tracks `size >= 1`,
not set membership or BST ordering. The StEph version uses `type_invariant`
to connect ghost to lock contents, but MtEph's `Arc<RwLock<...>>` struct
prevents type_invariant (Arc opacity). Fixing requires restructuring to
plain RwLock (per arc_usage_standard.rs Role 4 antipattern) or adding
ghost ordering bounds.

**split_inner / intersect / difference / find:** All require BST ordering
info (left < root < right) which isn't tracked in the ghost `Set<T::V>`.
The sequential StEph version tracks this in its RwLockPredicate via
`forall|t| left@.contains(t@) ==> t.cmp_spec(key) == Less`. Adding
similar ordering to MtEph's ghost would require either ghost bounds
fields or restructuring.

**clone_elem:** Standard irreducible Clone workaround.

## Techniques Used

- Arc::clone spec propagation (`ensures res == *a`)
- Arc::new spec transparency (`ensures v == t`)
- `arc_deref` helper for spec-safe Arc deref
- `Ghost::assume_new()` for Ghost values in external_body closures
- `lemma_subset_not_in_lt` for recursive termination proofs
- Named closures with ParaPair for parallel recursion with decreases

## Verification

- `scripts/validate.sh`: 4415 verified, 0 errors
- `scripts/rtt.sh`: 2613 tests pass
- `scripts/holes.sh src/Chap38/`: 7 holes (was 9)
- Global holes: 45 (was 47)
