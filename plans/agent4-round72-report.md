# R72 Agent 4 Report: Iterator Standard Fix for Chap43 OrderedTable

## Scope

Fixed iterator standard compliance in 2 files identified by R71 review:
OrderedTableStEph.rs and OrderedTableStPer.rs.

## Changes

| # | Chap | File | Change | Result |
|---|------|------|--------|--------|
| 1 | 43 | OrderedTableStEph.rs | Rewrote section 10: IntoIter wrapping + ghost protocol | -1 hole (assume in next removed) |
| 2 | 43 | OrderedTableStPer.rs | Rewrote section 10: IntoIter wrapping + ghost protocol | -1 hole (assume in next removed) |
| 3 | 43 | ProveOrderedTableStPer.rs | Updated PTT: 4 tests (2 loop + 2 for-loop) | all pass |

## What Was Done

### Iterator rewrite (both files)

Replaced manual `ArraySeqStPerS`-backed iterator with `IntoIter<Pair<K, V>>` wrapping
(pattern from BalBinTreeStEph.rs). Changes per file:

1. **Struct**: `{sorted, pos, len}` → `{inner: IntoIter<Pair<K, V>>}`
2. **View**: `(int, Seq<(K::V, V::V)>)` → `(int, Seq<Pair<K, V>>)` (matches IntoIter's
   raw exec-type view; old ArraySeqStPerS mapped through T::V)
3. **iter_invariant**: Simplified to `0 <= it@.0 <= it@.1.len()` (removed `obeys_feq_full`)
4. **next()**: Delegates to `self.inner.next()` — NO assume needed (was
   `assume(iter_invariant(self))`)
5. **iter()**: Removed `obeys_feq_full` from requires. Body: `sorted.seq.into_iter()`
6. **Ghost iterator**: Added `OrderedTableSt{Eph,Per}GhostIterator` with pos/elements
7. **ForLoopGhostIteratorNew**: Connects exec iterator view to ghost iterator
8. **ForLoopGhostIterator**: 6 spec fns (exec_invariant, ghost_invariant, ghost_ensures,
   ghost_decrease, ghost_peek_next, ghost_advance)
9. **IntoIterator for &Self**: Added for StEph (was missing); updated for StPer
10. **Debug/Display**: Added outside verus! for both iterator types

### Key technical insight

The old `ArraySeqStPerS<T: View>` has `View::V = Seq<T::V>` (maps elements through views).
`IntoIter<T>` has `View::V = (int, Seq<T>)` (raw exec type). The View type for the
iterator struct had to change from `Seq<(K::V, V::V)>` to `Seq<Pair<K, V>>` to match.
Consumer code (AugOrderedTableSt{Eph,Per}) only uses `.len()` so is unaffected.

### PTT updates

Changed ghost variable types from `Seq<(u64, u64)>` to `Seq<Pair<u64, u64>>` and
`items.push(x@)` to `items.push(x)` to match the new View type.

## Iterator Standard Compliance (post-fix)

| # | Component | StEph | StPer |
|---|-----------|-------|-------|
| 1 | Custom iterator struct | present | present |
| 2 | View for iterator | `(int, Seq<Pair<K,V>>)` | `(int, Seq<Pair<K,V>>)` |
| 3 | iter_invariant | present (simplified) | present (simplified) |
| 4 | Iterator::next | present (no assume) | present (no assume) |
| 5 | Ghost iterator struct | **present** | **present** |
| 6 | ForLoopGhostIteratorNew | **present** | **present** |
| 7 | ForLoopGhostIterator | **present** (6 spec fns) | **present** (6 spec fns) |
| 8 | View for ghost iterator | **present** | **present** |
| 9 | iter() method | present (relaxed requires) | present (relaxed requires) |
| 10 | IntoIterator for &Self | **present** | present |

All 10 components present in both files.

## Verification

- 4436 verified, 0 errors
- 2528 RTT pass
- 147 PTT pass (including 4 OrderedTableStPer iterator tests)

## Net Hole Change

-2 holes (removed `assume(iter_invariant(self))` from next() in both files).
Pre-existing algorithmic holes (axiom assumes, from_sorted_entries external_body) unchanged.
