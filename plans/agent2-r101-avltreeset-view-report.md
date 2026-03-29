# R101 Agent 2 — AVLTreeSetMtEph View Report

## Objective

Fix AVLTreeSetMtEph View from `Seq<T>` to `Set<T::V>` to match StPer/StEph.

## Finding

**Already done.** The View was already fixed in a prior round:

```rust
// src/Chap41/AVLTreeSetMtEph.rs:83-86
impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEph<T> {
    type V = Set<<T as View>::V>;
    open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
}
```

The trait also already constrains `View<V = Set<T::V>>` (line 108).

## Verification

| # | Chap | File | Verified | Errors | Holes |
|---|------|------|----------|--------|-------|
| 1 | 41 | AVLTreeSetMtEph.rs | 2031 (isolate) | 0 | 0 actionable |

## Callers

Only reference outside the file is a comment in `AVLTreeSetMtPer.rs`. No Chap52/Chap53
files import AVLTreeSetMtEph. No cascade needed.

## Hole Summary

- 0 actionable holes
- 2 accepted (iterator clone assume, reader predicate assume)
- 23 structural_false_positive RWLOCK_GHOST (inherent to Mt/RwLock pattern)
- 2 structural_false_positive UNSAFE_SEND_SYNC

## Steps Used: 1 of 20

No code changes needed — task was already complete.

---

# R101 Agent 2 (continued) — Iterator Views Seq<T> → Seq<T::V>

## Objective

Fix AVLTreeSetMtEph iterator ghost views from `Seq<T>` to `Seq<T::V>`.

## Changes

| # | Chap | File | Line | Before | After |
|---|------|------|------|--------|-------|
| 1 | 41 | AVLTreeSetMtEph.rs | 78 | `elements: Seq<T>` | `elements: Seq<T::V>` |
| 2 | 41 | AVLTreeSetMtEph.rs | 89 | `type V = (int, Seq<T>)` | `type V = (int, Seq<T::V>)` |
| 3 | 41 | AVLTreeSetMtEph.rs | 90 | `self.snapshot@` | `self.snapshot@.map_values(\|t: T\| t@)` |
| 4 | 41 | AVLTreeSetMtEph.rs | 96 | `type V = Seq<T>` | `type V = Seq<T::V>` |
| 5 | 41 | AVLTreeSetMtEph.rs | 520 | `type Item = T` | `type Item = T::V` |
| 6 | 41 | AVLTreeSetMtEph.rs | 544 | `Option<T>` | `Option<T::V>` |
| 7 | 41 | AVLTreeSetMtEph.rs | 495 | `element == old_seq[i]` | `element@ == old_seq[i]` |
| 8 | 41 | AVLTreeSetMtEph.rs | 505 | `item == old(self)@.1[...]` | `item@ == old(self)@.1[...]` |

## Verification

| Step | Result |
|------|--------|
| Isolate Chap41 | 2031 verified, 0 errors |
| RTT | 3083 passed |

## Steps Used: 2 of 15
