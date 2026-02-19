<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 41 — Sets: Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap41.txt`
**Source files:** `src/Chap41/ArraySetStEph.rs`, `src/Chap41/ArraySetEnumMtEph.rs`, `src/Chap41/AVLTreeSetStEph.rs`, `src/Chap41/AVLTreeSetStPer.rs`, `src/Chap41/AVLTreeSetMtEph.rs`, `src/Chap41/AVLTreeSetMtPer.rs`, `src/Chap41/Example41_3.rs`
**Test files:** 7 test files, 156 tests total
**PTT files:** None
**Verification status:** All 6 ADT modules verusified with `#[verifier::external_body]` specs; View type is `Set<T>`. Verus: 1863 verified, 0 errors. RTT: 2698 passed.

## Phase 1: Inventory

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap41 | AVLTreeSetMtEph | 12 | 14 | 0 | 0 | 14 | 0 | 0 | 14 | 0 |
| 2 | Chap41 | AVLTreeSetMtPer | 12 | 14 | 0 | 0 | 14 | 0 | 0 | 14 | 0 |
| 3 | Chap41 | AVLTreeSetStEph | 12 | 14 | 0 | 0 | 14 | 0 | 0 | 14 | 0 |
| 4 | Chap41 | AVLTreeSetStPer | 12 | 14 | 0 | 0 | 14 | 0 | 0 | 14 | 0 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 15 | 0 | 0 | 15 | 0 | 0 | 15 | 0 |
| 6 | Chap41 | ArraySetStEph | 12 | 14 | 0 | 0 | 14 | 0 | 0 | 14 | 0 |
| 7 | Chap41 | Example41_3 | 3 | 0 | 0 | 4 | 0 | 5 | 0 | 0 | 5 |

**Total:** 90 functions, 85 inside `verus!`, 5 outside, 85 with specs, 85 proof holes (all `external_body`).

## Phase 2: Prose Inventory

### Data Types

| # | Prose Item | Reference | Description |
|---|-----------|-----------|-------------|
| 1 | Data Type 41.1 — Sets ADT | §2 | Universe U with equality; type S representing power set |
| 2 | `size : S → N` | §2 | Number of elements |
| 3 | `toSeq : S → Seq` | §2 | Convert set to sequence |
| 4 | `empty : S` | §2 | Empty set |
| 5 | `singleton : U → S` | §2 | Single-element set |
| 6 | `fromSeq : Seq → S` | §2 | Convert sequence to set |
| 7 | `filter : (U → B) → S → S` | §2 | Elements satisfying predicate |
| 8 | `intersection : S × S → S` | §2 | Elements in both sets |
| 9 | `difference : S × S → S` | §2 | Elements in first but not second |
| 10 | `union : S × S → S` | §2 | Elements in either set |
| 11 | `find : S × U → B` | §2 | Membership test |
| 12 | `delete : S × U → S` | §2 | Remove element |
| 13 | `insert : S × U → S` | §2 | Add element |

### Cost Specifications

| # | Prose Item | Reference | Description |
|---|-----------|-----------|-------------|
| 14 | Cost Specification 41.3 (Arrays for Enumerable Sets) | §3 | Bit-array set costs; universe U = {0..u-1}; W,S for each op |
| 15 | Cost Specification 41.4 (Tree Sets) | §4 | BST-based set costs; m·lg(n/m) for set-set ops |

### Examples

| # | Prose Item | Reference | Description |
|---|-----------|-----------|-------------|
| 16 | Example 41.1 | §2 | Basic set operations demonstration |
| 17 | Example 41.3 | §3 | `fromSeq` via iterate insert |

### Implementation Mapping

The chapter provides 7 implementations of the Sets ADT across two representation strategies:

| # | Representation | Modules | Prose Section |
|---|---------------|---------|---------------|
| 1 | Sorted array (ArraySeq-backed) | ArraySetStEph | §3 |
| 2 | Bit array (enumerable sets) | ArraySetEnumMtEph | §3 |
| 3 | AVL tree (sequence-backed) | AVLTreeSetStEph, AVLTreeSetStPer, AVLTreeSetMtEph, AVLTreeSetMtPer | §4 |
| 4 | Example demonstrations | Example41_3 | §2, §3 |

### Implementation Status per ADT Operation

| # | ADT Operation | ArrayStEph | ArrayEnumMt | AVLStEph | AVLStPer | AVLMtEph | AVLMtPer | Ex41_3 |
|---|--------------|:----------:|:-----------:|:--------:|:--------:|:--------:|:--------:|:------:|
| 1 | size | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 2 | toSeq | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 3 | empty | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 4 | singleton | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 5 | fromSeq | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| 6 | filter | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 7 | intersection | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 8 | difference | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 9 | union | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 10 | find | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 11 | delete | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 12 | insert | Yes | Yes | Yes | Yes | Yes | Yes | - |
| 13 | Example 41.1 | - | - | - | - | - | - | Yes |
| 14 | Example 41.3 | - | - | - | - | - | - | Yes |

All 12 ADT operations implemented in all 6 set modules. Full coverage.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations — ArraySetStEph (sorted array, matching Cost Spec 41.3 loosely)

Note: Cost Spec 41.3 is for enumerable (bit-array) sets. ArraySetStEph uses sorted arrays instead, giving different costs.

| # | Operation | APAS (Bit Array) W | APAS (Bit Array) S | Code (Sorted Array) W | Code (Sorted Array) S | Match? | Notes |
|---|----------|-------------------|--------------------|-----------------------|-----------------------|:------:|-------|
| 1 | size | 1 | 1 | Θ(1) | Θ(1) | Yes | Delegates to ArraySeq.length() |
| 2 | empty | 1 | 1 | Θ(1) | Θ(1) | Yes | |
| 3 | singleton | u | 1 | Θ(1) | Θ(1) | N/A | Different representation |
| 4 | fromSeq | u + \|a\| | 1 | Θ(n lg n) | Θ(n lg n) | N/A | Sort + dedup |
| 5 | filter | u + Σ W(f) | 1 + max S(f) | Θ(n · W(f)) | Θ(n · W(f)) | N/A | Linear scan |
| 6 | intersection | u | 1 | Θ(m + n) | Θ(m + n) | N/A | Merge-style scan |
| 7 | difference | u | 1 | Θ(m + n) | Θ(m + n) | N/A | Merge-style scan |
| 8 | union | u | 1 | Θ(m + n) | Θ(m + n) | N/A | Merge-style scan |
| 9 | find | lg \|a\| | lg \|a\| | Θ(n) | Θ(n) | **No** | Linear scan instead of binary search |
| 10 | delete | lg \|a\| | lg \|a\| | Θ(n) | Θ(n) | **No** | Linear scan + rebuild |
| 11 | insert | lg \|a\| | lg \|a\| | Θ(n) | Θ(n) | **No** | Linear scan + rebuild |
| 12 | toSeq | \|a\| | 1 | Θ(n) | Θ(n) | Yes | |

### 3a. Cost Annotations — ArraySetEnumMtEph (bit array, matching Cost Spec 41.3)

| # | Operation | APAS W | APAS S | Code W | Code S | Match? | Notes |
|---|----------|--------|--------|--------|--------|:------:|-------|
| 1 | size | 1 | 1 | Θ(u) | Θ(u) | **No** | Counts bits by iterating; should be O(u/w) with popcount |
| 2 | empty | u | 1 | Θ(u) | Θ(u) | Partial | Sequential bit clear |
| 3 | singleton | u | 1 | Θ(u) | Θ(u) | Yes | Allocate + set one bit |
| 4 | fromSeq | u + \|a\| | 1 | Θ(u + \|a\|) | Θ(u + \|a\|) | Yes | |
| 5 | filter | u + Σ W(f) | 1 + max S(f) | Θ(u · W(f)) | Θ(u · W(f)) | Yes | Sequential scan |
| 6 | intersection | u | 1 | Θ(u/w) | Θ(u/w) | Yes | BitBox `&` (word-parallel) |
| 7 | difference | u | 1 | Θ(u/w) | Θ(u/w) | Yes | BitBox `& !` (word-parallel) |
| 8 | union | u | 1 | Θ(u/w) | Θ(u/w) | Yes | BitBox `\|` (word-parallel) |
| 9 | find | 1 | 1 | Θ(1) | Θ(1) | Yes | Direct bit index |
| 10 | delete | 1 | 1 | Θ(1) | Θ(1) | Yes | Set bit to 0 |
| 11 | insert | 1 | 1 | Θ(1) | Θ(1) | Yes | Set bit to 1 |
| 12 | toSeq | \|a\| | 1 | Θ(u) | Θ(u) | Partial | Scans all bits, not just set ones |

### 3a. Cost Annotations — AVL Tree Sets (matching Cost Spec 41.4)

Note: All four AVL tree set implementations use the backing AVLTreeSeq as a sorted sequence, performing linear scans and full rebuilds rather than using tree-native set operations (split/join). This fundamentally changes their cost profile.

| # | Operation | APAS W | APAS S | Code W | Code S | Match? | Notes |
|---|----------|--------|--------|--------|--------|:------:|-------|
| 1 | size | 1 | 1 | Θ(1) | Θ(1) | Yes | Delegates to AVLTreeSeq.length() |
| 2 | empty | 1 | 1 | Θ(1) | Θ(1) | Yes | |
| 3 | singleton | lg 1 = O(1) | lg 1 = O(1) | Θ(1) | Θ(1) | Yes | |
| 4 | fromSeq | n·lg n | lg²n | Θ(n lg n) | Θ(n lg n) [St] / Θ(lg²n) [MtPer] | Partial | MtPer parallel sort matches span; StEph is sequential |
| 5 | filter | Σ W(f) + n | lg n + max S(f) | Θ(n · W(f)) | Θ(n · W(f)) [St] / Θ(lg n) [MtPer] | Partial | MtPer parallel; others sequential |
| 6 | intersection | m·lg(n/m+1) | lg n | Θ(m + n) | Θ(m + n) [St] / Θ(lg(m+n)) [MtPer] | **No** | Linear merge instead of split-based O(m·lg(n/m)) |
| 7 | difference | m·lg(n/m+1) | lg n | Θ(m + n) | Θ(m + n) [St] / Θ(lg(m+n)) [MtPer] | **No** | Same issue |
| 8 | union | m·lg(n/m+1) | lg n | Θ(m + n) | Θ(m + n) [St] / Θ(lg(m+n)) [MtPer] | **No** | Same issue |
| 9 | find | lg \|a\| | lg \|a\| | Θ(n) | Θ(n) | **No** | Linear scan instead of BST search |
| 10 | delete | lg \|a\| | lg \|a\| | Θ(n) | Θ(n) | **No** | Linear scan + rebuild |
| 11 | insert | lg \|a\| | lg \|a\| | Θ(n) | Θ(n) | **No** | Linear scan + rebuild |
| 12 | toSeq | \|a\| | 1 | Θ(n) | Θ(n) | Partial | In-order traversal |

### 3b. Implementation Fidelity

| # | Algorithm | Module(s) | Fidelity | Detail |
|---|----------|-----------|:--------:|--------|
| 1 | Bit-array set ops | ArraySetEnumMtEph | Faithful | intersection/union/difference use BitBox bitwise ops, matching prose word-parallel model |
| 2 | Bit-array find/insert/delete | ArraySetEnumMtEph | Faithful | Direct bit indexing, O(1) |
| 3 | Sorted array set ops | ArraySetStEph | Correct but suboptimal | Uses linear scans where binary search possible (find, insert, delete) |
| 4 | AVL tree-based sets | All AVL modules | **Structural mismatch** | Uses AVLTreeSeq as flat sorted sequence, not as a BST with split/join. All set-set operations (intersection, union, difference) are O(m+n) merge scans instead of O(m·lg(n/m)) split-based algorithms. find/insert/delete are O(n) linear scans instead of O(lg n) tree operations. |
| 5 | fromSeq (parallel) | AVLTreeSetMtPer | Faithful | Parallel merge sort via `ParaPair!` with sequential cutoff |
| 6 | filter (parallel) | AVLTreeSetMtEph, AVLTreeSetMtPer | Faithful | Divide-and-conquer via `ParaPair!` |
| 7 | intersection (parallel) | AVLTreeSetMtEph, AVLTreeSetMtPer | Faithful structure | Uses `ParaPair!` for divide-and-conquer, but on the wrong algorithm (linear merge instead of split-based) |
| 8 | union (MtEph) | AVLTreeSetMtEph | **Sequential** | Explicit comment: "Union uses a simple merge strategy to avoid thread explosion" — no parallelism |
| 9 | union (MtPer) | AVLTreeSetMtPer | Parallel | Uses `ParaPair!` for divide-and-conquer |
| 10 | Example 41.1 | Example41_3 | Faithful | Both ArraySet and AVLTreeSet versions match prose |
| 11 | Example 41.3 (fromSeq) | Example41_3 | Faithful | Demonstrates iterate-insert pattern |

### 3c. Spec Fidelity

All 6 ADT modules now have:
- Code inside `verus! {}` blocks
- View impls with `type V = Set<T>` and `#[verifier::external_body]`
- All exec methods have `#[verifier::external_body]` with `ensures` clauses
- Set operation specs: empty, singleton, size, find, insert, delete, union, intersection, difference, filter, to_seq, from_seq, clone

Specs are complete (ensures clauses on every trait method). Implementation bodies are trusted via `external_body`; no verified proofs of correctness yet.

## Phase 4: Parallelism Review

### ArraySetEnumMtEph

| # | Function | Parallel? | Mechanism | Prose Match? | Notes |
|---|---------|:---------:|-----------|:------------:|-------|
| 1 | intersection | Sequential | BitBox `&` op | Partial | Word-level parallelism from hardware, not thread parallelism |
| 2 | difference | Sequential | BitBox `& !` op | Partial | Same |
| 3 | union | Sequential | BitBox `\|` op | Partial | Same |
| 4 | filter | Sequential | Linear scan | No | Prose span is O(1 + max S(f)); code is O(u·W(f)) |
| 5 | All other ops | Sequential | Direct bit ops | Yes | find/insert/delete are O(1) |

**Classification: Pseudo-parallel.** Named "Mt" (multi-threaded) but no actual thread spawning. BitBox operations provide implicit word-level parallelism through hardware SIMD/bitwise operations, which partially matches the prose's O(u/w) cost model but not the O(1) span.

### AVLTreeSetMtEph

| # | Function | Parallel? | Mechanism | Prose Match? | Notes |
|---|---------|:---------:|-----------|:------------:|-------|
| 1 | filter | Yes | `parallel_filter` via `ParaPair!` | Yes | Extracts Vec, divides, recurses |
| 2 | intersection | Yes | `parallel_intersect` via `ParaPair!` | Partial | Parallel but on linear algorithm |
| 3 | difference | Delegating | Calls `filter` (which is parallel) | Partial | `other.find(x)` is O(n), making it O(m·n) |
| 4 | union | **Sequential** | Merge + sort + dedup | **No** | Comment: "avoid thread explosion" |
| 5 | insert/delete | Delegating | Lock + delegate to StEph | N/A | O(1) lock overhead + StEph's O(n) |
| 6 | from_seq | Delegating | Wraps StEph::from_seq | N/A | |

**Classification: Partially parallel.** filter and intersection genuinely parallel; union sequential; difference delegates to parallel filter but with quadratic find. Thread safety via `Arc<Mutex>`.

### AVLTreeSetMtPer

| # | Function | Parallel? | Mechanism | Prose Match? | Notes |
|---|---------|:---------:|-----------|:------------:|-------|
| 1 | from_seq | Yes | `parallel_sort` via `ParaPair!` merge sort | Yes | Sequential cutoff at 128 |
| 2 | filter | Yes | Divide-and-conquer via `ParaPair!` | Yes | Sequential cutoff at SEQUENTIAL_CUTOFF |
| 3 | intersection | Yes | Divide-and-conquer via `ParaPair!` | Partial | Parallel but on linear merge algorithm |
| 4 | difference | Yes | Divide-and-conquer via `ParaPair!` | Partial | Same issue |
| 5 | union | Yes | Divide-and-conquer via `ParaPair!` | Partial | Parallel but O(m+n) merge not O(m·lg(n/m)) split-based |
| 6 | delete | Yes | Parallel sort + dedup after removal | Overkill | O(n lg n) instead of O(lg n) |
| 7 | insert | Yes | Parallel sort + dedup after insertion | Overkill | O(n lg n) instead of O(lg n) |

**Classification: Genuinely parallel.** All major operations use `ParaPair!` with sequential cutoffs. However, the underlying algorithms are linear merge scans rather than split-based tree operations, so parallelism reduces span but doesn't achieve the prose's O(m·lg(n/m)) work bound.

## Phase 5: Runtime Test Review

### Test Count Summary

| # | File | Tests | Coverage |
|---|------|------:|----------|
| 1 | TestArraySetStEph.rs | 10 | All 12 ADT ops + examples |
| 2 | TestArraySetEnumMtEph.rs | 9 | All 13 trait ops + thread safety + bounds |
| 3 | TestAVLTreeSetStEph.rs | 39 | All ops + edge cases + trait impls |
| 4 | TestAVLTreeSetStPer.rs | 11 | All ops + persistence + trait impls |
| 5 | TestAVLTreeSetMtEph.rs | 40 | All ops + parallel paths + edge cases + large sets + strings |
| 6 | TestAVLTreeSetMtPer.rs | 43 | All ops + parallel paths + persistence + edge cases + trait impls |
| 7 | TestExample41_3.rs | 4 | Both examples + additional operations |

**Total: 156 tests.**

### Coverage Assessment

- **Excellent breadth:** Every ADT operation tested for every implementation.
- **Edge cases well-covered:** Empty sets, singletons, duplicates, disjoint sets, identical sets, non-existent elements, all-match/none-match filters.
- **Parallel paths tested:** MtEph and MtPer have explicit tests for parallel filter, intersection, union, and from_seq paths (`test_filter_parallel_path`, `test_intersection_parallel_path`, etc.).
- **Persistence tested:** MtPer and StPer have tests verifying original set unchanged after mutation operations.
- **Thread safety:** ArraySetEnumMtEph has an explicit `test_array_set_enum_mt_thread_safety` test.
- **Large-scale tests:** Both MtEph and MtPer have `test_large_set_operations` tests.
- **Trait impl tests:** Clone, Debug, Display, Default, PartialOrd, Ord all tested where implemented.

### Missing Tests

| # | Gap | Recommendation |
|---|-----|----------------|
| 1 | No concurrent mutation test for MtEph | Test insert/delete from multiple threads simultaneously |
| 2 | No performance/timing test | Verify parallel speedup exists for Mt modules |
| 3 | No test for ArraySetEnumMtEph with universe_size=0 | Boundary condition |

## Phase 6: PTT Review

No PTTs exist. ADT modules are verusified with `external_body`; no proof-time verification of implementations to test.

## Phase 7: Gap Analysis

### Prose Items Not Implemented

| # | Priority | Gap | Detail | Recommendation |
|---|----------|-----|--------|----------------|
| 1 | **Critical** | AVL tree sets use wrong algorithm | All 4 AVLTreeSet modules use sorted-sequence linear scans instead of split/join-based tree operations. find/insert/delete are O(n) instead of O(lg n). intersection/union/difference are O(m+n) instead of O(m·lg(n/m)). | Rewrite using BST split/join from Chapter 38/39 |
| 2 | High | ArraySetStEph find is O(n) | Uses linear scan on sorted array. Should use binary search for O(lg n). | Replace with binary_search |
| 3 | Medium | ArraySetEnumMtEph is not truly Mt | Named "Mt" but has no thread spawning. Only word-level hardware parallelism from bitwise ops. | Either add true parallelism or rename to clarify |
| 4 | Medium | AVLTreeSetMtEph union is sequential | Explicit comment about avoiding thread explosion. Other set-set ops are parallel. | Implement parallel union or document trade-off |
| 5 | Medium | MtEph difference is O(m·n) | Delegates to filter with `other.find(x)` which is O(n) per element, giving O(m·n) total. | Use merge-based approach or BST-based approach |
| 6 | Low | All ADT impls use external_body | 85 proof holes; specs exist but bodies are trusted. | Future: replace external_body with verified proofs |
| 7 | Low | ArraySetEnumMtEph.size() is O(u) | Counts bits by iterating. Could maintain a count field for O(1). | Add size field or use popcount |

### Code With No Prose Counterpart

| # | Item | Module | Purpose |
|---|------|--------|---------|
| 1 | `new(u)` (universe constructor) | ArraySetEnumMtEph | Creates empty set with universe size |
| 2 | `default` | Most modules | Rust Default trait |
| 3 | `PartialOrd`/`Ord` | AVLTreeSetMtPer | Ordering on sets (lexicographic) |
| 4 | `parallel_sort` | AVLTreeSetMtPer | Internal merge sort for from_seq |
| 5 | `parallel_filter` / `parallel_intersect` | AVLTreeSetMtEph | Internal parallel helpers |
| 6 | Various macros | ArraySetStEph, ArraySetEnumMtEph, all AVL | Literal constructors |
| 7 | `View` impl | All 6 ADT modules | Verus view trait (ghost spec) |

## Phase 8: TOC Review

All 6 ADT files follow the 13-section TOC standard. Example41_3 has no `verus!` blocks.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:---:|:------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | ArraySetStEph.rs | - | - | ✅ out | - | - | - | - | ✅ out | View in V! |
| 2 | ArraySetEnumMtEph.rs | ✅ out (derive) | ✅ out (derive) | - | - | - | ✅ out (derive) | - | ✅ out | View in V! |
| 3 | AVLTreeSetStEph.rs | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | View in V! |
| 4 | AVLTreeSetStPer.rs | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | View in V! |
| 5 | AVLTreeSetMtEph.rs | ✅ out | ✅ out | ✅ out | - | - | ✅ out | ✅ out | ✅ out | View in V! |
| 6 | AVLTreeSetMtPer.rs | ✅ out | ✅ out (derive) | ✅ out | - | - | ✅ out | ✅ out | ✅ out | View in V!, Ord out |
| 7 | Example41_3.rs | - | - | - | - | - | - | - | - | |

## Proof Holes Summary

```
❌ AVLTreeSetMtEph.rs      14 × external_body (View + 12 ADT ops + clone)
❌ AVLTreeSetMtPer.rs      14 × external_body (View + 12 ADT ops + clone)
❌ AVLTreeSetStEph.rs      14 × external_body (View + 12 ADT ops + clone)
❌ AVLTreeSetStPer.rs      14 × external_body (View + 12 ADT ops + clone)
❌ ArraySetEnumMtEph.rs    15 × external_body (View + new + 12 ADT ops + clone)
❌ ArraySetStEph.rs        14 × external_body (View + 12 ADT ops + clone)
✓ Example41_3.rs           clean (demo code, no verus!)

═══════════════════════════════════════════════════════════════
SUMMARY
═══════════════════════════════════════════════════════════════

Modules:     1 clean, 6 holed, 7 total
Holes Found: 85 total (all external_body)
```

All holes are `#[verifier::external_body]` on View::view and trait impl methods. Each method has an `ensures` clause; the implementation body is trusted rather than verified.

## Spec Strength Summary

| Classification | Count |
|---------------|------:|
| strong | 0 |
| partial | 83 |
| weak | 0 |
| none | 5 |

**Partial:** 83 functions in ADT modules have `ensures` clauses but use `external_body` (trusted, not verified). **None:** 5 functions in Example41_3 (plain Rust, no specs).

## Action Items

| # | Priority | Action |
|---|----------|--------|
| 1 | Critical | Rewrite AVL tree sets using BST split/join for O(lg n) find/insert/delete and O(m·lg(n/m)) set-set ops |
| 2 | High | Add binary search to ArraySetStEph.find |
| 3 | Medium | Add true parallelism to ArraySetEnumMtEph or rename to clarify word-parallel only |
| 4 | Medium | Implement parallel union in AVLTreeSetMtEph |
| 5 | Medium | Fix AVLTreeSetMtEph.difference O(m·n) by using merge-based approach |
| 6 | Low | Replace external_body with verified proofs where feasible |
| 7 | Low | Optimize ArraySetEnumMtEph.size() to O(1) via maintained count or popcount |

## Overall Assessment

Chapter 41 provides a complete implementation of the Sets ADT (Data Type 41.1) across 6 modules covering sorted-array, bit-array, and tree-based representations in St/Mt and Eph/Per variants. All 12 ADT operations are implemented in every module, and 156 runtime tests provide excellent coverage.

**Verification status (2026-02-18):** All 6 ADT modules are verusified. Code lives inside `verus!` blocks with View type `Set<T>`. All trait methods have `ensures` clauses. All 85 impl methods use `#[verifier::external_body]` — specs are complete, bodies are trusted. Example41_3 remains plain Rust demo code. Verus: 1863 verified, 0 errors. RTT: 2698 passed.

**Strengths:**
- Complete ADT coverage: all 12 operations in all 6 modules
- Full verusification: all ADT code in `verus!` with View and ensures
- ArraySetEnumMtEph faithfully implements bit-array set operations with O(1) find/insert/delete
- AVLTreeSetMtPer provides genuine parallelism with `ParaPair!` and sequential cutoffs for from_seq, filter, intersection, difference, and union
- Excellent test suite: 156 tests covering all operations, edge cases, parallel paths, persistence, and trait implementations
- Examples (41.1, 41.3) implemented and tested

**Weaknesses:**
- **Fundamental algorithmic mismatch in AVL tree sets:** All 4 AVLTreeSet modules treat the backing AVL tree as a flat sorted sequence, giving O(n) for find/insert/delete and O(m+n) for set-set operations, versus the prose's O(lg n) and O(m·lg(n/m)) respectively. This is the most significant gap — the tree structure exists but is used only for sequential access, not for logarithmic operations.
- **85 proof holes:** All impl methods use `external_body`; no verified proofs of correctness yet
- ArraySetStEph.find uses linear scan instead of binary search on sorted array
- ArraySetEnumMtEph is named "Mt" but has no thread spawning
- AVLTreeSetMtEph.union is explicitly sequential
- AVLTreeSetMtEph.difference is O(m·n) due to linear find in filter predicate
