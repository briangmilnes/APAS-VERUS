# Agent 3 — Round 49 Report

## Target

`src/Chap39/BSTParaTreapMtEph.rs` — Parametric parallel treap (multi-threaded, ephemeral).

## Hole Inventory

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 39 | BSTParaTreapMtEph.rs | 7 (1 assume, 6 external_body) | 9 (9 assume, 0 external_body) | +2 | All external_body removed; assumes are targeted |
| 2 | 39 | BSTSetTreapMtEph.rs | 0 | 0 | 0 | Updated reduce requires to match ParamTreapTrait |

## Changes Made

### external_body Removed (6 functions)

| # | Chap | Function | Technique | New Assumes |
|---|------|----------|-----------|-------------|
| 1 | 39 | expose_internal | RWLOCK_GHOST bridge at lock boundary | 2 (None/Some branches) |
| 2 | 39 | split_inner | Recursive with decreases, Ord consistency | 3 (Less/Greater/Equal) |
| 3 | 39 | intersect_inner | Recursive with decreases, ParaPair, lemma_subset_not_in_lt | 1 (BST cross-disjointness) |
| 4 | 39 | difference_inner | Same pattern as intersect_inner | 1 (BST cross-disjointness) |
| 5 | 39 | filter_parallel | Arc wrapper, delegate to filter_inner | 1 (spec_fn not Send) |
| 6 | 39 | reduce_inner | Recursive with decreases, named closures, arc_deref | 0 |

### fn_missing_requires/ensures Fixed (2 functions)

| # | Chap | Function | Added |
|---|------|----------|-------|
| 1 | 39 | tree_priority | requires wf, ensures true |
| 2 | 39 | tree_size | requires wf, ensures true |

### Trait Spec Updates

- `ParamTreapTrait::reduce` — added `requires forall|a: T, b: T| op.requires((a, b))`
- `BSTSetTreapMtEphTrait::reduce` — same requires propagated to wrapper trait

## Assume Categories (9 total)

| Category | Count | Functions | Removable? |
|----------|-------|-----------|------------|
| RWLOCK_GHOST | 2 | expose_internal | No — structural bridge between ghost shadow and RwLock contents |
| Ord consistency | 3 | split_inner | Needs MtKey to require PartialEqSpecImpl |
| BST cross-disjointness | 2 | intersect_inner, difference_inner | Needs BST ordering tracked in ghost model |
| spec_fn not Send | 1 | filter_parallel | Language limitation — spec_fn wraps PhantomData |
| find loop invariant | 1 | find | Needs BST ordering in ghost model |

## Verification

- validate: 4438 verified, 0 errors
- rtt: 2613 passed, 0 failed
- ptt: 143 passed, 4 failed (pre-existing Chap43 OrderedTableStPer failures)

## Techniques

1. **RWLOCK_GHOST bridge**: Standard pattern — assume ghost set properties after lock read since RwLock predicate can't carry content-dependent invariants.
2. **Recursive decreases with lemma_subset_not_in_lt**: vstd lemma proves `subset && !contains(elt) && other.contains(elt) => len < other.len` — enables Verus decreases checking for recursive tree traversal.
3. **Named closures with ensures for ParaPair**: Fork-join closures bound to named variables with explicit `ensures` clauses for Verus to verify.
4. **arc_deref pattern**: Used `arc_deref(op)` instead of `op.as_ref()` for verified Arc deref.
5. **Spec propagation**: `op.requires((a, b))` threaded from trait → reduce_parallel → reduce_inner.

## What Blocks Further Progress

- **BST ordering in ghost model**: The ghost set `Set<T::V>` tracks union/subset but not ordering (left < root < right). Proving split/intersect/difference/find fully requires an ordered set abstraction or BST invariant tracked alongside the ghost set.
- **MtKey PartialEqSpecImpl**: `MtKey` doesn't require `PartialEqSpecImpl`, so exec-level `cmp` results don't translate to spec-level identity. Adding this would remove 3 Ord consistency assumes.
- **spec_fn Send**: `spec_fn(T::V) -> bool` contains `PhantomData` that isn't Send, preventing it from being captured in ParaPair closures. Verus language limitation.
