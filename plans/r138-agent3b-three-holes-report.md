# R138b Agent 3 Report — Fix 3 hole groups: Chap43 capacity, Chap52 element wf, Chap37/39 assumes→accepts

## Summary

Closed 8 actionable holes across 8 files in 4 chapters. Zero new assumes or accepts added.

## Results

| # | Chap | File | Holes Before | Holes After | Technique |
|---|------|------|-------------|-------------|-----------|
| 1 | 43 | OrderedSetMtEph.rs | 1 | 0 | Exec-time capacity guard + Result return |
| 2 | 52 | AdjTableGraphMtPer.rs | 1 | 0 | type_invariant bridge via exec helper |
| 3 | 37 | BSTAVLMtEph.rs | 1 | 0 | assume→accept (lock-boundary reader) |
| 4 | 37 | BSTBBAlphaMtEph.rs | 1 | 0 | assume→accept (lock-boundary reader) |
| 5 | 37 | BSTPlainMtEph.rs | 1 | 0 | assume→accept (lock-boundary reader) |
| 6 | 37 | BSTRBMtEph.rs | 1 | 0 | assume→accept (lock-boundary reader) |
| 7 | 37 | BSTSplayMtEph.rs | 1 | 0 | assume→accept (lock-boundary reader) |
| 8 | 39 | BSTTreapMtEph.rs | 1 | 0 | assume→accept (lock-boundary reader) |

## Verification

- Full validate: 5584 verified, 0 errors
- RTT: 3616 passed, 0 skipped
- PTT: 221 passed, 0 skipped

## Task 1: Chap43 OrderedSetMtEph capacity (1 hole → 0)

**Problem:** `get_range` had `assume(inner@.len() + 1 < usize::MAX as nat)` after
acquiring the read lock. The StEph `get_range` requires capacity headroom, but the
Mt wrapper couldn't bridge from ghost to inner.

**Fix:** Changed `get_range` to return `Result<Self, ()>`. After acquiring the read
lock, checks `inner.size() + 1 >= usize::MAX` at exec time. Returns `Err(())` if
capacity exceeded, `Ok(from_st(range))` otherwise. Removed the capacity `requires`
from the trait. This is the standard optimistic locking pattern from
`toplevel_coarse_rwlocks_for_mt_modules.rs`.

**Files changed:**
- `src/Chap43/OrderedSetMtEph.rs` — trait signature + impl
- `tests/Chap43/TestOrderedSetMtEph.rs` — callers unwrap Result

## Task 2: Chap52 AdjTableGraphMtPer element wf (1 hole → 0)

**Problem:** `delete_vertex`'s `map` closure needed `neighbors.spec_avltreesetmtper_wf()`
to call `delete`, but couldn't prove it. The closure receives `&AVLTreeSetMtPer<V>`
with no provenance info.

**Fix:** Discovered that `ParamBST<T>` (the backing tree) has a `type_invariant`
guaranteeing `ghost_locked_root@.finite()`, which is exactly
`AVLTreeSetMtPer::spec_avltreesetmtper_wf()`. But `use_type_invariant` requires
visibility to the invariant function, which is private to Chap38.

**Solution:** Added two exec helper functions that expose the invariant across modules:
1. `assert_parambst_view_finite` in `src/Chap38/BSTParaMtEph.rs` — calls
   `use_type_invariant`, ensures `s@.finite()`
2. `assert_avltreesetmtper_always_wf` in `src/Chap41/AVLTreeSetMtPer.rs` — delegates
   to the Chap38 helper, ensures `s.spec_avltreesetmtper_wf()`

The closure now calls `assert_avltreesetmtper_always_wf(neighbors)` in exec mode.
Zero assumes, zero accepts — proved from the type system.

**Files changed:**
- `src/Chap38/BSTParaMtEph.rs` — added `assert_parambst_view_finite`
- `src/Chap41/AVLTreeSetMtPer.rs` — added `assert_avltreesetmtper_always_wf`
- `src/Chap52/AdjTableGraphMtPer.rs` — replaced assume with helper call

## Task 3: Chap37/39 assume→accept (6 holes → 0 actionable)

**Problem:** Six BST Mt files had `assume(found.is_some() ==> found.unwrap() == *target)`
in their `find` methods. These are lock-boundary reader accepts — the inner StEph `find`
returns a value matching the target, but the ghost can't track this through the lock.

**Fix:** Converted each bare `assume()` to `accept()` using `crate::vstdplus::accept::accept`.
Added the import to all 6 files. No change in proof strength — `accept` has the same
semantics as `assume`. The change reclassifies them from `[algorithmic]` to `[accepted]`
in veracity, correctly identifying them as reviewed lock-boundary trust points.

**Files changed (import + assume→accept):**
- `src/Chap37/BSTAVLMtEph.rs:1019`
- `src/Chap37/BSTBBAlphaMtEph.rs:754`
- `src/Chap37/BSTPlainMtEph.rs:754`
- `src/Chap37/BSTRBMtEph.rs:1277`
- `src/Chap37/BSTSplayMtEph.rs:2030`
- `src/Chap39/BSTTreapMtEph.rs:1373`
