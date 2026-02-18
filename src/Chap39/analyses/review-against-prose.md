<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 39 — Treaps: Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6

## Phase 1: Inventory (tool-generated)

103 entries extracted across 4 source files by `veracity-review-module-fn-impls -d src/Chap39`.

### Files

| # | File | Functions |
|---|------|----------|
| 1 | `BSTParaTreapMtEph.rs` | 33 |
| 2 | `BSTSetTreapMtEph.rs` | 23 |
| 3 | `BSTTreapMtEph.rs` | 23 |
| 4 | `BSTTreapStEph.rs` | 24 |

**Verus status:** None of the files use `verus!` blocks. All code is plain Rust with no formal specifications.

## Phase 2: Prose Inventory

Source: `prompts/Chap39.txt`

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 39.1 (Treap) | BST over keys K with priority function p : K → Z, satisfying both BST property on keys and heap property on priorities. |
| 2 | Treap Type | `type T = TLeaf \| TNode of (T × K × Z × T)` — recursive type with (left, key, priority, right). |
| 3 | Exposed Type | `type E = Leaf \| Node of (T × K × T)` — exposed view hiding priority. |

### Algorithms / Data Structure Operations

| # | Item | Description |
|---|------|-------------|
| 1 | Algorithm 39.2 (qsTree) | Treap-generating quicksort — picks max-priority key as pivot, partitions, recurses in parallel. |
| 2 | Data Structure 39.3 — `priority(T)` | Returns priority of root, or −∞ for leaf. O(1). |
| 3 | Data Structure 39.3 — `join(T₁, (k,p), T₂)` | Core join operation maintaining BST + heap property. O(log n) w.h.p. |
| 4 | Data Structure 39.3 — `expose(T)` | Converts TNode to Node (stripping priority) or TLeaf to Leaf. O(1). |
| 5 | Data Structure 39.3 — `joinMid(E)` | Converts Exposed back to Treap: Leaf→TLeaf, Node(L,k,R)→join(L,(k,p(k)),R). |

### Cost Specs

| # | Operation | Work | Span | Notes |
|---|-----------|------|------|-------|
| 1 | priority | O(1) | O(1) | |
| 2 | join | O(h(T₁) + h(T₂)) = O(log(\|T₁\| + \|T₂\|)) w.h.p. | O(log(\|T₁\| + \|T₂\|)) w.h.p. | |
| 3 | expose | O(1) | O(1) | |
| 4 | joinMid | O(log n) | O(log n) | Via join. |
| 5 | split(T) | O(h(\|T\|)) = O(log \|T\|) w.h.p. | O(log \|T\|) w.h.p. | Each join in split is O(1) since the key has higher priority than both subtrees. |

### Theorems / Properties

| # | Item |
|---|------|
| 1 | Height of a treap is O(lg n) with high probability (via quicksort recursion tree isomorphism). |
| 2 | If priorities are unique, there is exactly one tree structure satisfying the treap properties (Exercise 39.1). |

### Exercises

| # | Item |
|---|------|
| 1 | Exercise 39.1 — Prove uniqueness of treap structure given unique priorities. |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All cost annotations have been added to source files in the `/// - APAS:` / `/// - Claude-Opus-4.6:` format.

### 3b. Implementation Fidelity

| # | Prose Item | Impl File | Notes |
|---|------------|-----------|-------|
| 1 | **join (Data Structure 39.3)** | `BSTParaTreapMtEph.rs` → `join_with_priority` | Faithful implementation. Three-way case: if p > both priorities, make node; else recurse into the higher-priority subtree. Matches prose exactly. |
| 2 | **expose** | `BSTParaTreapMtEph.rs` → `expose_internal` | Faithful. Returns Leaf or Node(L,k,R), stripping priority. |
| 3 | **joinMid** | `BSTParaTreapMtEph.rs` → `join_mid` | Faithful. Leaf→new(), Node(L,k,R)→join_with_priority(L,k,priority_for(k),R). |
| 4 | **priority** | `BSTParaTreapMtEph.rs` → `tree_priority` | Faithful. Returns `i64::MIN` for leaf, node priority otherwise. |
| 5 | **split** | `BSTParaTreapMtEph.rs` → `split_inner` | Faithful. Recursively splits by comparing key, rebuilds with join_with_priority. |
| 6 | **qsTree (Alg 39.2)** | Not implemented | No direct implementation of the treap-generating quicksort. This is analytical scaffolding in the prose, not a required ADT operation. |
| 7 | **BSTTreapStEph insert** | `BSTTreapStEph.rs` → `insert_link` | Uses rotation-based insertion with random priorities (classic bottom-up treap insert). This is a standard alternative to the prose's join-based approach. The cost is the same O(log n) w.h.p. |
| 8 | **BSTTreapMtEph insert** | `BSTTreapMtEph.rs` → `insert_link` | Same rotation-based approach as StEph, wrapped in `Arc<RwLock<>>`. |
| 9 | **BSTParaTreapMtEph insert** | `BSTParaTreapMtEph.rs` → `insert` | Uses split + join_with_priority. Matches the parametric BST interface approach from Ch38. |
| 10 | **BSTSetTreapMtEph aggregate ops** | `BSTSetTreapMtEph.rs` | **Major deviation**: union, intersection, difference, split, join_pair, join_m, delete, and filter all use sequential `BTreeSet`-based or `Vec`-based rebuilds. These are O(n) or O(n log n) where the prose (via Ch38 parametric interface + treap join) achieves O(m·lg(n/m)) work and O(lg n) span. This file explicitly documents this limitation in its module header. |

### 3c. Spec Fidelity

**No Verus specifications exist in any Chap39 file.** All 103 functions have `spec_strength: none`. There are no `requires`/`ensures` clauses, no `spec fn` definitions, and no `proof fn` definitions. The entire chapter is plain Rust without formal verification.

Key specs that would be needed for verification:
- BST invariant (all left keys < root key < all right keys)
- Heap property on priorities (parent priority ≥ child priorities)
- Size correctness (size field = 1 + left.size + right.size)
- join postcondition: result is a valid treap containing exactly left ∪ {k} ∪ right
- split postcondition: left < pivot, right > pivot, found iff pivot ∈ tree

## Phase 4: Parallelism Review

### 4a. Mt Module Classification

**BSTTreapMtEph.rs**: Thread-safe wrapper using `Arc<RwLock<>>` at the root. All operations acquire the lock and proceed sequentially. No actual parallelism.

**BSTSetTreapMtEph.rs**: Wraps BSTTreapMtEph. All aggregate operations (union, intersection, difference, filter, reduce) are sequential. Explicitly documented as such.

**BSTParaTreapMtEph.rs**: Genuinely parallel. Uses `ParaPair!` macro for fork-join parallelism in union, intersection, difference, filter, and reduce. This is the only module achieving the APAS span bounds.

### 4b–4c. Parallelism Gap Table

| # | Module | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|--------|----------|-----------|-------------|-----------|-------|
| 1 | BSTTreapMtEph | insert | O(lg n) | O(lg n) | Sequential | Single-threaded with lock; span = work. |
| 2 | BSTTreapMtEph | find | O(lg n) | O(lg n) | Sequential | Single-threaded read lock. |
| 3 | BSTSetTreapMtEph | union | O(lg n) | Θ(n + m) | Sequential | BTreeSet rebuild, not parallel. |
| 4 | BSTSetTreapMtEph | intersection | O(lg n) | Θ(n + m) | Sequential | Sequential filter + rebuild. |
| 5 | BSTSetTreapMtEph | difference | O(lg n) | Θ(n + m) | Sequential | Sequential filter + rebuild. |
| 6 | BSTSetTreapMtEph | split | O(lg n) | Θ(n) | Sequential | Linear scan + rebuild. |
| 7 | BSTSetTreapMtEph | filter | O(lg n) | Θ(n) | Sequential | Sequential filter. |
| 8 | BSTSetTreapMtEph | reduce | O(lg n) | Θ(n) | Sequential | Sequential fold. |
| 9 | BSTParaTreapMtEph | union | O(lg n) | O(lg n) | **Parallel** | ParaPair! fork-join. |
| 10 | BSTParaTreapMtEph | intersect | O(lg n) | O(lg n) | **Parallel** | ParaPair! fork-join. |
| 11 | BSTParaTreapMtEph | difference | O(lg n) | O(lg n) | **Parallel** | ParaPair! fork-join. |
| 12 | BSTParaTreapMtEph | filter | O(lg |t|) | O(lg |t|) | **Parallel** | ParaPair! fork-join. |
| 13 | BSTParaTreapMtEph | reduce | O(lg |t|) | O(lg |t|) | **Parallel** | ParaPair! fork-join. |

## Phase 5: Runtime Test Review

### 5a. Coverage

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | `BSTTreapStEph.rs` | `tests/Chap39/TestBSTTreapStEph.rs` | Present |
| 2 | `BSTTreapMtEph.rs` | (none) | **Missing RTT** |
| 3 | `BSTSetTreapMtEph.rs` | `tests/Chap39/TestBSTSetTreapMtEph.rs` | Present |
| 4 | `BSTParaTreapMtEph.rs` | `tests/Chap39/TestBSTParaTreapMtEph.rs` | Present |

**BSTTreapMtEph** has no dedicated test file, though its functionality is exercised indirectly through BSTSetTreapMtEph tests.

### 5b. Test Quality

**TestBSTTreapStEph.rs** (30 tests): Thorough coverage including empty tree, single element, large tree, duplicates, negative numbers, extremes (i32::MIN/MAX), reverse-order inserts, string keys, in-order traversal correctness. Good edge case coverage.

**TestBSTSetTreapMtEph.rs** (31 tests): Comprehensive trait-level tests for all operations. Includes union, intersection, difference, split, join_pair, join_m, filter, reduce, delete. Tests both direct calls and fully-qualified trait calls.

**TestBSTParaTreapMtEph.rs** (22 tests): Good coverage of parametric treap operations including expose, join_mid, split, join_pair, union, intersect, difference, filter, reduce. Tests empty trees, boundaries, string keys, disjoint/complete operations.

### 5c. Missing Tests

| # | Priority | Recommendation |
|---|----------|---------------|
| 1 | Medium | Add `tests/Chap39/TestBSTTreapMtEph.rs` for direct BSTTreapMtEph testing, especially concurrent access patterns. |
| 2 | Low | Add treap structural property tests (verify BST ordering + heap property on priorities after operations). |

## Phase 6: Proof-Time Test (PTT) Review

No `verus!` blocks exist in any Chap39 source file. There are no iterators, no verified loops, no spec functions, and no proof functions. **No PTTs are needed** until the chapter is verusified.

No PTT files exist in `rust_verify_test/tests/Chap39/`.

### 6a. Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | `BSTTreapStEph.rs` | `TestBSTTreapStEph.rs` | — | RTT only |
| 2 | `BSTTreapMtEph.rs` | — | — | **Missing RTT** |
| 3 | `BSTSetTreapMtEph.rs` | `TestBSTSetTreapMtEph.rs` | — | RTT only |
| 4 | `BSTParaTreapMtEph.rs` | `TestBSTParaTreapMtEph.rs` | — | RTT only |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Notes |
|---|------------|-------|
| 1 | Algorithm 39.2 (qsTree) | Treap-generating quicksort. Analytical tool for height analysis, not an ADT operation. Not needed as code. |
| 2 | Exercise 39.1 | Uniqueness proof. Would require a Verus proof, not currently possible without verusification. |

### Code With No Prose Counterpart

| # | Function | File | Notes |
|---|----------|------|-------|
| 1 | `rotate_left`, `rotate_right` | BSTTreapStEph, BSTTreapMtEph | Standard BST rotation helpers. Not in Ch39 prose but standard BST infrastructure. |
| 2 | `insert_link` (rotation-based) | BSTTreapStEph, BSTTreapMtEph | Bottom-up insertion with rotations. Alternative to prose's split/join approach; equivalent cost. |
| 3 | `height`, `height_rec` | Both StEph and MtEph | Utility function. Prose mentions height analytically but doesn't define a height function. |
| 4 | `in_order`, `pre_order`, traversal helpers | All files | Traversal utilities not in Ch39 prose. |
| 5 | `BSTSetTreapMtEph` (entire module) | BSTSetTreapMtEph.rs | Set interface wrapper. Not in Ch39 prose; provides a higher-level set API atop the treap. |
| 6 | `values_vec`, `rebuild_from_vec`, `from_sorted_iter` | BSTSetTreapMtEph.rs | Internal helpers for the sequential set implementation. |
| 7 | `contains`, `is_empty` | All files | Trivial wrappers. Not in prose but standard API. |
| 8 | `priority_for` (hash-based) | BSTParaTreapMtEph.rs | Uses Debug+Hash to derive deterministic priorities. Prose assumes a random function p(·). |
| 9 | `new_node`, `make_node` | Various | Node constructors. |
| 10 | `size_link`, `update`, `tree_size` | Various | Size maintenance helpers. |
| 11 | `Default` impls | StEph, MtEph | Standard Rust trait impls. |
| 12 | Macros (`BSTTreapStEphLit`, etc.) | All files | Convenience construction macros. |

### Notable Design Decisions

1. **priority_for uses hashing, not randomness.** The `BSTParaTreapMtEph` module derives priorities from `Debug` + `Hash` of the key, making them deterministic. This means the same key always gets the same priority (matching the prose's p(·) function requirement), but the distribution depends on the hash function quality rather than true randomness. The StEph and MtEph modules use `rand::Rng` for truly random priorities.

2. **Two insertion strategies.** StEph/MtEph use bottom-up rotation-based insertion (classic Aragon & Seidel). ParaTreapMtEph uses top-down split+join (matching the Ch38 parametric interface). Both achieve O(log n) w.h.p.

3. **BSTSetTreapMtEph is a compatibility layer.** It wraps BSTTreapMtEph with set operations but achieves worse asymptotic costs than the prose because it lacks access to split/join primitives. BSTParaTreapMtEph is the proper parallel implementation.

## Phase 8: Table of Contents Review

None of the four source files have a Table of Contents block. None use `verus!` blocks, so the TOC standard sections 1-11 (inside verus!) do not apply. The files are organized as plain Rust modules.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BSTTreapStEph.rs | - | - | ✅ out | - | - | ✅ out | - | ✅ out | — |
| 2 | BSTTreapMtEph.rs | - | - | ✅ out | - | - | ✅ out | - | ✅ out | — |
| 3 | BSTSetTreapMtEph.rs | - | - | - | - | - | ✅ out | - | ✅ out | — |
| 4 | BSTParaTreapMtEph.rs | - | - | - | - | - | - | - | ✅ out | — |

All derive impls and macros are correctly outside `verus!` (since there is no `verus!` block). Debug derives on Node types are appropriately outside. Default impls delegate to `new()`. No TOC headers present — acceptable since files are not verusified.

## Proof Holes Summary

```
✓ BSTParaTreapMtEph.rs
✓ BSTSetTreapMtEph.rs
✓ BSTTreapMtEph.rs
✓ BSTTreapStEph.rs

Modules: 4 clean, 0 holed
Holes Found: 0 total
```

No proof holes because no Verus verification exists. This is vacuously clean — the code has no formal specifications at all.

## Spec Strength Summary

| Classification | Count |
|---------------|-------|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 103 |

All 103 functions have `spec_strength: none`. The entire chapter is unverified plain Rust.

## Overall Assessment

Chapter 39 provides a solid **runtime implementation** of treaps across four modules spanning three concurrency profiles:

1. **BSTTreapStEph** — single-threaded, rotation-based insert. Correct and well-tested.
2. **BSTTreapMtEph** — thread-safe wrapper with `Arc<RwLock<>>`. All operations sequential under lock.
3. **BSTSetTreapMtEph** — set interface atop MtEph. Sequential aggregate ops (explicitly documented). Serves as compatibility/testing layer.
4. **BSTParaTreapMtEph** — parametric treap with genuine fork-join parallelism via `ParaPair!`. Implements the Ch38 ADT interface (expose/joinMid). This is the flagship implementation matching the prose's algorithmic vision.

### Strengths
- **BSTParaTreapMtEph** faithfully implements Data Structure 39.3 with correct parallel union/intersect/difference/filter/reduce.
- The `join_with_priority` algorithm exactly matches the prose's three-case join.
- Good runtime test coverage (3 out of 4 modules have dedicated test files).
- BSTSetTreapMtEph honestly documents its sequential limitations.

### Gaps
- **No Verus verification.** Zero spec functions, zero proof functions, zero requires/ensures. This is the largest gap.
- **No PTTs.** Cannot have PTTs without verusification.
- **Missing RTT** for BSTTreapMtEph (tested indirectly via BSTSetTreapMtEph).
- **BSTSetTreapMtEph cost gap.** Aggregate operations are O(n) where the prose achieves O(m·lg(n/m)). This is explicitly documented but represents a significant deviation.
- **priority_for** in BSTParaTreapMtEph uses hash-based deterministic priorities rather than random priorities. This may not provide the same probabilistic balance guarantees as the prose's random p(·).

### Recommendations (prioritized)

| # | Priority | Action |
|---|----------|--------|
| 1 | High | Verusify BSTParaTreapMtEph with BST invariant, heap property, and size correctness specs. |
| 2 | High | Add `spec fn` for treap validity (BST property ∧ heap property). |
| 3 | Medium | Add TestBSTTreapMtEph.rs with direct and concurrent access tests. |
| 4 | Medium | Consider whether BSTSetTreapMtEph should be deprecated in favor of BSTParaTreapMtEph for all use cases. |
| 5 | Low | Add structural property runtime tests (verify BST ordering after operations). |
