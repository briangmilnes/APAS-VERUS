<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 41 — Sets ADT: Review Against Prose

- **Date**: 2026-02-17
- **Reviewer**: Claude-Opus-4.6
- **Prose source**: `prompts/Chap41.txt`
- **Source files**: 7 files in `src/Chap41/`
- **Test files**: 7 RTT files in `tests/Chap41/`
- **PTT files**: None
- **Proof Holes**: 0 (all clean)

## Phase 2: Prose Inventory

### Data Type 41.1 (Sets ADT)

The prose defines a SET abstract data type over a universe U supporting equality, consisting of:

| # | Operation | Signature | Description |
|---|-----------|-----------|-------------|
| 1 | size | S → N | Number of elements |
| 2 | toSeq | S → Seq | Convert to sequence (arbitrary ordering) |
| 3 | empty | S | Empty set |
| 4 | singleton | U → S | Single-element set |
| 5 | fromSeq | Seq → S | Sequence to set (eliminate duplicates) |
| 6 | filter | (U → B) → S → S | Elements satisfying predicate |
| 7 | intersection | S → S → S | Set intersection |
| 8 | difference | S → S → S | Set difference |
| 9 | union | S → S → S | Set union |
| 10 | find | S → U → B | Membership test |
| 11 | delete | S → U → S | Remove element |
| 12 | insert | S → U → S | Add element |

### Cost Specifications

| # | Item | Description |
|---|------|-------------|
| 1 | Cost Spec 41.3 (Arrays for Enumerable Sets) | Universe {0..u-1}, boolean array representation |
| 2 | Cost Spec 41.4 (Tree Sets) | Balanced BST representation, n=max(\|a\|,\|b\|), m=min(\|a\|,\|b\|) |

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

### Examples and Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Example 41.1 | Concrete set operation examples (size, filter, find, union, toSeq, fromSeq) |
| 2 | Example 41.3 (sequential fromSeq) | `fromseq a = Seq.iterate Set.insert ∅ a` — work efficient, sequential |
| 3 | Example 41.3 (parallel fromSeq) | `fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩` — work efficient, parallel |

### Prose Properties

| # | Property | Description |
|---|----------|-------------|
| 1 | Set uniqueness | A set is a collection of distinct objects |
| 2 | Finite size restriction | S consists only of finite subsets; no complement operation |
| 3 | Equality requirement | Universe U must support equality |
| 4 | No map on sets | Map does not make sense for sets (can collapse elements) |
| 5 | Bulk vs singular | find/insert/delete are singular versions of intersection/union/difference |
| 6 | iterate/reduce | Defined via toSeq: `iterate f x a = Sequence.iterate f (toSeq a)` |

## Phase 3: Algorithmic Analysis

### 3a. Implementation Inventory

| # | Module | Backing Store | APAS Cost Spec | Quad | Functions |
|---|--------|--------------|:--------------:|------|-----------|
| 1 | ArraySetStEph | Sorted `ArraySeqStEphS<T>` | None (no APAS counterpart) | St/Eph | 12 |
| 2 | ArraySetEnumMtEph | `BitBox` (1-bit-per-element) | 41.3 | Mt/Eph | 13 |
| 3 | AVLTreeSetStEph | `AVLTreeSeqStEphS<T>` (as flat sequence) | 41.4 (claimed) | St/Eph | 12 |
| 4 | AVLTreeSetStPer | `AVLTreeSeqStPerS<T>` (as flat sequence) | 41.4 (claimed) | St/Per | 12 |
| 5 | AVLTreeSetMtEph | `Arc<Mutex<AVLTreeSetStEph<T>>>` | 41.4 (claimed) | Mt/Eph | 12 |
| 6 | AVLTreeSetMtPer | `AVLTreeSeqMtPerS<T>` (as sorted sequence) | 41.4 (claimed) | Mt/Per | 12 |
| 7 | Example41_3 | — | — | — | 4 |

### 3b. ArraySetStEph (Sorted Array — No APAS Cost Spec Counterpart)

This module uses a sorted `Vec`-backed sequence. APAS does not define a cost spec for sorted-array sets (only enumerable boolean arrays and trees). It has only `claude-4-sonet` cost annotations.

| # | Function | Prose Match | Actual Work | Actual Span | Notes |
|---|----------|:-----------:|-------------|-------------|-------|
| 1 | size | ✅ | Θ(1) | Θ(1) | Delegates to `elements.length()` |
| 2 | to_seq | ✅ | Θ(n) | Θ(n) | Clones element vector |
| 3 | empty | ✅ | Θ(1) | Θ(1) | Empty construction |
| 4 | singleton | ✅ | Θ(1) | Θ(1) | Single-element construction |
| 5 | from_seq | ⚠️ | Θ(n²) | Θ(n²) | Sequential loop with union; comment claims "parallel" but is sequential |
| 6 | filter | ✅ | Θ(n) | Θ(n) | Linear scan |
| 7 | intersection | ✅ | Θ(nm) | Θ(nm) | Linear scan of self with linear `find` on other |
| 8 | difference | ✅ | Θ(nm) | Θ(nm) | Linear scan of self with linear `find` on other |
| 9 | union | ✅ | Θ((n+m) log(n+m)) | Θ((n+m) log(n+m)) | Merge + sort |
| 10 | find | ⚠️ | Θ(n) | Θ(n) | Linear scan despite sorted storage; could use binary search |
| 11 | delete | ✅ | Θ(n) | Θ(n) | Rebuild without element |
| 12 | insert | ✅ | Θ(n log n) | Θ(n log n) | Find + append + sort |

**Issues:**
- `find` uses linear scan O(n) despite sorted backing; binary search would give O(log n).
- `from_seq` comment says "work efficient and parallel" but implementation is a sequential loop with union, yielding O(n²) work.

### 3c. ArraySetEnumMtEph (Boolean Bit Array — Cost Spec 41.3)

| # | Function | Prose Match | APAS Work | Actual Work | APAS Span | Actual Span | Notes |
|---|----------|:-----------:|-----------|-------------|-----------|-------------|-------|
| 1 | new | ✅ | — | Θ(u) | — | Θ(u) | Constructor |
| 2 | size | ✅ | u | Θ(u/w) | 1 | Θ(u/w) | `popcount`; bit-packing optimization noted in prose |
| 3 | to_seq | ✅ | u | Θ(u) | 1 | Θ(u) | Scans all bits |
| 4 | empty | ✅ | — | Θ(u) | — | Θ(u) | Delegates to `new` |
| 5 | singleton | ✅ | u | Θ(u) | 1 | Θ(1) | Allocates u-bit array + sets 1 bit |
| 6 | from_seq | ✅ | — | Θ(u + \|seq\|) | — | Θ(u + \|seq\|) | Alloc + scan |
| 7 | filter | ⚠️ | u + Σ W(f(x)) | Θ(u + Σ W(f(x))) | 1 + max S(f(x)) | **Θ(Σ S(f(x)))** | **Pseudo-parallel**: sequential spawn-join loop |
| 8 | intersection | ✅ | u | Θ(u) | 1 | Θ(u) | Element-wise AND (not bitwise) |
| 9 | difference | ✅ | u | Θ(u) | 1 | Θ(u) | Element-wise AND-NOT |
| 10 | union | ✅ | u | Θ(u) | 1 | Θ(u) | Element-wise OR |
| 11 | find | ✅ | 1 | Θ(1) | 1 | Θ(1) | Direct bit lookup |
| 12 | delete | ⚠️ | u | Θ(1) | 1 | Θ(1) | Ephemeral bit clear is O(1); APAS u assumes copy |
| 13 | insert | ⚠️ | u | Θ(1) | 1 | Θ(1) | Ephemeral bit set is O(1); APAS u assumes copy |

**Issues:**
- **filter is pseudo-parallel**: spawns one thread per element but calls `handle.join().unwrap()` immediately in the same loop iteration. This serializes all thread work. Actual span is Σ S(f(x)), not 1 + max S(f(x)). The APAS span guarantee is not achieved.
- **intersection/difference/union use element-wise loops** (`for i in 0..self.universe_size` with individual bit get/set) instead of `bitvec`'s built-in bitwise operations (`&`, `|`, `&!`). This misses the u/w bit-packing optimization.
- **insert/delete**: APAS says Work u (assuming array copy for persistence), but the ephemeral impl mutates in place — O(1). The APAS cost annotation on the trait is misleading for this ephemeral type.

### 3d. AVLTreeSetStEph / AVLTreeSetStPer (Tree Set — Claims Cost Spec 41.4)

Both implementations are structurally identical (ephemeral uses `&mut self`, persistent returns `Self`). **Neither uses actual AVL tree operations.** They treat the AVL tree sequence as a flat sorted array, using linear scans and full rebuilds.

| # | Function | APAS Work | Actual Work | APAS Span | Actual Span | Match? |
|---|----------|-----------|-------------|-----------|-------------|:------:|
| 1 | size | 1 | Θ(1) | 1 | Θ(1) | ✅ |
| 2 | to_seq | \|a\| | Θ(n) | lg \|a\| | Θ(n) | ⚠️ span |
| 3 | empty | 1 | Θ(1) | 1 | Θ(1) | ✅ |
| 4 | singleton | 1 | Θ(1) | 1 | Θ(1) | ✅ |
| 5 | from_seq | — | Θ(n²) | — | Θ(n²) | ❌ sequential reduce with union |
| 6 | filter | Σ W(f(x)) | Θ(n²) | lg\|a\|+max S(f(x)) | Θ(n²) | ❌ sequential scan with insert |
| 7 | intersection | m·lg(1+n/m) | Θ(nm) | lg(n) | Θ(nm) | ❌ linear find per element |
| 8 | difference | m·lg(1+n/m) | Θ(nm) | lg(n) | Θ(nm) | ❌ linear find per element |
| 9 | union | m·lg(1+n/m) | Θ(n²) | lg(n) | Θ(n²) | ❌ insert each element (linear per insert) |
| 10 | find | lg \|a\| | Θ(n) | lg \|a\| | Θ(n) | ❌ linear scan |
| 11 | delete | lg \|a\| | Θ(n) | lg \|a\| | Θ(n) | ❌ full rebuild |
| 12 | insert | lg \|a\| | Θ(n log n) | lg \|a\| | Θ(n log n) | ❌ rebuild + sort |

**Major deviation**: The trait annotations claim Cost Spec 41.4 (tree-based O(log n) operations), but the implementations do not use balanced tree search/insert/delete at all. Every operation that should be O(log n) is O(n) or worse. The AVL tree backing store is used purely as a sequence container, completely defeating the purpose of tree-based sets.

### 3e. AVLTreeSetMtEph (Multi-threaded Ephemeral — Claims Cost Spec 41.4)

Wraps `AVLTreeSetStEph` in `Arc<Mutex<...>>`. Adds parallel implementations for filter, intersection, and difference.

| # | Function | APAS Work | Actual Work | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------------|:---------:|-------|
| 1 | size | 1 | Θ(1) | 1 | Θ(1) | — | Lock + delegate |
| 2 | to_seq | \|a\| | Θ(n) | lg\|a\| | Θ(n) | — | Lock + delegate (sequential copy) |
| 3 | empty | 1 | Θ(1) | 1 | Θ(1) | — | Constructor |
| 4 | singleton | 1 | Θ(1) | 1 | Θ(1) | — | Constructor |
| 5 | from_seq | — | Θ(n²) | — | Θ(n²) | ❌ | Delegates to sequential StEph |
| 6 | filter | Σ W(f(x)) | Θ(n) | lg\|a\|+max S(f(x)) | Θ(log n) | ✅ | ParaPair! divide-and-conquer |
| 7 | intersection | m·lg(1+n/m) | Θ(nm) | lg(n) | Θ(log(n+m)) | ✅ | ParaPair! divide-and-conquer |
| 8 | difference | m·lg(1+n/m) | Θ(nm) | lg(n) | Θ(log n) | ✅ | Via parallel filter |
| 9 | union | m·lg(1+n/m) | Θ((n+m) log(n+m)) | lg(n) | Θ((n+m) log(n+m)) | ❌ | **Sequential** sort+dedup merge |
| 10 | find | lg\|a\| | Θ(n) | lg\|a\| | Θ(n) | — | Lock + linear scan via StEph |
| 11 | delete | lg\|a\| | Θ(n) | lg\|a\| | Θ(n) | — | Lock + rebuild via StEph |
| 12 | insert | lg\|a\| | Θ(n log n) | lg\|a\| | Θ(n log n) | — | Lock + rebuild+sort via StEph |

**Issues:**
- `union` is explicitly sequential ("Simple merge (sequential to avoid thread explosion)").
- Inherits all the O(n) problems from `AVLTreeSetStEph` for delegated operations (find, insert, delete).
- `parallel_filter` has a naming bug: `left_vals` holds elements from `mid..end` and `right_vals_final` holds `0..mid` (swapped names via `split_off`). Functionally correct since both halves are processed identically.
- `parallel_intersect` clones `other_vals` for each recursive branch — Work is Θ(nm) not Θ(n+m).

### 3f. AVLTreeSetMtPer (Multi-threaded Persistent — Claims Cost Spec 41.4)

The best-implemented variant. Uses sorted `AVLTreeSeqMtPerS<T>` as backing, binary search for `find`, and ParaPair! for parallel bulk operations.

| # | Function | APAS Work | Actual Work | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------------|:---------:|-------|
| 1 | size | 1 | Θ(1) | 1 | Θ(1) | — | `elements.length()` |
| 2 | to_seq | \|a\| | Θ(1) | lg\|a\| | Θ(1) | — | Clone (Arc-based, O(1)) |
| 3 | empty | 1 | Θ(1) | 1 | Θ(1) | — | Constructor |
| 4 | singleton | 1 | Θ(1) | 1 | Θ(1) | — | Constructor |
| 5 | from_seq | — | Θ(n log n) | — | Θ(log² n) | ✅ | Parallel merge sort + dedup |
| 6 | filter | Σ W(f(x)) | Θ(n log n) | lg\|a\|+max S(f(x)) | Θ(log n) | ✅ | ParaPair! divide-and-conquer |
| 7 | intersection | m·lg(1+n/m) | Θ(nm) | lg(n) | Θ(log(n+m)) | ✅ | ParaPair! divide-and-conquer |
| 8 | difference | m·lg(1+n/m) | — | lg(n) | Θ(log n) | ✅ | Via parallel filter |
| 9 | union | m·lg(1+n/m) | Θ((n+m) log(n+m)) | lg(n) | Θ(log(n+m)) | ✅ | ParaPair! divide-and-conquer |
| 10 | find | lg\|a\| | Θ(log n) | lg\|a\| | Θ(log n) | — | **Binary search** (correct!) |
| 11 | delete | lg\|a\| | Θ(n) | lg\|a\| | Θ(log n) | ✅ | Via parallel filter |
| 12 | insert | lg\|a\| | Θ(n log n) | lg\|a\| | Θ(log² n) | ✅ | find + push + parallel from_seq |

**Strengths:**
- `find` correctly uses binary search — the only variant that matches APAS's O(log n).
- `to_seq` is O(1) due to Arc-based clone.
- Bulk operations (filter, intersection, union) are genuinely parallel with ParaPair!.

**Issues:**
- `filter` merge step does sort+dedup after combining halves — Work Θ(n log n) instead of Θ(n). Should use a linear merge since both halves are already sorted and disjoint (filter preserves order and elements are unique).
- `intersection` clones `other` for each branch — full other set cloned at each recursion level.
- `union` causes **thread explosion** in practice — tests for union, intersection, difference, and filter are disabled in `TestAVLTreeSetMtPer.rs` (10 tests commented out). The recursive ParaPair! calls create O(log n) levels of nested parallelism exceeding available threads.
- `insert` rebuilds via `from_seq` (parallel sort of n+1 elements) — Θ(n log n) work instead of the APAS O(log n) for a balanced tree insert.
- `delete` via parallel filter is O(n) work instead of O(log n).

### 3g. Example41_3

| # | Function | Prose Reference | Match? | Notes |
|---|----------|----------------|:------:|-------|
| 1 | `example_41_1_array_set` | Example 41.1 | ✅ | Covers all 6 test cases from prose |
| 2 | `example_41_1_avl_set` | Example 41.1 | ✅ | Same cases with AVLTreeSetStEph |
| 3 | `example_41_3_from_seq_demonstration` | Example 41.3 | ✅ | Demonstrates singleton-map + union-reduce |
| 4 | `additional_set_operations` | — | ✅ | Extra coverage for intersection, difference, delete, insert |

### 3h. Spec Fidelity

No functions have `requires`/`ensures`. All code is outside `verus!` blocks (except trivial `View` impls in AVLTreeSetStEph, AVLTreeSetStPer, AVLTreeSetMtPer). There is no formal specification to compare against the prose.

| Classification | Count |
|---------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | ~88 |

## Phase 4: Parallelism Review

### 4a. Mt Function Classification

#### ArraySetEnumMtEph

| # | Function | Classification | Mechanism |
|---|----------|---------------|-----------|
| 1 | new | Sequential | Constructor |
| 2 | size | Sequential | `count_ones()` |
| 3 | to_seq | Sequential | Bit scan loop |
| 4 | empty | Sequential | Constructor |
| 5 | singleton | Sequential | Constructor |
| 6 | from_seq | Sequential | Scan + set bits |
| 7 | filter | ⚠️ **Pseudo-parallel** | Spawn-join per element (sequential join loop) |
| 8 | intersection | Sequential | Element-wise AND loop |
| 9 | difference | Sequential | Element-wise AND-NOT loop |
| 10 | union | Sequential | Element-wise OR loop |
| 11 | find | Sequential | O(1) bit lookup |
| 12 | delete | Sequential | O(1) bit clear |
| 13 | insert | Sequential | O(1) bit set |

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
| 9 | union | **Sequential** | Sort + dedup merge |
| 10 | find | Delegating | Lock + StEph (linear scan) |
| 11 | delete | Delegating | Lock + StEph |
| 12 | insert | Delegating | Lock + StEph |

#### AVLTreeSetMtPer

| # | Function | Classification | Mechanism |
|---|----------|---------------|-----------|
| 1 | size | Sequential | `elements.length()` |
| 2 | to_seq | Sequential | Arc clone |
| 3 | empty | Sequential | Constructor |
| 4 | singleton | Sequential | Constructor |
| 5 | from_seq | **Parallel** | ParaPair! merge sort |
| 6 | filter | **Parallel** | ParaPair! divide-and-conquer |
| 7 | intersection | **Parallel** | ParaPair! divide-and-conquer |
| 8 | difference | **Parallel** | Via parallel filter |
| 9 | union | **Parallel** | ParaPair! divide-and-conquer |
| 10 | find | Sequential | Binary search O(log n) |
| 11 | delete | **Parallel** | Via parallel filter |
| 12 | insert | **Parallel** | find + parallel from_seq |

### 4b. Span Audit

| # | Module.Function | APAS Span | Annotated Span | Actual Span | Match? |
|---|-----------------|-----------|----------------|-------------|:------:|
| 1 | EnumMtEph.filter | 1 + max S(f(x)) | Θ(log u) | Θ(Σ S(f(x))) | ❌ pseudo-parallel |
| 2 | MtEph.filter | lg\|a\| + max S(f(x)) | Θ(log n) | Θ(log n) | ✅ |
| 3 | MtEph.intersection | lg(n) | Θ(log(m+n)) | Θ(log(m+n)) | ✅ |
| 4 | MtEph.union | lg(n) | Θ(log(m+n)) | Θ((n+m) log(n+m)) | ❌ sequential |
| 5 | MtPer.filter | lg\|a\| + max S(f(x)) | Θ(log n) | Θ(log n) | ✅ |
| 6 | MtPer.intersection | lg(n) | Θ(log(m+n)) | Θ(log(m+n)) | ✅ |
| 7 | MtPer.union | lg(n) | Θ(log(m+n)) | Θ(log(m+n)) | ✅ (but thread explosion in practice) |
| 8 | MtPer.from_seq | — | Θ(log n) | Θ(log² n) | ⚠️ merge sort depth |

### 4c. Parallelism Gap Table

| # | Module | Function | APAS Span | Actual | Parallel? | Notes |
|---|--------|----------|-----------|--------|:---------:|-------|
| 1 | EnumMtEph | filter | 1 + max S(f) | Θ(Σ S(f)) | ❌ | Spawn-join per element in sequential loop |
| 2 | MtEph | filter | lg n | Θ(log n) | ✅ | ParaPair! divide-and-conquer |
| 3 | MtEph | intersection | lg n | Θ(log(m+n)) | ✅ | ParaPair! |
| 4 | MtEph | difference | lg n | Θ(log n) | ✅ | Via parallel filter |
| 5 | MtEph | union | lg n | Θ((n+m) log(n+m)) | ❌ | Sequential sort+dedup |
| 6 | MtPer | filter | lg n | Θ(log n) | ✅ | ParaPair! |
| 7 | MtPer | intersection | lg n | Θ(log(m+n)) | ✅ | ParaPair! |
| 8 | MtPer | difference | lg n | Θ(log n) | ✅ | Via parallel filter |
| 9 | MtPer | union | lg n | Θ(log(m+n)) | ⚠️ | ParaPair! but **thread explosion** in tests |
| 10 | MtPer | from_seq | — | Θ(log² n) | ✅ | Parallel merge sort |

### 4d. Thread Explosion Issue (AVLTreeSetMtPer)

The `TestAVLTreeSetMtPer.rs` test file has **10 tests disabled** due to thread explosion from recursive ParaPair! calls. The following tests are commented out:

| # | Disabled Test | Operation | Reason |
|---|--------------|-----------|--------|
| 1 | test_union | union | Recursive ParaPair! |
| 2 | test_intersection | intersection | Recursive ParaPair! |
| 3 | test_difference | difference | Via recursive filter |
| 4 | test_filter | filter | Recursive ParaPair! |
| 5 | test_union_extended | union | Recursive ParaPair! |
| 6 | test_intersection_extended | intersection | Recursive ParaPair! |
| 7 | test_difference_extended | difference | Via recursive filter |
| 8 | test_intersection_disjoint | intersection | Recursive ParaPair! |
| 9 | test_union_empty | union | Recursive ParaPair! |
| 10 | test_union_parallel_path | union | Recursive ParaPair! |

The newer replacement tests (e.g., `test_filter_operation`, `test_intersection_operation`, `test_union_operation`) work because they use the `AVLTreeSetMtPerLit!` macro which builds sets via sequential `insert` calls, avoiding the recursive parallel code paths for small sizes. The `test_filter_parallel_path` and `test_intersection_parallel_path` tests pass with 10-12 elements (shallow recursion).

This indicates the recursive divide-and-conquer parallelism needs a sequential threshold (base case cutoff) to prevent unbounded thread spawning.

## Phase 5: Runtime Test Review

### 5a. Coverage Check

| # | Source Module | Test File | Active Tests | Disabled Tests | Status |
|---|-------------|-----------|:------------:|:--------------:|--------|
| 1 | ArraySetStEph | `TestArraySetStEph.rs` | 10 | 0 | ✅ Good |
| 2 | ArraySetEnumMtEph | `TestArraySetEnumMtEph.rs` | 9 | 0 | ✅ Good |
| 3 | AVLTreeSetStEph | `TestAVLTreeSetStEph.rs` | 39 | 0 | ✅ Comprehensive |
| 4 | AVLTreeSetStPer | `TestAVLTreeSetStPer.rs` | 11 | 0 | ✅ Good |
| 5 | AVLTreeSetMtEph | `TestAVLTreeSetMtEph.rs` | 40 | 0 | ✅ Comprehensive |
| 6 | AVLTreeSetMtPer | `TestAVLTreeSetMtPer.rs` | 33 | 10 | ⚠️ Feature-gated; 10 disabled |
| 7 | Example41_3 | `TestExample41_3.rs` | 4 | 0 | ✅ Good |

**Total**: 146 active tests, 10 disabled tests across 7 test files.

### 5b. Test Quality

#### Example 41.1 Coverage

All 6 test cases from the prose are covered:

| # | Prose Case | ArraySet | AVLTreeSet |
|---|-----------|:--------:|:----------:|
| 1 | \|{a, b, c}\| = 3 | ✅ | ✅ |
| 2 | {x ∈ {4, 11, 2, 6} \| x < 7} = {4, 2, 6} | ✅ | ✅ |
| 3 | find {6, 2, 9, 11, 8} 4 = false | ✅ | ✅ |
| 4 | {2, 7, 8, 11} ∪ {7, 9, 11, 14, 17} = {2, 7, 8, 9, 11, 14, 17} | ✅ | ✅ |
| 5 | toSeq {2, 7, 8, 11} (4 elements) | ✅ | ✅ |
| 6 | fromSeq ⟨2, 7, 2, 8, 11, 2⟩ = {8, 2, 11, 7} | ✅ | ✅ |

#### Edge Case Coverage

| # | Edge Case | ArraySetStEph | EnumMtEph | StEph | StPer | MtEph | MtPer |
|---|-----------|:------------:|:---------:|:-----:|:-----:|:-----:|:-----:|
| 1 | Empty set operations | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 2 | Singleton set | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 3 | Duplicate insert | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 4 | Delete nonexistent | — | — | ✅ | — | ✅ | — |
| 5 | Delete all elements | — | — | ✅ | — | ✅ | — |
| 6 | Disjoint set operations | — | — | ✅ | — | ✅ | — |
| 7 | Identical set operations | — | — | ✅ | — | ✅ | — |
| 8 | Filter: all match | — | — | ✅ | — | ✅ | — |
| 9 | Filter: none match | — | — | ✅ | — | ✅ | — |
| 10 | Large set (100+ elements) | — | — | ✅ | — | ✅ | ✅ |
| 11 | String element type | — | — | ✅ | — | ✅ | — |
| 12 | Persistence guarantee | — | — | — | ✅ | — | ✅ |
| 13 | Out-of-bounds (universe) | — | ✅ | — | — | — | — |
| 14 | Thread safety (concurrent reads) | — | ✅ | — | — | — | — |
| 15 | Negative numbers | — | — | — | — | — | ✅ |
| 16 | Parallel path (large input) | — | — | — | — | ✅ | ✅ |

### 5c. Test Issues

| # | Priority | Issue | Details |
|---|----------|-------|---------|
| 1 | High | MtPer thread explosion | 10 tests disabled; union/intersection/difference/filter with small inputs cause unbounded thread spawning |
| 2 | Medium | MtPer feature-gated | `TestAVLTreeSetMtPer.rs` requires `#![cfg(feature = "all_chapters")]` — may not run in default CI |
| 3 | Low | No benchmark tests | Prose requests benchmarks; none exist |

## Phase 6: Proof-Time Test (PTT) Review

No PTTs exist for Chap41. All code is unverified exec code outside `verus!` blocks. No PTTs are needed until the code is verusified.

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | `iterate f x a = Sequence.iterate f (toSeq a)` | Not implemented | Prose defines set iteration via toSeq |
| 2 | `reduce f x a = Sequence.reduce f (toSeq a)` | Not implemented | Prose defines set reduction via toSeq |
| 3 | Hash-table based set implementation | Not implemented | Prose mentions hash tables as a common approach |
| 4 | Complement excluded by design | ✅ Correct | Prose explicitly notes no complement (infinite result) |

### Implementation Deficiencies vs Prose

| # | Deficiency | Severity | Description |
|---|-----------|----------|-------------|
| 1 | AVL tree ops not used | **High** | StEph/StPer use AVL tree as flat sequence container; all O(log n) ops are O(n) |
| 2 | ArraySetEnumMtEph.filter pseudo-parallel | **High** | Sequential spawn-join loop instead of fork-join; APAS span not achieved |
| 3 | AVLTreeSetMtEph.union sequential | **Medium** | Comment says "avoid thread explosion"; APAS says Span lg(n) |
| 4 | AVLTreeSetMtPer thread explosion | **Medium** | 10 tests disabled; recursive ParaPair! needs base-case threshold |
| 5 | from_seq comment misleading | **Low** | ArraySetStEph and StEph both comment "parallel" but implement sequentially |
| 6 | No formal specs | **High** | 0/88 functions have requires/ensures |

### Code With No Prose Counterpart

| # | Item | Purpose |
|---|------|---------|
| 1 | `ArraySetStEph` | Sorted-array set variant; APAS only discusses enumerable boolean arrays and balanced trees |
| 2 | `AVLTreeSetMtEph` Arc\<Mutex\> wrapper | Thread-safety infrastructure |
| 3 | `AVLTreeSetMtPer` Ord/PartialOrd impls | Required by `OrderedTableMtPer` caller |
| 4 | All `*Lit!` macros | Convenience constructors |
| 5 | `View` impls in `verus!` blocks | Verus scaffolding (trivial identity views) |
| 6 | `parallel_filter`, `parallel_intersect`, `parallel_sort` | Internal helper functions for Mt parallelism |

## Phase 8: Table of Contents and Style Review

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
| 1 | ArraySetStEph | ❌ out (derive) | ❌ out (derive) | ❌ out | - | - | ❌ out (derive) | - | ✅ out | - |
| 2 | ArraySetEnumMtEph | ❌ out (derive) | ❌ out (derive) | - | - | - | - | - | ✅ out | - |
| 3 | AVLTreeSetStEph | ❌ out | ❌ out (derive) | ❌ out | - | - | ✅ out | ✅ out | ✅ out | View ✅ in |
| 4 | AVLTreeSetStPer | ❌ out | ❌ out (derive) | ❌ out | - | - | ❌ out (derive) | ✅ out | ✅ out | View ✅ in |
| 5 | AVLTreeSetMtEph | ❌ out | - | ❌ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 6 | AVLTreeSetMtPer | ❌ out | ❌ out (derive) | ❌ out | - | - | ✅ out | ✅ out | ✅ out | View ✅ in, Ord ❌ out |
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

## Review TODOs

| # | Priority | Category | TODO |
|---|----------|----------|------|
| 1 | **High** | Algorithmic | Fix AVLTreeSetStEph/StPer to use actual AVL tree balanced operations (find → tree search O(log n), insert → tree insert O(log n), delete → tree delete O(log n)) instead of linear scans and full rebuilds |
| 2 | **High** | Parallelism | Fix ArraySetEnumMtEph.filter to use proper parallel fork-join: collect all handles first, then join all (or use ParaPair! divide-and-conquer) instead of sequential spawn-join loop |
| 3 | **High** | Parallelism | Add sequential threshold (base case cutoff, e.g., n ≤ 128) to AVLTreeSetMtPer recursive ParaPair! calls to prevent thread explosion; re-enable 10 disabled tests |
| 4 | **High** | Verification | Verusify core set types with proper `requires`/`ensures` (at minimum: find, insert, delete, size, empty, singleton) |
| 5 | **Medium** | Parallelism | Parallelize AVLTreeSetMtEph.union with ParaPair! divide-and-conquer (like MtPer variant) instead of sequential sort+dedup |
| 6 | **Medium** | Performance | Use bitvec's built-in bitwise operations (`&`, `|`, `!` + `&`) for ArraySetEnumMtEph intersection/difference/union instead of element-wise loops |
| 7 | **Medium** | Performance | Fix AVLTreeSetMtPer.filter merge step to use linear merge (both halves are sorted and disjoint) instead of sort+dedup |
| 8 | **Medium** | Cost Annotations | Fix APAS cost annotations on ArraySetEnumMtEph insert/delete: annotated as "Work u" but ephemeral impl is O(1); clarify persistent vs ephemeral cost distinction |
| 9 | **Medium** | Comment | Fix misleading "parallel" comments in ArraySetStEph.from_seq and AVLTreeSetStEph.from_seq — both are sequential |
| 10 | **Low** | Missing Feature | Implement `iterate` and `reduce` on sets as defined in prose (`iterate f x a = Sequence.iterate f (toSeq a)`) |
| 11 | **Low** | Style | Add TOC headers to all 7 source files per table-of-contents-standard |
| 12 | **Low** | Style | Move Clone, PartialEq/Eq, Default impls inside `verus!` with specs when verusifying |
| 13 | **Low** | Testing | Add benchmark tests as requested in prose |
| 14 | **Low** | Performance | ArraySetStEph.find should use binary search (elements are sorted) |
| 15 | **Low** | Naming | Fix swapped variable names in AVLTreeSetMtEph.parallel_filter (`left_vals` holds right half, `right_vals_final` holds left half via `split_off`) |
