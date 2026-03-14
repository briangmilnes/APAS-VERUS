# Agent 3 — Round 10 Report

## Summary

Enriched BSTParaStEph.rs predicate with ghost contents linkage, reducing Chap38 from 36 to 32 holes (-4). BSTTreapMtEph, BSTSetTreapMtEph, BSTParaTreapMtEph, and BSTParaMtEph all have fundamental structural blockers preventing incremental hole reduction.

## Hole Changes

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 38 | BSTParaStEph.rs | 19 | 15 | -4 |
| 2 | 38 | BSTParaMtEph.rs | 17 | 17 | 0 |
| 3 | 39 | BSTParaTreapMtEph.rs | 15 | 15 | 0 |
| 4 | 39 | BSTTreapMtEph.rs | 8 | 8 | 0 |
| 5 | 39 | BSTSetTreapMtEph.rs | 3 | 3 | 0 |
| - | - | **Total** | **62** | **58** | **-4** |

## Techniques Used

1. **Enriched RwLock predicate**: Replaced `PhantomData<T>` in `BSTParaStEphInv` with `pub ghost contents: Set<T::V>`. Added structural properties (disjointness, non-containment, ordering, size bounds) to predicate inv.

2. **Type invariant linkage**: Extended type_invariant with `self.ghost_locked_root@ =~= self.locked_root.pred().contents`, connecting ghost state to predicate.

3. **`use_type_invariant(self)`**: Critical Verus API — type invariants must be explicitly activated in each function. Added to `expose` and `size`.

4. **`*self = rebuilt` pattern**: Replaced lock-swap pattern (acquire_write → clone → release_write) in insert/delete with direct `*self = Self::join_m(...)`. Eliminates writer accept assumes entirely.

5. **Trivial view elimination**: Removed clone assumes from Exposed and NodeInner — their views are `()`, so `cloned@ == self@` is `() == ()`, trivially true.

## Verification

3986 verified, 0 errors (unchanged from baseline).

## Remaining Holes — Blockers

### BSTParaStEph.rs (15 holes)

| # | Hole | Blocker |
|---|------|---------|
| 1 | expose: k@ == node.key@ + ordering | T::clone has no verified ensures in Verus |
| 2 | insert overflow | Needs `self@.len() < usize::MAX` precondition (API change) |
| 3 | delete overflow | Same as insert |
| 4-14 | union/intersect/difference (11) | T-vs-T::V bridge: can't get exec witnesses from spec-level set membership |
| 15 | clone external_body | RwLock clone boundary |

### BSTTreapMtEph.rs (8 holes)

All assumes blocked by interior mutability design. Uses `&self` for insert/delete with RwLock. Ghost field (`ghost_locked_root`) is set at construction and never updated — no `&mut self` available to track mutations. No mechanism to connect ghost state to locked data for read-only operations either.

### BSTSetTreapMtEph.rs (3 holes)

All 3 (singleton, insert, delete) blocked by upstream BSTParaTreapMtEph having `ensures true` on insert/delete. Cannot prove post-state properties without upstream specs.

### BSTParaTreapMtEph.rs (15 external_body) + BSTParaMtEph.rs (17 external_body)

Both have fake `view()` (external_body returning `Set::empty()`), no ghost state tracking, unit-struct predicates, and algorithmic code outside `verus!`. All 32 external_body holes require complete structural rewrites (add ghost fields, real view, move code inside verus!, write proof bodies). Not incremental work.

## Commit

`aec369d6` on `agent3/ready`
