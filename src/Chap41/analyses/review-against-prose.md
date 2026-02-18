<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 41 — Sets ADT: Review Against Prose

- **Date**: 2026-02-13
- **Reviewer**: Claude-Opus-4.6
- **Prose source**: `prompts/Chap41.txt`
- **Source files**: 7 files in `src/Chap41/`
- **Test files**: 1 RTT file in `tests/Chap41/`
- **PTT files**: None

## Phase 1: Inventory (tool-generated)

Run: `veracity-review-module-fn-impls -f src/Chap41/*.rs`

88 functions extracted across 7 modules. All functions are outside `verus!` blocks (no formal specs). Three modules have minimal `verus!` blocks containing only `View` impls (`AVLTreeSetStEph`, `AVLTreeSetStPer`, `AVLTreeSetMtPer`).

| # | Module | Functions | Inside verus! | Has Specs | Proof Holes |
|---|--------|-----------|:-------------:|:---------:|:-----------:|
| 1 | AVLTreeSetMtEph | 15 | 0 | 0 | 0 |
| 2 | AVLTreeSetMtPer | 16 | 0 | 0 | 0 |
| 3 | AVLTreeSetStEph | 13 | 0 | 0 | 0 |
| 4 | AVLTreeSetStPer | 13 | 0 | 0 | 0 |
| 5 | ArraySetEnumMtEph | 13 | 0 | 0 | 0 |
| 6 | ArraySetStEph | 13 | 0 | 0 | 0 |
| 7 | Example41_3 | 5 | 0 | 0 | 0 |

## Phase 2: Prose Inventory (manual)

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Data Type 41.1 (Sets) | SET ADT: type S representing power set of U (finite subsets), with size, toSeq, empty, singleton, fromSeq, filter, intersection, difference, union, find, delete, insert |
| 2 | Universe U | Elements that support equality; may also support hashing or total ordering |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Example 41.3 (fromSeq sequential) | `fromseq a = Seq.iterate Set.insert ∅ a` — work efficient but sequential |
| 2 | Example 41.3 (fromSeq parallel) | `fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩` — work efficient and parallel |

### Cost Specifications

| # | Item | Description |
|---|------|-------------|
| 1 | Cost Spec 41.3 (Arrays for Enumerable Sets) | Universe {0..u-1}, boolean array representation |
| 2 | Cost Spec 41.4 (Tree Sets) | Balanced BST representation, where n=max(\|a\|,\|b\|), m=min(\|a\|,\|b\|) |

### Cost Spec 41.3 (Enumerable Array Sets)

| # | Operation | Work | Span |
|---|-----------|------|------|
| 1 | size a | u | 1 |
| 2 | singleton x | u | 1 |
| 3 | toSeq a | u | 1 |
| 4 | filter f a | u + Σ W(f(x)) | 1 + max S(f(x)) |
| 5 | intersection a₁ a₂ | u | 1 |
| 6 | union a₁ a₂ | u | 1 |
| 7 | difference a₁ a₂ | u | 1 |
| 8 | find a e | 1 | 1 |
| 9 | insert a x | u | 1 |
| 10 | delete a x | u | 1 |

### Cost Spec 41.4 (Tree Sets)

| # | Operation | Work | Span |
|---|-----------|------|------|
| 1 | size a | 1 | 1 |
| 2 | singleton x | 1 | 1 |
| 3 | toSeq a | \|a\| | lg \|a\| |
| 4 | filter f a | Σ W(f(x)) | lg \|a\| + max S(f(x)) |
| 5 | intersection a b | m·lg(1+n/m) | lg(n) |
| 6 | union a b | m·lg(1+n/m) | lg(n) |
| 7 | difference a b | m·lg(1+n/m) | lg(n) |
| 8 | find a e | lg \|a\| | lg \|a\| |
| 9 | insert a x | lg \|a\| | lg \|a\| |
| 10 | delete a x | lg \|a\| | lg \|a\| |

### Theorems/Properties

| # | Item | Description |
|---|------|-------------|
| 1 | Set uniqueness | A set is a collection of distinct objects |
| 2 | Finite size restriction | S only consists of finite sized subsets; no complement operation |

### Exercises/Problems

None explicitly numbered for implementation beyond Example 41.1 test cases and Example 41.3.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

APAS cost annotations have been added to all trait declarations alongside existing `claude-4-sonet` annotations. Files updated:
- `ArraySetEnumMtEph.rs` — Cost Spec 41.3
- `AVLTreeSetStEph.rs` — Cost Spec 41.4
- `AVLTreeSetStPer.rs` — Cost Spec 41.4
- `AVLTreeSetMtEph.rs` — Cost Spec 41.4
- `AVLTreeSetMtPer.rs` — Cost Spec 41.4

`ArraySetStEph.rs` has no APAS cost spec counterpart (it's a sorted array, not the enumerable boolean array nor a tree). The existing `claude-4-sonet` costs remain.

### 3b. Implementation Fidelity

#### ArraySetStEph (Sorted Array Set — no direct APAS cost spec)

| # | Function | Prose Match | Notes |
|---|----------|:-----------:|-------|
| 1 | size | ✅ | Delegates to `elements.length()` |
| 2 | to_seq | ✅ | Returns clone of elements |
| 3 | empty | ✅ | Empty set |
| 4 | singleton | ✅ | Single-element set |
| 5 | from_seq | ⚠️ | Uses sequential loop with union (prose Example 41.3 suggests `Seq.reduce Set.union ∅`); sequential, not parallel reduce |
| 6 | filter | ✅ | Linear scan |
| 7 | intersection | ✅ | Linear scan with find |
| 8 | difference | ✅ | Linear scan with find |
| 9 | union | ✅ | Add all from self, then non-duplicates from other; sorts result |
| 10 | find | ⚠️ | Linear scan O(n), not binary search despite sorted storage |
| 11 | delete | ✅ | Rebuild without element |
| 12 | insert | ✅ | Find + append + sort |

#### ArraySetEnumMtEph (Boolean Array — Cost Spec 41.3)

| # | Function | Prose Match | Notes |
|---|----------|:-----------:|-------|
| 1 | new | ✅ | Initializes bit array of size u |
| 2 | size | ✅ | popcount — work is Θ(u/w) vs APAS u; bit-packing optimization noted in prose |
| 3 | to_seq | ✅ | Scans bits, collects set members |
| 4 | empty | ✅ | All zeros |
| 5 | singleton | ✅ | Single bit set |
| 6 | from_seq | ✅ | Set bits for each element |
| 7 | filter | ⚠️ | Spawns one thread per element but joins immediately (sequential spawn/join loop, not true parallel fork-join). APAS Span = 1 + max S(f(x)); actual Span ≈ Σ S(f(x)) |
| 8 | intersection | ✅ | Bitwise AND loop — prose says u, impl uses u/w (bit-packing optimization) |
| 9 | difference | ✅ | Bitwise AND-NOT loop |
| 10 | union | ✅ | Bitwise OR loop |
| 11 | find | ✅ | O(1) bit lookup |
| 12 | delete | ✅ | O(1) bit clear |
| 13 | insert | ✅ | O(1) bit set |

#### AVLTreeSetStEph / AVLTreeSetStPer (Tree Set — Cost Spec 41.4)

Both implementations are structurally identical (ephemeral uses `&mut self`, persistent returns `Self`). Neither uses the underlying AVL tree's O(log n) operations directly. Instead:

| # | Function | Prose Match | Notes |
|---|----------|:-----------:|-------|
| 1 | size | ✅ | Delegates to `elements.length()` — O(1) |
| 2 | to_seq | ⚠️ | Copies elements one by one into a new sequence. Work Θ(n), Span Θ(n). APAS says Span lg \|a\| |
| 3 | empty | ✅ | Empty tree |
| 4 | singleton | ✅ | Single element |
| 5 | from_seq | ⚠️ | Sequential reduce with union (not parallel reduce). Work Θ(n²) due to n union calls each O(n) |
| 6 | filter | ⚠️ | Sequential scan with insert. Work Θ(n²) due to n inserts each rebuilding. APAS says Σ W(f(x)) |
| 7 | intersection | ⚠️ | Sequential scan with find+insert. Work Θ(n²). APAS says m·lg(1+n/m) |
| 8 | difference | ⚠️ | Same pattern as intersection |
| 9 | union | ⚠️ | Sequential scan inserting all elements. Work Θ(n²). APAS says m·lg(1+n/m) |
| 10 | find | ⚠️ | Linear scan O(n). APAS says lg \|a\|. Does not use AVL tree search |
| 11 | delete | ⚠️ | Rebuild without element. Work Θ(n). APAS says lg \|a\| |
| 12 | insert | ⚠️ | Find + rebuild + sort. Work Θ(n log n). APAS says lg \|a\| |

**Major deviation**: The AVL tree set implementations do not actually use AVL tree operations (balanced insert/delete/search). They treat the AVL tree as a flat sequence and use linear scans + rebuilds. This makes all operations significantly more expensive than Cost Spec 41.4.

#### AVLTreeSetMtEph (Multi-threaded Ephemeral Tree Set)

| # | Function | Prose Match | Notes |
|---|----------|:-----------:|-------|
| 1 | size | ✅ | Lock + delegate |
| 2 | to_seq | ✅ | Lock + delegate |
| 3 | empty | ✅ | Arc<Mutex<StEph>> |
| 4 | singleton | ✅ | Wraps StEph singleton |
| 5 | from_seq | ✅ | Wraps StEph from_seq |
| 6 | filter | ✅ parallel | Extract-parallelize-rebuild with ParaPair! divide-and-conquer |
| 7 | intersection | ✅ parallel | Extract-parallelize-rebuild with ParaPair! |
| 8 | difference | ✅ parallel | Delegates to filter |
| 9 | union | ⚠️ sequential | "Simple merge (sequential to avoid thread explosion)" — sort + dedup |
| 10 | find | ✅ | Lock + delegate (linear scan) |
| 11 | delete | ✅ | Lock + delegate |
| 12 | insert | ✅ | Lock + delegate |

#### AVLTreeSetMtPer (Multi-threaded Persistent Tree Set)

| # | Function | Prose Match | Notes |
|---|----------|:-----------:|-------|
| 1 | size | ✅ | Delegate |
| 2 | to_seq | ✅ | Clone |
| 3 | empty | ✅ | Empty sequence |
| 4 | singleton | ✅ | Single element |
| 5 | from_seq | ✅ parallel | ParaPair! parallel merge sort + dedup |
| 6 | filter | ✅ parallel | ParaPair! divide-and-conquer |
| 7 | intersection | ✅ parallel | ParaPair! divide-and-conquer |
| 8 | difference | ✅ parallel | Delegates to filter |
| 9 | union | ✅ parallel | ParaPair! divide-and-conquer |
| 10 | find | ✅ | Binary search O(log n) — correctly implements APAS cost |
| 11 | delete | ✅ parallel | Delegates to filter |
| 12 | insert | ✅ | find + push + parallel from_seq |

### 3c. Spec Fidelity

No functions have `requires`/`ensures`. All 88 entries classified as `none`. The code is entirely outside `verus!` blocks (except trivial `View` impls). There is no formal specification to compare against the prose.

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

#### AVLTreeSetMtEph

| # | Function | Classification | Mechanism |
|---|----------|---------------|-----------|
| 1 | size | Delegating | Lock + StEph |
| 2 | to_seq | Delegating | Lock + StEph |
| 3 | empty | Sequential | Constructor |
| 4 | singleton | Sequential | Constructor |
| 5 | from_seq | Delegating | Lock + StEph (sequential) |
| 6 | filter | **Parallel** | ParaPair! divide-and-conquer |
| 7 | intersection | **Parallel** | ParaPair! divide-and-conquer |
| 8 | difference | **Parallel** | Via parallel filter |
| 9 | union | Sequential | sort + dedup merge |
| 10 | find | Delegating | Lock + StEph (linear scan) |
| 11 | delete | Delegating | Lock + StEph |
| 12 | insert | Delegating | Lock + StEph |

#### AVLTreeSetMtPer

| # | Function | Classification | Mechanism |
|---|----------|---------------|-----------|
| 1 | size | Sequential | Delegate to length() |
| 2 | to_seq | Sequential | Clone |
| 3 | empty | Sequential | Constructor |
| 4 | singleton | Sequential | Constructor |
| 5 | from_seq | **Parallel** | ParaPair! merge sort |
| 6 | filter | **Parallel** | ParaPair! divide-and-conquer |
| 7 | intersection | **Parallel** | ParaPair! divide-and-conquer |
| 8 | difference | **Parallel** | Via parallel filter |
| 9 | union | **Parallel** | ParaPair! divide-and-conquer |
| 10 | find | Sequential | Binary search |
| 11 | delete | **Parallel** | Via parallel filter |
| 12 | insert | **Parallel** | find + parallel from_seq |

#### ArraySetEnumMtEph

| # | Function | Classification | Mechanism |
|---|----------|---------------|-----------|
| 1 | new | Sequential | Constructor |
| 2 | size | Sequential | popcount |
| 3 | to_seq | Sequential | Scan bits |
| 4 | empty | Sequential | Constructor |
| 5 | singleton | Sequential | Constructor |
| 6 | from_seq | Sequential | Scan + set bits |
| 7 | filter | ⚠️ Pseudo-parallel | Spawns threads per element but joins immediately in a loop |
| 8 | intersection | Sequential | Bitwise loop |
| 9 | difference | Sequential | Bitwise loop |
| 10 | union | Sequential | Bitwise loop |
| 11 | find | Sequential | O(1) lookup |
| 12 | delete | Sequential | O(1) clear |
| 13 | insert | Sequential | O(1) set |

### 4b. Span Audit

| # | Function | APAS Span | Annotated Span | Actual Span | Match? |
|---|----------|-----------|----------------|-------------|:------:|
| 1 | MtEph.filter | lg\|a\|+max S(f(x)) | Θ(log n) | Θ(log n) | ✅ |
| 2 | MtEph.intersection | lg(n) | Θ(log(m+n)) | Θ(log(m+n)) | ✅ |
| 3 | MtEph.union | lg(n) | Θ(log(m+n)) | Θ(n+m) sequential | ❌ |
| 4 | MtPer.filter | lg\|a\|+max S(f(x)) | Θ(log n) | Θ(log n) | ✅ |
| 5 | MtPer.intersection | lg(n) | Θ(log(m+n)) | Θ(log(m+n)) | ✅ |
| 6 | MtPer.union | lg(n) | Θ(log(m+n)) | Θ(log(m+n)) | ✅ |
| 7 | MtPer.from_seq | — | Θ(log n) | Θ(log² n) | ⚠️ |
| 8 | EnumMtEph.filter | 1+max S(f(x)) | Θ(log u) | Θ(Σ S(f(x))) | ❌ |

### 4c. Parallelism Gap Table

| # | Module | Function | APAS Span | Actual | Parallel? | Notes |
|---|--------|----------|-----------|--------|:---------:|-------|
| 1 | MtEph | filter | lg n | Θ(log n) | ✅ | ParaPair! divide-and-conquer |
| 2 | MtEph | intersection | lg n | Θ(log(m+n)) | ✅ | ParaPair! |
| 3 | MtEph | difference | lg n | Θ(log n) | ✅ | Via parallel filter |
| 4 | MtEph | union | lg n | Θ(n+m) | ❌ seq | Sequential sort+dedup; gap |
| 5 | MtPer | filter | lg n | Θ(log n) | ✅ | ParaPair! |
| 6 | MtPer | intersection | lg n | Θ(log(m+n)) | ✅ | ParaPair! |
| 7 | MtPer | difference | lg n | Θ(log n) | ✅ | Via parallel filter |
| 8 | MtPer | union | lg n | Θ(log(m+n)) | ✅ | ParaPair! |
| 9 | MtPer | from_seq | — | Θ(log n) | ✅ | Parallel merge sort |
| 10 | EnumMtEph | filter | 1+max S(f) | Θ(Σ S(f)) | ⚠️ | Spawn-per-element with sequential join loop |

## Phase 5: Runtime Test Review

### 5a. Coverage Check

Only one test file exists: `tests/Chap41/TestExample41_3.rs`

| # | Source Module | Test File | Status |
|---|-------------|-----------|--------|
| 1 | ArraySetStEph | `tests/Chap41/TestExample41_3.rs` (partial) | Tested via Example41_3 |
| 2 | ArraySetEnumMtEph | — | **Missing** |
| 3 | AVLTreeSetStEph | `tests/Chap41/TestExample41_3.rs` (partial) | Tested via Example41_3 |
| 4 | AVLTreeSetStPer | — | **Missing** |
| 5 | AVLTreeSetMtEph | — | **Missing** |
| 6 | AVLTreeSetMtPer | — | **Missing** |
| 7 | Example41_3 | `tests/Chap41/TestExample41_3.rs` | 4 tests |

### 5b. Test Quality

The existing test file exercises the Example 41.1 cases from the prose:
- ✅ `|{a,b,c}| = 3`
- ✅ `{x ∈ {4,11,2,6} | x < 7} = {4,2,6}`
- ✅ `find {6,2,9,11,8} 4 = false`
- ✅ `{2,7,8,11} ∪ {7,9,11,14,17} = {2,7,8,9,11,14,17}`
- ✅ `toSeq {2,7,8,11}` has 4 elements
- ✅ `fromSeq ⟨2,7,2,8,11,2⟩ = {8,2,11,7}`
- ✅ Example 41.3 fromSeq demonstration
- ✅ Additional intersection, difference, delete, insert tests

Tests cover happy paths and basic edge cases. No tests for empty sets, very large sets, or performance characteristics.

### 5c. Missing Tests

| # | Priority | Module | Reason |
|---|----------|--------|--------|
| 1 | High | ArraySetEnumMtEph | Only Mt module with parallelism; no tests at all |
| 2 | High | AVLTreeSetMtPer | Parallel operations with complex ParaPair! patterns; no tests |
| 3 | High | AVLTreeSetMtEph | Parallel operations; no tests |
| 4 | Medium | AVLTreeSetStPer | Persistent variant; no tests |

## Phase 6: Proof-Time Test (PTT) Review

No PTTs exist for Chap41. No modules have iterators, verified loops, or are inside `verus!` blocks. **No PTTs are needed** — all code is unverified exec code outside `verus!`.

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Iteration: `iterate f x a = Sequence.iterate f (toSeq a)` | Not implemented as a separate function |
| 2 | Reduction: `reduce f x a = Sequence.reduce f (toSeq a)` | Not implemented as a separate function |
| 3 | Hash-table based set implementation | Not implemented (only array and tree variants) |

### Code With No Prose Counterpart

| # | Item | Purpose |
|---|------|---------|
| 1 | `ArraySetStEph` | Sorted-array set; APAS discusses enumerable (boolean) arrays and trees, not sorted arrays |
| 2 | `AVLTreeSetMtEph` Arc<Mutex<>> wrapper | Thread-safety infrastructure |
| 3 | `AVLTreeSetMtPer` Ord/PartialOrd impls | Required by `OrderedTableMtPer` caller |
| 4 | All `*Lit!` macros | Convenience constructors |
| 5 | `View` impls in `verus!` blocks | Verus scaffolding (trivial identity views) |
| 6 | `parallel_filter`, `parallel_intersect`, `parallel_sort` | Internal helper functions for Mt parallelism |

## Phase 8: Table of Contents Review

### TOC Presence

| # | File | TOC Present? | Section Headers? |
|---|------|:------------:|:----------------:|
| 1 | ArraySetStEph.rs | ❌ No | ❌ No |
| 2 | ArraySetEnumMtEph.rs | ❌ No | ❌ No |
| 3 | AVLTreeSetStEph.rs | ❌ No | ❌ No |
| 4 | AVLTreeSetStPer.rs | ❌ No | ❌ No |
| 5 | AVLTreeSetMtEph.rs | ❌ No | ❌ No |
| 6 | AVLTreeSetMtPer.rs | ❌ No | ❌ No |
| 7 | Example41_3.rs | ❌ No | ❌ No |

No files follow the TOC standard. All files are pre-verusification exec code without section organization.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | ArraySetStEph | ❌ out | ❌ out | ❌ out | - | - | ❌ out | - | ✅ out | - |
| 2 | ArraySetEnumMtEph | ❌ out | ❌ out | - | - | - | - | - | ✅ out | - |
| 3 | AVLTreeSetStEph | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | View ✅ in |
| 4 | AVLTreeSetStPer | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | View ✅ in |
| 5 | AVLTreeSetMtEph | ❌ out | - | ❌ out | - | - | ❌ out | ❌ out | ✅ out | - |
| 6 | AVLTreeSetMtPer | ❌ out | ❌ out | ❌ out | - | - | ❌ out | ❌ out | ✅ out | View ✅ in, Ord ❌ out |
| 7 | Example41_3 | - | - | - | - | - | - | - | - | - |

All derive impls (Clone, PartialEq, Eq, Default) are outside `verus!` without specs. Debug/Display correctly outside. Macros correctly outside.

## Proof Holes Summary

```
veracity-review-proof-holes output:

✓ AVLTreeSetMtEph.rs
✓ AVLTreeSetMtPer.rs
✓ AVLTreeSetStEph.rs
✓ AVLTreeSetStPer.rs
✓ ArraySetEnumMtEph.rs
✓ ArraySetStEph.rs
✓ Example41_3.rs

SUMMARY
  Modules: 7 clean, 0 holed, 7 total
  Proof Functions: 0 clean, 0 holed, 0 total
  Holes Found: 0 total
```

No proof holes — because there are no proofs. All code is unverified exec code outside `verus!`.

## Spec Strength Summary

| Classification | Count |
|---------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 88 |

All 88 functions have no formal specifications (`requires`/`ensures`). Chapter 41 is entirely unverified.

## Overall Assessment

Chapter 41 implements the Sets ADT from the APAS textbook with six module variants covering all four quadrants (St/Mt × Eph/Per) plus an enumerable bit-array variant and an example module. The implementations are **functionally correct** (tests pass) but **entirely unverified** (no Verus specs or proofs).

### Key Findings

1. **No formal verification**: 0/88 functions have specs. All code is outside `verus!` blocks.

2. **Algorithmic inefficiency in AVL tree variants**: The `AVLTreeSetStEph` and `AVLTreeSetStPer` implementations do not use the AVL tree's O(log n) operations. They treat the backing AVL tree sequence as a flat array, using linear scans for `find` and O(n) rebuilds for `insert`/`delete`. This makes most operations O(n) or O(n²) instead of the O(log n) claimed by Cost Spec 41.4.

3. **`AVLTreeSetMtPer` is the best variant**: Uses binary search for `find` (O(log n)), parallel merge sort for `from_seq`, and ParaPair! divide-and-conquer for bulk operations. This most closely matches APAS cost expectations.

4. **`AVLTreeSetMtEph.union` is sequential**: While filter, intersection, and difference are parallel, union uses a sequential sort+dedup merge "to avoid thread explosion."

5. **`ArraySetEnumMtEph.filter` is pseudo-parallel**: Spawns one thread per element but joins immediately in a sequential loop, so actual span is Σ S(f(x)) not 1 + max S(f(x)).

6. **Test coverage is thin**: Only Example41_3 has runtime tests. The four main set implementations (ArraySetEnumMtEph, AVLTreeSetMtEph, AVLTreeSetMtPer, AVLTreeSetStPer) have no dedicated tests.

7. **No TOC standard compliance**: No file has the standard TOC layout.

### Recommendations

1. **Verusify** the core set types with proper specs (at minimum `ensures` on find, insert, delete, size).
2. **Fix AVL tree set operations** to use actual tree search/insert/delete instead of linear scans.
3. **Fix ArraySetEnumMtEph.filter** to use proper parallel fork-join (collect handles, then join all) instead of sequential spawn-join.
4. **Parallelize AVLTreeSetMtEph.union** with ParaPair! like the MtPer variant.
5. **Add runtime tests** for all set variants, especially the Mt ones.
6. **Add TOC headers** to all source files.
