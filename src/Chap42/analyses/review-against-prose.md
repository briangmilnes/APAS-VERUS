<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 42: Tables — Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap42.txt` (Data Type 42.1, Algorithm 42.3, Cost Specification 42.5)

## Phase 1: ADT Coverage

Data Type 42.1 specifies 15 operations for the TABLE abstract data type:

| # | APAS Operation | TableStEph | TableStPer | TableMtEph | Notes |
|---|---|:---:|:---:|:---:|---|
| 1 | `size` | Yes | Yes | Yes | |
| 2 | `empty` | Yes | Yes | Yes | |
| 3 | `singleton` | Yes | Yes | Yes | |
| 4 | `domain` | Yes | Yes | Yes | |
| 5 | `tabulate` | Yes | Yes | Yes | |
| 6 | `map` | Yes | Yes | Yes | |
| 7 | `filter` | Yes | Yes | Yes | |
| 8 | `intersection` | Yes | Yes | Yes | |
| 9 | `union` | Yes | Yes | Yes | |
| 10 | `difference` | Yes | Yes | Yes | |
| 11 | `find` | Yes | Yes | Yes | |
| 12 | `delete` | Yes | Yes | Yes | |
| 13 | `insert` | Yes | Yes | Yes | |
| 14 | `restrict` | Yes | Yes | Yes | |
| 15 | `subtract` | Yes | Yes | Yes | |

**Extra operations (not in ADT 42.1):**

| # | Operation | File | Notes |
|---|---|---|---|
| 1 | `collect` | All 3 files | Returns entries as a sequence. Referenced in Algorithm 42.3 prose. |
| 2 | `from_sorted_entries` | All 3 files | Constructor helper for building from sorted Vec. |
| 3 | `TableStEphLit!` macro | TableStEph.rs | Literal syntax for creating tables. |
| 4 | `TableStPerLit!` macro | TableStPer.rs | Literal syntax for creating tables. |
| 5 | `TableMtEphLit!` macro | TableMtEph.rs | Literal syntax for creating tables. |

**Coverage: 15/15 ADT operations implemented across all three variants.**

## Phase 2: Algorithm Coverage

| # | APAS Algorithm | Implemented | File | Notes |
|---|---|:---:|---|---|
| 1 | Algorithm 42.3 (collect) | Partial | All 3 | The APAS specifies collect as `Sequence.reduce (Table.union Sequence.append) {} ⟨{k→⟨v⟩}:(k,v)∈a⟩`. The implementations simply return `self.entries.clone()`, which is a trivial O(n) collect, not the reduce-based formulation. The prose collect gathers all values with the same key into sequences; the implementations return key-value pairs directly. |

## Phase 3: Example Coverage

| # | APAS Example | Implemented | File | Notes |
|---|---|:---:|---|---|
| 1 | Example 42.1 | Partial | `Example42_1.rs` | Demonstrates find, filter, map, tabulate, union, subtract with all 3 implementations. Does not replicate the exact APAS example values (`'a'→4, 'b'→11, 'c'→2`, `'b'→3, 'd'→5`). Uses integer keys instead of character keys. The specific results from the prose (e.g., `a['b'] = 11`, `filter {k→x∈a | x<7}`) are not verified. |

## Phase 4: Cost Annotation Audit

### TableStEph.rs — Single-threaded ephemeral

| # | Operation | APAS Cost | Claude Cost | Match | Notes |
|---|---|---|---|:---:|---|
| 1 | `size` | W:1, S:1 | W:Θ(1), S:Θ(1) | Yes | |
| 2 | `empty` | W:1, S:1 | W:Θ(1), S:Θ(1) | Yes | |
| 3 | `singleton` | W:1, S:1 | W:Θ(1), S:Θ(1) | Yes | |
| 4 | `domain` | W:\|a\|, S:lg\|a\| | W:Θ(n log n), S:Θ(n log n) | No | Impl does sequential insertion into ArraySetStEph, costing O(n) per insert × n inserts = O(n²) in worst case. APAS assumes O(n) work via parallel extraction. |
| 5 | `tabulate` | W:\|s\|*W(f), S:lg\|s\|+S(f) | W:Θ(\|keys\|×W(f)), S:Θ(\|keys\|×S(f)) | Partial | Work matches. Span is sequential (n×S(f)) rather than APAS's logarithmic. Also does a sort at the end. |
| 6 | `map` | W:Σ W(f(v)), S:lg\|a\|+max S(f(v)) | W:Θ(n×W(f)), S:Θ(n×S(f)) | Partial | Work matches. Span is sequential rather than APAS's logarithmic. |
| 7 | `filter` | W:Σ W(p(k,v)), S:lg\|a\|+max S(p(k,v)) | W:Θ(n×W(f)), S:Θ(n×S(f)) | Partial | Same as map. |
| 8 | `intersection` | W:m*lg(1+n/m), S:lg(n+m) | W:Θ(m+n), S:Θ(m+n) | No | Merge-based impl is O(m+n). APAS assumes divide-and-conquer with O(m*lg(1+n/m)). |
| 9 | `union` | Same as intersection | W:Θ(m+n), S:Θ(m+n) | No | Same issue. |
| 10 | `difference` | Same as intersection | W:Θ(m+n), S:Θ(m+n) | No | Same issue. |
| 11 | `find` | W:lg\|a\|, S:lg\|a\| | W:Θ(log n), S:Θ(log n) | Yes | Binary search matches APAS. |
| 12 | `delete` | W:lg\|a\|, S:lg\|a\| | W:Θ(n), S:Θ(n) | No | Linear scan to rebuild array. APAS assumes O(lg n) via tree-based structure. |
| 13 | `insert` | W:lg\|a\|, S:lg\|a\| | W:Θ(n), S:Θ(n) | No | Linear rebuild + sort. APAS assumes O(lg n). |
| 14 | `restrict` | W:m*lg(1+n/m), S:lg(n+m) | W:Θ(m+n), S:Θ(m+n) | No | Linear scan with find per entry. |
| 15 | `subtract` | Same as restrict | W:Θ(m+n), S:Θ(m+n) | No | Same issue. |

### TableStPer.rs — Single-threaded persistent

| # | Operation | APAS Cost | Claude Cost | Match | Notes |
|---|---|---|---|:---:|---|
| 1 | `size` | W:1, S:1 | W:Θ(1), S:Θ(1) | Yes | |
| 2 | `empty` | W:1, S:1 | W:Θ(1), S:Θ(1) | Yes | |
| 3 | `singleton` | W:1, S:1 | W:Θ(1), S:Θ(1) | Yes | |
| 4 | `domain` | W:\|a\|, S:lg\|a\| | W:Θ(n log n), S:Θ(n log n) | No | Sequential. |
| 5 | `tabulate` | W:\|s\|*W(f), S:lg\|s\|+S(f) | W:Θ(\|keys\|×W(f)), S:Θ(log\|keys\|+S(f)) | Partial | Uses `tabulate` from ArraySeqStPer but no sort guarantee. |
| 6 | `map` | W:Σ W(f(v)), S:lg\|a\|+S(f) | W:Θ(n×W(f)), S:Θ(log n+S(f)) | Yes | Uses `tabulate` for parallel-ready construction. |
| 7 | `filter` | W:Σ W(p(k,v)), S:lg\|a\|+S(p(k,v)) | W:Θ(n×W(f)), S:Θ(log n+S(f)) | Partial | Sequential loop implementation. |
| 8 | `intersection` | W:m*lg(1+n/m), S:lg(n+m) | W:Θ(m log(1+n/m)), S:Θ(log(n+m)) | Yes | Matches APAS exactly. |
| 9 | `union` | Same as intersection | Same | Yes | Decomposed into intersection + 2 differences as in prose. |
| 10 | `difference` | Same as intersection | Same | Yes | Merge-based but annotated with APAS costs. |
| 11 | `find` | W:lg\|a\|, S:lg\|a\| | W:Θ(log n), S:Θ(log n) | Yes | Binary search. |
| 12 | `delete` | W:lg\|a\|, S:lg\|a\| | W:Θ(n), S:Θ(n) | No | Linear scan. |
| 13 | `insert` | W:lg\|a\|, S:lg\|a\| | W:Θ(n), S:Θ(n) | No | Linear scan for insertion point. |
| 14 | `restrict` | W:m*lg(1+n/m), S:lg(n+m) | W:Θ(m+n), S:Θ(m+n) | No | Sequential scan. |
| 15 | `subtract` | Same as restrict | Same | No | Sequential scan. |

### TableMtEph.rs — Multi-threaded ephemeral

| # | Operation | APAS Cost | Claude Cost | Match | Notes |
|---|---|---|---|:---:|---|
| 1 | `size` | W:1, S:1 | W:Θ(1), S:Θ(1) | Yes | |
| 2 | `empty` | W:1, S:1 | W:Θ(1), S:Θ(1) | Yes | |
| 3 | `singleton` | W:1, S:1 | W:Θ(1), S:Θ(1) | Yes | |
| 4 | `domain` | W:\|a\|, S:lg\|a\| | W:Θ(n log n), S:Θ(log n) | Partial | Parallel spawn/join for key extraction, but inserts back into sequential ArraySetStEph. |
| 5 | `tabulate` | W:\|s\|*W(f), S:lg\|s\|+S(f) | W:Θ(\|keys\|×W(f)), S:Θ(log\|keys\|+S(f)) | Yes | Parallel spawn/join. |
| 6 | `map` | W:Σ W(f(v)), S:lg\|a\|+S(f) | W:Θ(n×W(f)), S:Θ(log n+S(f)) | Yes | Parallel spawn/join. |
| 7 | `filter` | W:Σ W(p(k,v)), S:lg\|a\|+S(p(k,v)) | W:Θ(n×W(f)), S:Θ(log n+S(f)) | Yes | Parallel spawn/join. |
| 8 | `intersection` | W:m*lg(1+n/m), S:lg(n+m) | W:Θ(m+n), S:Θ(log(m+n)) | Partial | Sequential merge despite being in MtEph file. |
| 9 | `union` | Same | Same | Partial | Sequential merge. |
| 10 | `difference` | Same | Same | Partial | Sequential merge. |
| 11 | `find` | W:lg\|a\|, S:lg\|a\| | W:Θ(log n), S:Θ(log n) | Yes | Binary search. |
| 12 | `delete` | W:lg\|a\|, S:lg\|a\| | W:Θ(n), S:Θ(log n) | Partial | Parallel filter, but linear work vs APAS's O(lg n). |
| 13 | `insert` | W:lg\|a\|, S:lg\|a\| | W:Θ(n), S:Θ(log n) | Partial | Parallel but linear work. |
| 14 | `restrict` | W:m*lg(1+n/m), S:lg(n+m) | W:Θ(m+n), S:Θ(log(m+n)) | Partial | Parallel filter, good span, suboptimal work. |
| 15 | `subtract` | Same as restrict | Same | Partial | Same as restrict. |

## Phase 5: Structural Review

### Data Representation

All three implementations use a **sorted array** (`ArraySeqStEphS`/`ArraySeqStPerS`/`ArraySeqMtEphS` of `Pair<K, V>`) as the backing store. The APAS cost specification assumes a **balanced BST** (or similar log-depth structure), which gives O(lg n) for single-element operations (find, insert, delete). The sorted-array representation achieves O(log n) for find (binary search) but O(n) for insert and delete (array rebuild).

This is the fundamental mismatch: the data structure chosen does not match the cost model assumed by the textbook.

### Interface Fidelity

| # | Issue | Severity | Notes |
|---|---|---|---|
| 1 | Ephemeral `map`/`filter` take `&mut self` | Low | Matches ephemeral semantics (modify in place). |
| 2 | Persistent `map`/`filter` return `Self` | Correct | Matches persistent semantics. |
| 3 | `union` in StEph uses merge-style | Medium | APAS defines `union = intersection ∪ diff_a ∪ diff_b`. StPer follows this decomposition; StEph/MtEph use direct merge. Both are correct but StPer is closer to prose. |
| 4 | `insert` signature includes combine fn | Correct | Matches APAS's `insert(f)(a)(k,v) = union f a (singleton(k,v))`. |
| 5 | No `Pair` type alias in prose | Low | Prose uses `K × V` tuples; implementation uses `Pair(K, V)` struct from Types.rs. |

### TOC Headers

None of the 4 source files have TOC headers. This is expected since none use `verus!` blocks.

## Phase 6: Verus Verification Status

**No Verus code exists in Chapter 42.** All 4 files are plain Rust without `verus!` blocks, `requires`/`ensures` clauses, `spec fn` definitions, or any verification-related constructs. This means:

- No formal specifications
- No loop invariants
- No proof functions
- No View implementations
- No ghost state

## Phase 7: Test Coverage

| # | Test Type | Count | Notes |
|---|---|:---:|---|
| 1 | RTT (cargo tests in `tests/`) | 0 | No runtime tests found |
| 2 | PTT (proof-time tests in `rust_verify_test/tests/`) | 0 | No proof-time tests found |

**Example42_1.rs** contains demonstration code (`example_42_1()` and `performance_comparison()`) that exercises the API but is not structured as automated tests with assertions.

## Phase 8: Cost Discrepancy Analysis

The core cost discrepancies stem from the **sorted-array representation**:

| # | Discrepancy | APAS Expects | Implementation Achieves | Root Cause |
|---|---|---|---|---|
| 1 | `insert`/`delete` | O(lg n) work | O(n) work | Array rebuild vs BST node insertion |
| 2 | `intersection`/`union`/`difference` | O(m lg(1+n/m)) work | O(m+n) work | Merge vs divide-and-conquer |
| 3 | `restrict`/`subtract` | O(m lg(1+n/m)) work | O(m+n) work | Linear scan vs BST-based |
| 4 | `domain` | O(\|a\|) work | O(n²) worst-case | Sequential insert into set per entry |
| 5 | Span for single-threaded ops | O(lg n) | O(n) | No parallelism in St variants |

The APAS cost specification is designed for a **balanced BST** (like an AVL or treap) implementation, which Chapter 43 provides. Chapter 42 is the interface chapter; these array-based implementations are functionally correct but do not achieve the specified costs.

## Proof Holes Summary

```
Modules:
   4 clean (no holes)
   0 holed (contains holes)
   4 total

Holes Found: 0 total
```

No proof holes because there is no Verus code. The "clean" status is vacuously true.

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 54 |

All 54 functions have no Verus specifications (`requires`/`ensures`). This is expected since no code is inside `verus!` blocks.

## Overall Assessment

Chapter 42 provides a **functionally complete** implementation of the TABLE ADT (Data Type 42.1) with all 15 operations across three variants (StEph, StPer, MtEph). The implementations are correct in behavior but use a sorted-array backing structure rather than a balanced BST, resulting in asymptotically worse costs for insert, delete, and bulk operations compared to the APAS cost specification.

**Strengths:**
1. Complete ADT coverage: all 15 operations implemented in all 3 variants
2. Clean separation: ephemeral (mut) vs persistent (return Self) semantics
3. MtEph variant uses spawn/join parallelism for map, filter, delete, tabulate
4. Convenient macro literals for all three variants
5. Example42_1 demonstrates interoperability across all variants
6. No proof holes (vacuously)

**Weaknesses:**
1. **No Verus verification** — entirely plain Rust, no specs, no proofs
2. **No tests** — neither RTT nor PTT
3. **Wrong data structure for costs** — sorted array gives O(n) insert/delete instead of O(lg n)
4. **Cost mismatches** — 8 of 15 operations do not meet APAS cost spec
5. **No TOC headers** in any file
6. **collect semantics differ** — APAS collect gathers values by key into sequences; implementation returns flat entries
7. **MtEph intersection/union/difference are sequential** despite being in the multi-threaded file
8. **Algorithm 42.3 not implemented** — the reduce-based collect formulation is absent
9. **Example 42.1 does not match prose** — uses different keys/values than the textbook example
