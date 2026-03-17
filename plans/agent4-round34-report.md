# Agent 4 Round 34 Report

## Assignment

Prove 6 `assume()` holes in `src/Chap39/BSTTreapMtEph.rs` — lock-boundary bridges
between locked `Link<T>` operations and the ghost `Set<V>` view.

## Findings: Structural Lock-Boundary Holes (Unprovable)

All 6 assumes are **structurally unprovable** with Verus's current `RwLock` API. They
match the `accept()` pattern documented in `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`
(lines 222–238). The standard itself uses `accept()` at identical boundaries.

### Root Cause

The ghost-shadow pattern stores two disconnected pieces of state:
- `locked_root: RwLock<Link<T>, BSTTreapMtEphInv>` — the actual tree (behind lock)
- `ghost_locked_root: Ghost<Set<V>>` — the abstract set view (outside lock)

The `RwLockPredicate` (`BSTTreapMtEphInv`) is a unit struct — it cannot connect
`spec_set_of_link(locked_link)` to `ghost_locked_root@`. Four independent mechanisms
were investigated:

| # | Mechanism | Why It Fails |
|---|-----------|-------------|
| 1 | Strengthen RwLock invariant with ghost set | Predicate is immutable (fixed at construction); ghost set evolves with insert/delete |
| 2 | RwLock spec-level value accessor | `RwLock` has no `spec fn value(&self) -> V`; only `ReadHandle::view()` exposes it during lock hold |
| 3 | Type invariant connection | `#[verifier::type_invariant]` can't reference the locked value — it's behind the lock |
| 4 | Lock (Link, Ghost<Set>) together | Ghost inside lock still disconnected from ghost outside lock; same gap |

### Proof Chain Analysis

Each function follows: acquire_read → call link operation → release_read → assume.

The link operations provide:
- `find_link`: `found.is_some() <==> spec_contains_link(link, *target)`
- `size_link`: `sz as nat == Lnk::spec_size_link(link)`
- `min_link`/`max_link`: `result.is_some() ==> spec_contains_link(link, *result.unwrap())`
- `in_order_collect`/`pre_order_collect`: `out@.len() == old(out)@.len() + Lnk::spec_size_link(link)`

The gap in ALL cases is: **`spec_set_of_link(handle.view()) == self.ghost_locked_root@`**.
This relationship is maintained by `insert`/`delete` (which update both the locked tree
and the ghost set), but no formal connection exists because the RwLock API doesn't
allow expressing a relationship between the locked value and external state.

### Survey: Lock-Boundary Assumes Across All Mt Modules

| Pattern | Files | Example |
|---------|-------|---------|
| Lock-boundary `assume()` | 3 files (15 assumes) | BSTTreapMtEph, AVLTreeSetMtEph, OrderedSetMtEph |
| Lock-boundary `accept()` | 5 files (34 accepts) | SetMtEph, DirGraphMtEph, etc. |
| No lock-boundary holes | 5 files | MinEditDistMtEph, SubsetSumMtEph, etc. (delegation pattern) |

The 5 clean files avoid the gap by **delegating** to StEph methods whose `ensures`
directly match the Mt trait's `ensures`, without referencing `self@`.

## Changes Made

| # | Chap | File | Change | Effect |
|---|------|------|--------|--------|
| 1 | 39 | BSTTreapMtEph.rs | Remove `requires true` from `clone_link` (was L354) | -1 warning |
| 2 | 39 | BSTTreapMtEph.rs | Remove `requires true` from `size_link` (was L390) | -1 warning |

No holes removed. These functions genuinely have no preconditions.

## Verification State

- 4154 verified, 0 errors
- 2613 RTT pass
- 171 total holes (unchanged)
- 2 `requires_true` warnings removed

## Recommendation

Convert the 6 assumes to `accept()` to match the project standard. This would:
1. Align with `toplevel_coarse_rwlocks_for_mt_modules.rs` pattern
2. Reclassify them from "holes" to "structural false positives" (lock-boundary accepts)
3. Net effect: -6 holes

This requires explicit user approval per CLAUDE.md.
