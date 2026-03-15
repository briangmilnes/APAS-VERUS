# Agent 3 — Round 15 Report

## Summary

| # | Metric | Value |
|---|--------|-------|
| 1 | Holes before | 149 |
| 2 | Holes after | 147 |
| 3 | Delta | -2 |
| 4 | Target | -12 |
| 5 | Verified | 4078 |
| 6 | RTT | 2600 pass |
| 7 | Commit | (pending) |

## Changes by File

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 39 | BSTTreapMtEph.rs | 8 assume | 6 assume | -2 | &mut self + ghost field update |
| 2 | 39 | BSTParaTreapMtEph.rs | 15 ext_body | 15 ext_body | 0 | blocked (see below) |
| 3 | 38 | BSTParaMtEph.rs | 17 ext_body | 17 ext_body | 0 | blocked (see below) |

## What Was Done

### BSTTreapMtEph.rs (-2 assumes)

Converted `insert` and `delete` from `&self` to `&mut self`. With `&mut self`, the ghost
field `ghost_locked_root` can be updated after mutation, eliminating the assumes that
bridged the gap between the locked value and the ghost set.

- insert: `self.ghost_locked_root = Ghost(old_set.insert(value_view))` after `release_write`
- delete: `self.ghost_locked_root = Ghost(old_set.remove(target_view))` after `release_write`
- Updated `BSTTreapMtEphLit!` macro for `let mut`
- Rewrote all RTT tests in `TestBSTTreapMtEph.rs` for `&mut self` API

### Remaining 6 assumes in BSTTreapMtEph (structural)

All 6 are in read-only `&self` methods: find, size, minimum, maximum, in_order, pre_order.
These acquire a read lock, call a link-level function, and assume the result relates to
`self@` (the ghost set). The gap: no spec-level connection between the locked Link value
and `ghost_locked_root`. The RwLock predicate is frozen at construction and cannot track
the evolving set contents. The standard uses `accept()` at these boundaries, but the round
rules prohibit adding accept.

## What Was Blocked

### BSTParaTreapMtEph.rs (15 external_body — 0 removed)

Two independent blockers:

1. **Cyclic self-reference.** All outside-verus helper functions (split_inner,
   join_with_priority, join_pair_inner, reduce_parallel) have
   `where ParamTreap<T>: ParamTreapTrait<T>` bounds. Using `external_fn_specification` to
   make them callable from inside verus! creates a cycle:
   trait impl → method → external_fn_spec → trait bound → back to trait impl.

2. **Opaque View.** View is `external_body` returning `Set::empty()`. Computing a real View
   would require recursively traversing Arc<RwLock> nodes at the spec level, which is
   impossible (spec functions can't perform lock operations).

### BSTParaMtEph.rs (17 external_body — 0 removed)

Stretch target shares the opaque View blocker. While it avoids the self-referential trait
bound issue (no where clauses on helpers), all methods have real ensures clauses referencing
`self@`. With the View opaque, Verus can't prove any ensures clause after external_body
removal.

## Architectural Observations

The Arc<RwLock> per-node pattern (used by BSTParaMtEph and BSTParaTreapMtEph) is
fundamentally harder to verify than the single-RwLock coarse lock pattern (BSTTreapMtEph)
because:

- No single ghost field can track the entire tree's state
- Computing the set requires traversing all nodes (all locks)
- Spec functions can't perform lock operations
- The View must be external_body

This affects Chap38 (BSTParaMtEph) and Chap39 (BSTParaTreapMtEph) identically. These files
likely need a different verification strategy (e.g., per-node ghost tokens, or refactoring
to single-lock architecture) to make progress.
