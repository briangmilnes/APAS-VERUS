# R71 Agent 4 Report: Iterator Standard Review

## Scope

Reviewed 14 files across Chap43, Chap49, and Chap50 for compliance with the
iterator standard (`src/standards/iterators_standard.rs`).

## Summary Table

| # | Chap | File | Type | Verified Iter | Components | Missing | Issues |
|---|------|------|------|---------------|------------|---------|--------|
| 1 | 43 | OrderedTableStEph.rs | collection | partial | 1,2,3,4,9 | 5,6,7,8,10 | assume in next; weak ensures |
| 2 | 43 | OrderedTableStPer.rs | collection | partial | 1,2,3,4,9,10 | 5,6,7,8 | assume in next; weak ensures |
| 3 | 49 | MinEditDistStEph.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 4 | 49 | MinEditDistStPer.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 5 | 49 | SubsetSumStEph.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 6 | 49 | SubsetSumStPer.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 7 | 50 | MatrixChainMtEph.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 8 | 50 | MatrixChainMtPer.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 9 | 50 | MatrixChainStEph.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 10 | 50 | MatrixChainStPer.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 11 | 50 | OptBinSearchTreeMtEph.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 12 | 50 | OptBinSearchTreeMtPer.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 13 | 50 | OptBinSearchTreeStEph.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |
| 14 | 50 | OptBinSearchTreeStPer.rs | algorithm | none | N/A | N/A | unverified IntoIterator only |

**Result: 0 files fully comply. 2 files need work. 12 files are algorithm modules where the iterator standard does not apply.**

## Detailed Findings

### Chap43 OrderedTableStEph.rs — 5 of 10 components present

**Present:** Custom iterator struct (1), View for iterator (2), iter_invariant (3),
Iterator::next with ensures (4), iter() method (9).

**Missing:** Ghost iterator struct (5), ForLoopGhostIteratorNew (6),
ForLoopGhostIterator with 6 spec fns (7), View for ghost iterator (8),
IntoIterator for &Self (10).

**Issues:**

1. **assume in next()**: `assume(iter_invariant(self))` in Iterator::next body.
   This proof hole would be eliminated if the ghost iterator protocol (components
   5-8) were implemented.
2. **Weak iter() ensures**: `it@.1.len() == self.tree@.len()` (length only).
   Standard requires `it@.1 == self.data@` (exact sequence equality). Callers
   cannot reason about which elements the iterator produces.
3. **Owned Item type**: `type Item = Pair<K, V>` instead of standard `&'a T`.
   Structural choice: BST-backed storage materializes sorted entries into an owned
   ArraySeqStPerS. No underlying slice to borrow from.
4. **No lifetime on iterator struct**: `OrderedTableStEphIter<K, V>` has no `'a`
   parameter. Follows from the owned-data design.
5. **Missing Debug/Display** for OrderedTableStEphIter outside verus!.
6. **Missing IntoIterator for &Self** (component 10) — `for x in &table` does not compile.

### Chap43 OrderedTableStPer.rs — 6 of 10 components present

**Present:** Custom iterator struct (1), View for iterator (2), iter_invariant (3),
Iterator::next with ensures (4), iter() method (9), IntoIterator for &Self (10).

**Missing:** Ghost iterator struct (5), ForLoopGhostIteratorNew (6),
ForLoopGhostIterator with 6 spec fns (7), View for ghost iterator (8).

**Issues:**

1. **assume in next()**: `assume(iter_invariant(self))` in Iterator::next body.
   Same hole as StEph.
2. **Weak iter() ensures**: `it@.1.len() == self.tree@.len()` (length only).
   Same weakness as StEph.
3. **Owned Item type**: `type Item = Pair<K, V>` instead of `&'a T`. Same
   structural choice as StEph.
4. **No lifetime on iterator struct**: Same as StEph.
5. **Missing Debug/Display** for OrderedTableStPerIter outside verus!.
6. **No IntoIterator for Self (consuming)**: Not present (optional but noted).

### Chap49 — All 4 files: algorithm modules, no collection to iterate

MinEditDistStEph, MinEditDistStPer, SubsetSumStEph, SubsetSumStPer are all DP
algorithm solvers. They do not store user-facing collections. The iterator
standard does not apply.

Each file provides unverified `IntoIterator` impls outside `verus!` as convenience
for runtime tests:
- StEph files: `IntoIterator for Self`, `&Self`, `&mut Self`
- StPer files: `IntoIterator for Self`, `&Self`
- No `assume`, `accept`, `unsafe`, or `external_body` in any iterator code.

### Chap50 MatrixChain — All 4 files: algorithm modules, no collection to iterate

MatrixChainStEph, MatrixChainStPer, MatrixChainMtEph, MatrixChainMtPer are DP
algorithm modules for optimal matrix chain parenthesization. The iterator standard
does not apply.

Each file provides unverified `IntoIterator` impls outside `verus!` over the
dimensions vector:
- StEph/MtEph: `IntoIterator for Self`, `&Self`, `&mut Self`
- StPer/MtPer: `IntoIterator for Self`, `&Self`
- Mt variants clone from behind Arc/RwLock as expected.
- No `assume`, `accept`, `unsafe`, or `external_body` in any iterator code.

### Chap50 OptBinSearchTree — All 4 files: algorithm modules, no collection to iterate

OptBinSearchTreeStEph, OptBinSearchTreeStPer, OptBinSearchTreeMtEph,
OptBinSearchTreeMtPer are DP algorithm modules for optimal BST cost computation.
The iterator standard does not apply.

Each file provides unverified `IntoIterator` impls outside `verus!` over the
keys vector:
- StEph/MtEph: `IntoIterator for Self`, `&Self`, `&mut Self`
- StPer/MtPer: `IntoIterator for Self`, `&Self`
- MtPer consuming variant uses Arc::try_unwrap optimization.
- No `assume`, `accept`, `unsafe`, or `external_body` in any iterator code.

## Component-Level Summary for Chap43

| # | Component | Standard | StEph | StPer |
|---|-----------|----------|-------|-------|
| 1 | Custom iterator struct | required | present | present |
| 2 | View for iterator | (int, Seq<T>) | present | present |
| 3 | iter_invariant | 0<=pos<=len | present | present |
| 4 | Iterator::next | two-arm ensures | present (+ assume) | present (+ assume) |
| 5 | Ghost iterator struct | pos, elements, phantom | **MISSING** | **MISSING** |
| 6 | ForLoopGhostIteratorNew | ghost_iter() | **MISSING** | **MISSING** |
| 7 | ForLoopGhostIterator | 6 spec fns | **MISSING** | **MISSING** |
| 8 | View for ghost iterator | elements.take(pos) | **MISSING** | **MISSING** |
| 9 | iter() method | pos=0, seq=data | present (weak ensures) | present (weak ensures) |
| 10 | IntoIterator for &Self | for x in &coll | **MISSING** | present |

## Actionable Work

Only the 2 Chap43 collection files need iterator standard work:

1. **OrderedTableStEph.rs**: Add components 5-8, 10. Strengthen iter() ensures.
   Remove assume from next(). Add Debug/Display.
2. **OrderedTableStPer.rs**: Add components 5-8. Strengthen iter() ensures.
   Remove assume from next(). Add Debug/Display.

The structural challenge is that both files use an owned-data iterator (no
underlying slice), so the ghost iterator will need to track the materialized
sequence rather than delegating to a std::slice::Iter ghost. The assume in
next() exists precisely because the ghost protocol is missing.

The 12 algorithm files (Chap49, Chap50) are correctly exempt from the iterator
standard. Their unverified IntoIterator impls are appropriate for test convenience.
