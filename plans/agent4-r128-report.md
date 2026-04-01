# Agent 4 — R128 Report: Parallelize Chap41 AVLTreeSet Mt operations

## Summary

Restructured AVLTreeSetMtEph and AVLTreeSetMtPer to use BSTParaMtEph::ParamBST as
the backing store instead of wrapping AVLTreeSetStEph/StPer in Arc<RwLock>. This
gives parallel union/intersection/difference via BSTParaMtEph's fork-join D&C
implementations.

The key insight: the old architecture had two layers of indirection — Arc<RwLock>
around AVLTreeSetStEph, which itself delegates to BSTParaStEph (sequential BST).
The new architecture directly stores BSTParaMtEph::ParamBST, which already has
per-node RwLock locking and parallel D&C for union/intersect/difference.

Benefits:
- 3 operations parallelized per file (union, intersection, difference)
- Eliminated ~12 assume holes related to lock/ghost-shadow bridging
- Simpler struct layout (no outer Arc<RwLock>, no ghost_set_view)
- No regression in verification count or test results

## Results Table

| # | Chap | File | Function | Parallelized? | Old Span | New Span | Reason if not |
|---|------|------|----------|--------------|----------|----------|---------------|
| 1 | 41 | AVLTreeSetMtEph.rs | union | Yes | O(m·lg(1+n/m)) | O(lg² n) | |
| 2 | 41 | AVLTreeSetMtEph.rs | intersection | Yes | O(m·lg(1+n/m)) | O(lg² n) | |
| 3 | 41 | AVLTreeSetMtEph.rs | difference | Yes | O(m·lg(1+n/m)) | O(lg² n) | |
| 4 | 41 | AVLTreeSetMtEph.rs | filter | No | O(n + Σ W(f)) | O(n + Σ W(f)) | spec_fn not Send — Verus Ghost types cannot cross thread boundaries |
| 5 | 41 | AVLTreeSetMtEph.rs | to_seq | No | O(n) | O(n) | Sequential in-order traversal; no parallel in_order in BSTParaMtEph |
| 6 | 41 | AVLTreeSetMtEph.rs | from_seq | No | O(n lg n) | O(n lg n) | Sequential insert loop; parallel build needs D&C infrastructure |
| 7 | 41 | AVLTreeSetMtPer.rs | union | Yes | O(m·lg(1+n/m)) | O(lg² n) | |
| 8 | 41 | AVLTreeSetMtPer.rs | intersection | Yes | O(m·lg(1+n/m)) | O(lg² n) | |
| 9 | 41 | AVLTreeSetMtPer.rs | difference | Yes | O(m·lg(1+n/m)) | O(lg² n) | |
| 10 | 41 | AVLTreeSetMtPer.rs | filter | No | O(n + Σ W(f)) | O(n + Σ W(f)) | spec_fn not Send — same Verus limitation |
| 11 | 41 | AVLTreeSetMtPer.rs | to_seq | No | O(n) | O(n) | Same as MtEph |
| 12 | 41 | AVLTreeSetMtPer.rs | from_seq | No | O(n lg n) | O(n lg n) | Same as MtEph |
| 13 | 41 | ArraySetEnumMtEph.rs | size | No | O(u) | O(u) | PRAM Span O(1) not achievable with fork-join; annotation clarified |
| 14 | 41 | ArraySetEnumMtEph.rs | filter | No | O(u + Σ W(f)) | O(u + Σ W(f)) | PRAM Span O(1) not achievable with fork-join; annotation clarified |

## Verification

- Isolate Chap41: 2113 verified, 1 error (pre-existing rlimit in Chap37 AVLTreeSeqStEph)
- RTT: 3534 passed
- PTT: 221 passed

## Technique

The restructuring replaced the two-layer architecture:
```
AVLTreeSetMtEph { inner: Arc<RwLock<AVLTreeSetStEph<T>>> }
  → AVLTreeSetStEph { tree: BSTParaStEph::ParamBST<T> }
    → sequential union/intersect/difference
```

With direct delegation to the parallel BST:
```
AVLTreeSetMtEph { tree: BSTParaMtEph::ParamBST<T> }
  → parallel union/intersect/difference via ParaPair! fork-join
```

## Remaining DIFFERS (8 of 14)

- **filter** (2 files): Blocked by Verus limitation — Ghost<spec_fn> types are not Send,
  preventing parallel closures from capturing the spec predicate. This is fundamental to
  Verus's ghost type system.
- **to_seq** (2 files): BSTParaMtEph has no parallel in_order traversal. Would need a
  D&C flatten (left || right, concat) with O(n) concat cost.
- **from_seq** (2 files): Would need parallel D&C tree construction (sort + build halves
  in parallel, join). The BST join operation exists but the full pipeline isn't built.
- **ArraySetEnumMtEph size/filter** (2 functions): APAS Cost Spec 41.3 assumes PRAM
  with O(1) span. Fork-join D&C would give O(lg(u/w)) span, which is better than O(u)
  but cannot match O(1). Annotations updated to clarify the PRAM assumption.
