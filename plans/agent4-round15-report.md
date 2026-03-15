# Agent 4 — Round 15 Report

## Summary

- **Holes before**: 149
- **Holes after**: 147 (-2)
- **Verified**: 4079 (was 4078, +1 new proof fn)
- **Errors**: 0
- **Chapters closed**: 0
- **Commit**: (pending)

## Holes Before/After by File

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 37 | AVLTreeSeqStPer.rs | 1 | 0 | -1 | feq broadcast trigger for clone-view bridge |
| 2 | 41 | AVLTreeSetStEph.rs | 2 | 1 | -1 | lemma_wf_implies_len_bound (inductive proof) |
| 3 | 37 | AVLTreeSeq.rs | 1 | 1 | 0 | Blocked: Iterator::next() has no requires |
| 4 | 37 | AVLTreeSeqMtPer.rs | 2 | 2 | 0 | Blocked: parallel code (nested fn + ParaPair) |
| 5 | 37 | BSTSplayStEph.rs | 1 | 1 | 0 | Blocked: trivial_wf needs BST pres. through splay |
| 6 | 47 | ParaHashTableStEph.rs | 2 | 2 | 0 | Blocked: opaque Fn/hash (not probe fns) |
| 7 | 45 | BalancedTreePQ.rs | 1 | 1 | 0 | Blocked: external on filter/map with closures |
| 8 | 45 | BinaryHeapPQ.rs | 1 | 1 | 0 | Blocked: sorted proof needs heap invariant cascade |
| 9 | 41 | AVLTreeSetMtEph.rs | 9 | 9 | 0 | Blocked: view bridge (2), parallel (5), unsafe (2) |
| 10 | 41 | ArraySetEnumMtEph.rs | 1 | 1 | 0 | Blocked: missing closure requires in trait |

## Fixes Applied

### 1. AVLTreeSeqStPer.rs clone-view assume (Chap37)

**Problem**: `assume(val@ == a@[mid as int]@)` in `build_balanced_from_slice` after cloning.

**Fix**: Replaced with feq broadcast trigger pattern:
```rust
assert(obeys_feq_full_trigger::<T>());
assert(cloned(a@[mid as int], val));
```
The broadcast group `group_feq_axioms` provides `axiom_obeys_feq_full` (triggered by
`obeys_feq_full_trigger`) and `axiom_cloned_implies_eq_owned` (triggered by `cloned`),
which together prove clone preserves view equality.

### 2. AVLTreeSetStEph.rs delete vec-length assume (Chap41)

**Problem**: `assume(result_vec@.len() < usize::MAX)` in `delete` — needed for `from_vec`.

**Fix**: Added inductive proof fn `lemma_wf_implies_len_bound` showing that under tree wf,
`spec_inorder(link).len() < usize::MAX`. Since delete produces a subsequence of the original
inorder traversal, the result length is bounded by the original, which is < usize::MAX.

## Remaining Holes — Blocker Analysis

### Chap37 (4 holes, was 5)

1. **AVLTreeSeq.rs:1117** (external_body on Iterator::next): `std::iter::Iterator::next()`
   has no `requires` clause. Internal call to `nth()` needs wf + bounds check. Verus doesn't
   support requires on trait methods from std. **Blocked permanently by Verus design.**

2. **AVLTreeSeqMtPer.rs:508** (external_body on build_balanced_from_slice): Uses nested
   `fn rec` + `ParaPair!` macro for parallel tree construction. Verus cannot verify nested
   functions or macro-generated parallel code. **Blocked by Verus parallel limitations.**

3. **AVLTreeSeqMtPer.rs:623** (external_body on subseq_copy): Uses `spawn`/`wait` with
   `Arc<Mutex<Option<T>>>` slot pattern. **Blocked by Verus parallel limitations.**

4. **BSTSplayStEph.rs:464** (trivial_wf `{ true }`): Strengthening to `spec_is_bst_link`
   requires proving BST preservation through all 6 splay rotation cases (zig, zig-zig,
   zig-zag, zag, zag-zag, zag-zig) AND through bst_insert. ALL helper functions currently
   have `ensures true`. Estimated 4-8 iterations, 120+ lines of proof assertions. **Blocked
   by massive proof cascade. Will not close chapter regardless (3 other holes blocked).**

### Chap47 (2 holes)

1. **ParaHashTableStEph.rs:56** (external_body on call_hash_fn): Wraps opaque `Fn(&Key, usize)
   -> usize` closure. Verus cannot reason about opaque Fn trait. **Blocked by Verus.**

2. **ParaHashTableStEph.rs:84** (external_body on compute_second_hash): Uses `std::hash::Hash`
   and `DefaultHasher`, opaque to Verus. **Blocked by Verus.**

Note: The prompt suggested these were probe functions — they are NOT. The probe functions
(linear, quadratic) were already fixed in Round 13. These are infrastructure helpers.

### Chap45 (2 effective holes, excluding Example45_2)

1. **BalancedTreePQ.rs:557** (external on filter/map impl): Generic closure-based filter/map
   with `#[verifier::external]` on entire impl block. **Blocked by closure verification.**

2. **BinaryHeapPQ.rs:947** (assume sorted in extract_all_sorted): Proving sortedness requires:
   (a) `spec_leq_view` is `uninterp` — no connection to `TotalOrder::le`;
   (b) `delete_min` ensures multiset preservation but NOT minimality or heap preservation;
   (c) Would need heap invariant through bubble_down → heapify → delete_min → extract_all.
   Estimated 3-5 iterations of multi-function proof cascade. **Blocked by uninterp spec
   and missing ensures cascade.**

### Chap41 (11 holes, excluding MtPer/Example)

1. **AVLTreeSetStEph.rs:865** (assume in insert): `new_vec@.len() < usize::MAX` after insert.
   Tree wf gives n < usize::MAX, but after insert new length is n+1. Need n+1 < usize::MAX
   which requires n < usize::MAX - 1. Wf doesn't guarantee this. Would need trait API change
   to add `self.spec_size() + 1 < usize::MAX` to insert requires. **Blocked by API design.**

2. **AVLTreeSetMtEph.rs** (9 holes): 2 assume (view bridge — ghost field not connected to
   locked state), 5 external_body (parallel filter/intersection/difference/union/to_seq —
   nested fns, ParaPair, sort/dedup), 2 unsafe_impl (Send/Sync — structural). **All blocked.**

3. **ArraySetEnumMtEph.rs** (1 hole): external_body on filter. Missing closure requires
   `f.requires((i,))` in trait signature. **Blocked by trait design.**

## Techniques Used

1. **feq broadcast trigger pattern**: `assert(obeys_feq_full_trigger::<T>())` +
   `assert(cloned(old, new))` to bridge clone-view equality through broadcast axioms.
   Previously used in Round 13 for Chap47 probe functions.

2. **Inductive wf-implies-bound lemma**: Recursive proof matching tree structure showing
   that wf constraint `left_size + right_size + 1 < usize::MAX` at each node implies
   total inorder length < usize::MAX.

## Assessment

This was a low-yield round. The 2 fixes that were tractable (clone-view bridge and
delete vec-length bound) were found and applied. All remaining holes across Chap37,
Chap47, Chap45, and Chap41 are blocked by fundamental Verus limitations:

- **Iterator::next() requires** (Verus can't add requires to std trait methods)
- **Parallel code** (nested fns, ParaPair!, spawn/wait not verifiable)
- **Opaque Fn/hash** (Verus can't reason about std Fn trait or std::hash)
- **Uninterp specs** (spec_leq_view disconnected from TotalOrder::le)
- **Massive proof cascades** (splay BST preservation, heap sort invariant)
- **API design constraints** (insert requires, closure requires in traits)

No chapters were closed. Chap37 cannot be closed without resolving the iterator and
parallel blockers. Chap47 cannot be closed without resolving the opaque Fn/hash blockers.
Chap45 cannot be closed without resolving the closure and uninterp spec blockers.
