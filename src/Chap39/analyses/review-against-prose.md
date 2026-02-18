<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 39 — Treaps: Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6

## Phase 2: Prose Inventory

Source: `prompts/Chap39.txt`

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 39.1 (Treap) | BST over keys K with priority function p : K → Z, satisfying both BST property on keys and max-heap property on priorities. For every internal node u with parent v: p(k(v)) ≥ p(k(u)). |
| 2 | Treap Type | `type T = TLeaf \| TNode of (T × K × Z × T)` — recursive type: (left, key, priority, right). |
| 3 | Exposed Type | `type E = Leaf \| Node of (T × K × T)` — exposed view that hides the priority field. |

### Algorithms and Data Structure Operations (Data Structure 39.3)

| # | Item | Prose Signature | Description |
|---|------|-----------------|-------------|
| 1 | `priority(T)` | `T → Z` | Returns priority of root node, or −∞ for leaf. O(1) work and span. |
| 2 | `join(T₁, (k, p), T₂)` | `T × (K × Z) × T → T` | Core join maintaining BST + heap property. Three cases: (a) p > both subtree priorities → make node directly; (b) priority(T₁) > priority(T₂) → T₁'s root becomes overall root, recurse on R₁; (c) symmetric. |
| 3 | `expose(T)` | `T → E` | TLeaf → Leaf, TNode(L,k,\_,R) → Node(L,k,R). Strips priority. O(1). |
| 4 | `joinMid(E)` | `E → T` | Leaf → TLeaf, Node(L,k,R) → join(L, (k, p(k)), R). Recomputes priority from key. |
| 5 | Algorithm 39.2 (qsTree) | — | Treap-generating quicksort used to prove height bound via quicksort recursion tree isomorphism. Not an ADT operation. |

### Cost Specifications

| # | Operation | Work | Span | Notes |
|---|-----------|------|------|-------|
| 1 | priority | O(1) | O(1) | |
| 2 | join | O(h(T₁) + h(T₂)) = O(log(\|T₁\| + \|T₂\|)) w.h.p. | O(log(\|T₁\| + \|T₂\|)) w.h.p. | Each step goes one level deeper in T₁ or T₂. |
| 3 | expose | O(1) | O(1) | |
| 4 | joinMid | O(log n) w.h.p. | O(log n) w.h.p. | Delegates to join. |
| 5 | split(T, k) | O(h(\|T\|)) = O(log \|T\|) w.h.p. | O(log \|T\|) w.h.p. | Each joinMid in split is O(1) since the key has higher priority than both subtrees (it was an ancestor in the original tree). |

### Theorems and Properties

| # | Item | Description |
|---|------|-------------|
| 1 | Height bound | Treap height is O(lg n) with high probability, proved via isomorphism to quicksort recursion tree. |
| 2 | Exercise 39.1 | If priorities are unique, there is exactly one tree structure satisfying the treap properties. |
| 3 | Priority assignment | Prose assumes p(·) is a random function — always returns the same integer for a given key, but the value is random. Unique priorities assumed. |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All cost annotations are present in the `/// - APAS:` / `/// - Claude-Opus-4.6:` format across all four source files.

### 3b. Implementation Fidelity

| # | Prose Item | Impl File | Function | Fidelity | Notes |
|---|------------|-----------|----------|----------|-------|
| 1 | `join(T₁, (k,p), T₂)` | BSTParaTreapMtEph.rs | `join_with_priority` | **Faithful** | Three-way case structure matches prose exactly: (a) p > both priorities → make node, (b) left priority higher → recurse into left's right subtree, (c) symmetric. |
| 2 | `expose(T)` | BSTParaTreapMtEph.rs | `expose_internal` / `expose` | **Faithful** | Returns Leaf or Node(L,k,R), stripping priority as in prose. |
| 3 | `joinMid(E)` | BSTParaTreapMtEph.rs | `join_mid` | **Faithful** | Leaf→new(), Node(L,k,R)→join\_with\_priority(L,k,priority\_for(k),R). Matches prose definition. |
| 4 | `priority(T)` | BSTParaTreapMtEph.rs | `tree_priority` | **Faithful** | Returns `i64::MIN` for empty tree (≡ −∞), node priority otherwise. |
| 5 | `split` (Ch38 interface) | BSTParaTreapMtEph.rs | `split_inner` | **Faithful** | Recursively splits by comparing key, rebuilds subtrees with `join_with_priority`. Follows the parametric split from Ch38. |
| 6 | Treap type | BSTParaTreapMtEph.rs | `ParamTreap` + `NodeInner` | **Faithful** | `NodeInner` has (key, priority, size, left, right) matching TNode(T×K×Z×T) plus augmented size. `ParamTreap` root wraps in `Arc<RwLock<>>`. |
| 7 | Exposed type | BSTParaTreapMtEph.rs | `Exposed<T>` | **Faithful** | `enum Exposed { Leaf, Node(ParamTreap, T, ParamTreap) }` matches E = Leaf \| Node(T×K×T). |
| 8 | Algorithm 39.2 (qsTree) | — | — | **Not implemented** | Analytical scaffolding for height proof, not an ADT operation. Omission is appropriate. |
| 9 | StEph insert | BSTTreapStEph.rs | `insert_link` | **Alternative** | Uses bottom-up rotation-based insertion (classic Aragon & Seidel) rather than split+join. Same O(log n) w.h.p. cost. Valid alternative implementation strategy. |
| 10 | MtEph insert | BSTTreapMtEph.rs | `insert_link` | **Alternative** | Same rotation-based approach as StEph, wrapped in `Arc<RwLock<>>`. |
| 11 | ParaTreap insert | BSTParaTreapMtEph.rs | `insert` | **Faithful** | Uses split + join\_with\_priority, matching the parametric BST interface from Ch38. |
| 12 | BSTSetTreapMtEph aggregate ops | BSTSetTreapMtEph.rs | union, intersection, difference, split, join\_pair, join\_m, filter, reduce, delete | **Major deviation** | All aggregate operations use sequential BTreeSet/Vec rebuilds: O(n+m) or O(n log n) work and span. Prose achieves O(m·lg(n/m)) work, O(lg n) span. Module header explicitly documents this limitation. |

### 3c. Cost Fidelity Table

| # | Function | File | APAS Work | Impl Work | APAS Span | Impl Span | Match? |
|---|----------|------|-----------|-----------|-----------|-----------|--------|
| 1 | join\_with\_priority | BSTParaTreapMtEph | O(log n) w.h.p. | O(log n) w.h.p. | O(log n) w.h.p. | O(log n) w.h.p. | Yes |
| 2 | expose | BSTParaTreapMtEph | O(1) | O(1) | O(1) | O(1) | Yes |
| 3 | join\_mid | BSTParaTreapMtEph | O(log n) | O(log n) | O(log n) | O(log n) | Yes |
| 4 | tree\_priority | BSTParaTreapMtEph | O(1) | O(1) | O(1) | O(1) | Yes |
| 5 | split\_inner | BSTParaTreapMtEph | O(log n) w.h.p. | O(log n) w.h.p. | O(log n) w.h.p. | O(log n) w.h.p. | Yes |
| 6 | insert (ParaTreap) | BSTParaTreapMtEph | O(lg \|t\|) | O(lg \|t\|) | O(lg \|t\|) | O(lg \|t\|) | Yes |
| 7 | delete (ParaTreap) | BSTParaTreapMtEph | O(lg \|t\|) | O(lg \|t\|) | O(lg \|t\|) | O(lg \|t\|) | Yes |
| 8 | union | BSTParaTreapMtEph | O(m·lg(n/m)) | O(m·lg(n/m)) | O(lg n) | O(lg n) | Yes |
| 9 | intersect | BSTParaTreapMtEph | O(m·lg(n/m)) | O(m·lg(n/m)) | O(lg n) | O(lg n) | Yes |
| 10 | difference | BSTParaTreapMtEph | O(m·lg(n/m)) | O(m·lg(n/m)) | O(lg n) | O(lg n) | Yes |
| 11 | filter | BSTParaTreapMtEph | O(\|t\|) | O(\|t\|) | O(lg \|t\|) | O(lg \|t\|) | Yes |
| 12 | reduce | BSTParaTreapMtEph | O(\|t\|) | O(\|t\|) | O(lg \|t\|) | O(lg \|t\|) | Yes |
| 13 | insert\_link (StEph) | BSTTreapStEph | O(log n) w.h.p. | O(log n) w.h.p. | O(log n) w.h.p. | O(log n) w.h.p. | Yes |
| 14 | union | BSTSetTreapMtEph | O(m·lg(n/m)) | **Θ(n+m)** | O(lg n) | **Θ(n+m)** | **No** |
| 15 | intersection | BSTSetTreapMtEph | O(m·lg(n/m)) | **Θ(n+m)** | O(lg n) | **Θ(n+m)** | **No** |
| 16 | difference | BSTSetTreapMtEph | O(m·lg(n/m)) | **Θ(n+m)** | O(lg n) | **Θ(n+m)** | **No** |
| 17 | split | BSTSetTreapMtEph | O(log n) | **Θ(n)** | O(log n) | **Θ(n)** | **No** |
| 18 | join\_pair | BSTSetTreapMtEph | O(log n) | **Θ(n+m)** | O(log n) | **Θ(n+m)** | **No** |
| 19 | join\_m | BSTSetTreapMtEph | O(log n) | **Θ(n+m)** | O(log n) | **Θ(n+m)** | **No** |
| 20 | delete | BSTSetTreapMtEph | O(log n) | **Θ(n)** | O(log n) | **Θ(n)** | **No** |
| 21 | filter | BSTSetTreapMtEph | Θ(n) | Θ(n) | O(lg n) | **Θ(n)** | **Partial** (work matches, span doesn't) |
| 22 | reduce | BSTSetTreapMtEph | Θ(n) | Θ(n) | O(lg n) | **Θ(n)** | **Partial** (work matches, span doesn't) |

### 3d. Spec Fidelity

**No Verus specifications exist in any Chap39 file.** All functions have `spec_strength: none`. There are no `requires`/`ensures` clauses, no `spec fn` definitions, and no `proof fn` definitions. The `#[cfg(not(verus_keep_ghost))]` gate on the module in `lib.rs` confirms the entire chapter is excluded from Verus verification.

Key specifications that would be needed for eventual verification:
- BST invariant: all keys in left subtree < root key < all keys in right subtree
- Max-heap property on priorities: parent priority ≥ child priorities
- Size correctness: size field = 1 + left.size + right.size
- join postcondition: result is a valid treap containing exactly keys(T₁) ∪ {k} ∪ keys(T₂)
- split postcondition: keys(left) < pivot, keys(right) > pivot, found ↔ pivot ∈ keys(tree)
- expose/joinMid roundtrip: joinMid(expose(t)) has same key set as t

### 3e. Notable Design Decisions

1. **priority\_for uses hashing, not randomness.** `BSTParaTreapMtEph` derives priorities via `Debug` formatting + `Hash`, making them deterministic. The same key always maps to the same priority (matching the prose's p(·) function semantics), but the probabilistic balance guarantee depends on hash distribution rather than true randomness. The StEph and MtEph modules use `rand::Rng` for genuinely random priorities.

2. **Two insertion strategies coexist.** StEph/MtEph use bottom-up rotation-based insertion (classic Aragon & Seidel 1989). ParaTreapMtEph uses top-down split+join matching the Ch38 parametric interface. Both achieve O(log n) w.h.p.

3. **BSTSetTreapMtEph is a compatibility layer.** It wraps BSTTreapMtEph with a set API but uses sequential BTreeSet/Vec-based rebuilds for all aggregate operations. The module header explicitly acknowledges this and directs users to BSTParaTreapMtEph for parallel operations.

4. **priority\_for relies on Debug output for hashing.** The function formats the key via `Debug`, hashes the resulting string, and casts to `i64`. This is fragile: two structurally identical keys with different `Debug` output would get different priorities, and the approach adds allocation overhead. However, it satisfies the prose's requirement that p(·) is a deterministic function of the key.

## Phase 4: Parallelism Review

### 4a. Mt Module Classification

| # | Module | Threading Model | True Parallelism? |
|---|--------|-----------------|--------------------|
| 1 | BSTTreapMtEph | `Arc<RwLock<>>` at root; all ops acquire lock then proceed sequentially. | No — thread-safe but sequential. |
| 2 | BSTSetTreapMtEph | Wraps BSTTreapMtEph. Sequential BTreeSet/Vec rebuilds for aggregates. | No — explicitly sequential. |
| 3 | BSTParaTreapMtEph | `Arc<RwLock<>>` per node. `ParaPair!` for fork-join in union/intersect/difference/filter/reduce. | **Yes** — genuine parallelism matching APAS span bounds. |

### 4b. Parallel Operations in BSTParaTreapMtEph

| # | Function | Uses ParaPair!? | APAS Span | Achieved Span |
|---|----------|:---------------:|-----------|---------------|
| 1 | union\_inner | Yes | O(lg n) | O(lg n) |
| 2 | intersect\_inner | Yes | O(lg n) | O(lg n) |
| 3 | difference\_inner | Yes | O(lg n) | O(lg n) |
| 4 | filter\_inner | Yes | O(lg \|t\|) | O(lg \|t\|) |
| 5 | reduce\_inner | Yes | O(lg \|t\|) | O(lg \|t\|) |
| 6 | join\_with\_priority | No (sequential recursion) | O(log n) | O(log n) |
| 7 | split\_inner | No (sequential recursion) | O(log n) | O(log n) |
| 8 | insert | No (split + join) | O(lg \|t\|) | O(lg \|t\|) |
| 9 | delete | No (split + join\_pair) | O(lg \|t\|) | O(lg \|t\|) |
| 10 | collect\_in\_order | No (sequential) | O(\|t\|) | O(\|t\|) |

### 4c. Parallelism Gap: BSTSetTreapMtEph

| # | Function | APAS Span | Actual Span | Gap |
|---|----------|-----------|-------------|-----|
| 1 | union | O(lg n) | **Θ(n+m)** | Sequential BTreeSet rebuild. |
| 2 | intersection | O(lg n) | **Θ(n+m)** | Sequential filter + rebuild. |
| 3 | difference | O(lg n) | **Θ(n+m)** | Sequential filter + rebuild. |
| 4 | split | O(lg n) | **Θ(n)** | Linear scan + rebuild. |
| 5 | join\_pair | O(lg n) | **Θ(n+m)** | BTreeSet rebuild. |
| 6 | join\_m | O(lg n) | **Θ(n+m)** | BTreeSet rebuild. |
| 7 | filter | O(lg n) | **Θ(n)** | Sequential filter. |
| 8 | reduce | O(lg n) | **Θ(n)** | Sequential fold. |
| 9 | delete | O(lg n) | **Θ(n)** | Vec scan + rebuild. |

All gaps are explicitly documented in the BSTSetTreapMtEph module header.

### 4d. Parallelism Quality Assessment

**BSTParaTreapMtEph** is the flagship parallel implementation and matches all APAS span bounds. The `ParaPair!` fork-join pattern is used correctly for divide-and-conquer over tree structure in all five aggregate operations. The recursive depth equals O(lg n) for balanced trees, giving O(lg n) span as required.

**Concern**: `join_with_priority` and `split_inner` are sequential recursive functions. The prose's cost analysis for split relies on the observation that each joinMid call during split is O(1) because the join key's priority is higher than both subtrees. The implementation correctly handles this — `join_with_priority` will immediately satisfy the first branch (`priority > left_priority && priority > right_priority`) in these cases, making each call O(1) as the prose predicts.

**BSTSetTreapMtEph** exists as a compatibility/testing layer and should not be used where parallel performance matters. Its module header is honest about this limitation.

## Phase 5: RTT (Run-Time Test) Review

### 5a. Coverage Matrix

| # | Source Module | RTT File | Test Count | Status |
|---|-------------|----------|:----------:|--------|
| 1 | BSTTreapStEph.rs | `tests/Chap39/TestBSTTreapStEph.rs` | 30 | Present |
| 2 | BSTTreapMtEph.rs | — | 0 | **Missing RTT** |
| 3 | BSTSetTreapMtEph.rs | `tests/Chap39/TestBSTSetTreapMtEph.rs` | 31 | Present |
| 4 | BSTParaTreapMtEph.rs | `tests/Chap39/TestBSTParaTreapMtEph.rs` | 26 | Present |

### 5b. Test Quality Assessment

**TestBSTTreapStEph.rs** (30 tests):
- Empty tree operations (all return None/0/empty)
- Single element (size, min, max, height)
- Insertion: sequential, reverse-order, random-order, duplicates
- Find: present, missing, negative, extremes (i32::MIN/MAX)
- Contains: after many inserts, step-by-3 pattern
- Min/max: negative numbers, mixed positive/negative
- Height: bounded assertion (< 20 for 64 elements)
- Traversals: in-order sorted correctness, pre-order, empty tree, singleton
- String keys
- Large tree (100 elements)
- Edge: zero value, extreme i32 values
- **Quality: Thorough.** Good edge case and property coverage.

**TestBSTSetTreapMtEph.rs** (31 tests):
- All 18 trait methods tested via direct and fully-qualified trait calls
- Set operations: union (overlap, empty, disjoint), intersection (overlap, disjoint), difference (partial, complete)
- Split, join\_pair, join\_m (including with empty)
- Filter, reduce
- Delete: single, multiple, nonexistent (implicit)
- Duplicate insert (size stays 1)
- Large set (100 elements)
- Negative numbers
- In-order traversal correctness
- as\_tree accessor
- **Quality: Comprehensive.** Exercises every public trait method.

**TestBSTParaTreapMtEph.rs** (26 tests):
- Macro construction (empty, with elements)
- Core ADT: expose (leaf/node), join\_mid (roundtrip), expose\_with\_priority
- Insert, find, delete, split (found/not-found, boundaries, empty)
- join\_pair, union (disjoint, overlap), intersect (overlap, empty), difference (partial, complete)
- Filter (all, none, partial), reduce (sum, empty)
- Large tree (100 elements), string keys
- Sequential operations (interleaved insert/delete/find)
- Duplicate insert
- **Quality: Good.** Covers all ParamTreapTrait methods. Missing: concurrent access stress test.

### 5c. Missing Tests

| # | Priority | Recommendation |
|---|----------|---------------|
| 1 | Medium | Add `tests/Chap39/TestBSTTreapMtEph.rs` for direct BSTTreapMtEph testing (insert, find, contains, min/max, height, traversals, Default trait). |
| 2 | Medium | Add concurrent access tests for BSTTreapMtEph (multiple threads inserting/reading simultaneously). |
| 3 | Low | Add treap structural property tests: verify BST ordering + max-heap property on priorities after operations. |
| 4 | Low | Add BSTParaTreapMtEph concurrent stress test (multiple threads calling union/intersect concurrently). |
| 5 | Low | Test priority\_for determinism: same key → same priority across calls. |

## Phase 6: PTT (Proof-Time Test) Review

No `verus!` blocks exist in any Chap39 source file. The entire chapter is gated with `#[cfg(not(verus_keep_ghost))]` in `lib.rs`, confirming it is excluded from Verus verification. There are no iterators with ghost invariants, no spec functions, and no proof functions.

**No PTTs are needed** until the chapter is verusified. No PTT files exist.

### Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | BSTTreapStEph.rs | TestBSTTreapStEph.rs | — | RTT only |
| 2 | BSTTreapMtEph.rs | — | — | **Missing RTT** |
| 3 | BSTSetTreapMtEph.rs | TestBSTSetTreapMtEph.rs | — | RTT only |
| 4 | BSTParaTreapMtEph.rs | TestBSTParaTreapMtEph.rs | — | RTT only |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Assessment |
|---|------------|-----------|
| 1 | Algorithm 39.2 (qsTree) | Treap-generating quicksort. Analytical tool used to prove height bound via quicksort recursion tree isomorphism. Not an ADT operation — omission is appropriate. |
| 2 | Exercise 39.1 (uniqueness proof) | Requires formal proof that unique priorities → unique tree structure. Would need Verus spec fn for treap structure + proof fn. Not implementable without verusification. |
| 3 | Height bound theorem | O(lg n) height w.h.p. Probabilistic analysis — not directly expressible as a Verus spec. The TestBSTTreapStEph.rs `test_height_balanced` test provides a runtime sanity check (`height < 20` for 64 elements). |

### Code With No Direct Prose Counterpart

| # | Item | File(s) | Notes |
|---|------|---------|-------|
| 1 | `rotate_left`, `rotate_right` | BSTTreapStEph, BSTTreapMtEph | Standard BST rotation helpers for bottom-up insertion. Not in Ch39 prose but standard BST infrastructure. |
| 2 | Rotation-based `insert_link` | BSTTreapStEph, BSTTreapMtEph | Alternative to prose's split/join insertion. Classic Aragon & Seidel approach. Same O(log n) cost. |
| 3 | `height` / `height_rec` | BSTTreapStEph, BSTTreapMtEph | Utility function. Prose discusses height analytically but does not define a height operation. |
| 4 | `in_order`, `pre_order`, traversal helpers | All files | Traversal utilities not in Ch39 prose. Standard BST infrastructure. |
| 5 | BSTSetTreapMtEph (entire module) | BSTSetTreapMtEph.rs | Set interface wrapper not in Ch39 prose. Provides higher-level set API atop the basic treap. |
| 6 | `find` | BSTTreapStEph, BSTTreapMtEph | Standard BST find. Not in Ch39 prose but fundamental BST operation. |
| 7 | `contains`, `is_empty`, `minimum`, `maximum` | All files | Standard convenience operations. Not in Ch39 prose but standard API surface. |
| 8 | `priority_for` (hash-based) | BSTParaTreapMtEph | Deterministic priority function using Debug+Hash. Prose assumes a random function p(·). |
| 9 | `size` field / `update` / `size_link` / `tree_size` | All files | Size augmentation. Prose defers size augmentation to Chapter 40, but implementation includes it early. |
| 10 | `delete` | BSTParaTreapMtEph, BSTSetTreapMtEph | Not in Ch39 prose. Implemented via split (dropping the found key) + join. |
| 11 | `join_pair` (without middle key) | BSTParaTreapMtEph, BSTSetTreapMtEph | Joining two trees without a middle key (split right, use root of right as middle). From Ch38 parametric interface. |
| 12 | `union`, `intersect`, `difference` | BSTParaTreapMtEph, BSTSetTreapMtEph | Set operations from the Ch38 parametric BST interface, not in Ch39 prose specifically. |
| 13 | `filter`, `reduce` | BSTParaTreapMtEph, BSTSetTreapMtEph | Aggregate operations. Not in Ch39 but part of the ADT interface. |
| 14 | `Default` impls | BSTTreapStEph, BSTTreapMtEph | Standard Rust trait impls. |
| 15 | Macros (`BSTTreapStEphLit`, etc.) | All files | Convenience construction macros. |
| 16 | `expose_with_priority` | BSTParaTreapMtEph | Exposes node contents including priority. Not in prose (expose deliberately hides priority). Used internally by join/split. |

### Observations on Extra Code

Items 5-16 are extensions beyond Ch39's scope that either come from the Ch38 parametric BST interface (union, intersect, difference, split, join\_pair, filter, reduce, delete) or are standard BST infrastructure (find, min, max, traversals, size). The BSTSetTreapMtEph compatibility layer (item 5) is the main area of concern due to its asymptotic cost deviation.

Item 9 (size augmentation) is notable: the prose explicitly says "We hold off on implementing a size function until the next chapter when we discuss augmenting trees." The implementation includes it early, which is pragmatically useful but deviates from the textbook's pedagogical ordering.

## Phase 8: TOC Review and In/Out Table

### TOC Status

None of the four source files contain a Table of Contents block. Since no file uses `verus!` blocks, the TOC standard (sections 1-13) does not directly apply. Files are organized as plain Rust modules.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BSTTreapStEph.rs | ✅ out (derive on Node, struct) | - | ✅ out | - | - | ✅ out (derive) | - | ✅ out | — |
| 2 | BSTTreapMtEph.rs | ✅ out (derive on Node, struct) | - | ✅ out | - | - | ✅ out (derive) | - | ✅ out | — |
| 3 | BSTSetTreapMtEph.rs | ✅ out (derive on struct) | - | - | - | - | ✅ out (derive) | - | ✅ out | — |
| 4 | BSTParaTreapMtEph.rs | ✅ out (derive on enum, struct, NodeInner) | - | - | - | - | - | - | ✅ out | — |

All derive impls and macros are outside `verus!` (there is no `verus!` block). This is the expected state for a non-verusified chapter. When verusified:
- Clone should move inside `verus!` with `ensures cloned@ == self@`
- PartialEq/Eq should follow the PartialEqSpecImpl pattern
- Debug should remain outside
- Macros should remain outside

## Proof Holes Summary

```
✓ BSTParaTreapMtEph.rs — CLEAN
✓ BSTSetTreapMtEph.rs  — CLEAN
✓ BSTTreapMtEph.rs     — CLEAN
✓ BSTTreapStEph.rs     — CLEAN

Modules: 4 clean, 0 holed
Holes Found: 0 total
```

Vacuously clean — no Verus verification exists. The entire chapter is plain Rust gated with `#[cfg(not(verus_keep_ghost))]`.

## Review TODOs

| # | Priority | Category | Action |
|---|----------|----------|--------|
| 1 | High | Verification | Verusify BSTParaTreapMtEph as the flagship Ch39 module: add BST invariant spec, heap property spec, size correctness spec, and join/split postconditions. |
| 2 | High | Verification | Define `spec fn valid_treap(t)` capturing BST property ∧ max-heap property on priorities ∧ size correctness. |
| 3 | High | Verification | Add `requires`/`ensures` to `join_with_priority`: requires valid treaps + BST ordering (T₁ < k < T₂), ensures result is valid treap with correct key set. |
| 4 | High | Verification | Add `requires`/`ensures` to `split_inner`: requires valid treap, ensures left < pivot, right > pivot, found ↔ pivot ∈ keys. |
| 5 | Medium | Testing | Add `tests/Chap39/TestBSTTreapMtEph.rs` with direct tests for BSTTreapMtEph (currently tested only indirectly through BSTSetTreapMtEph). |
| 6 | Medium | Testing | Add concurrent access tests for BSTTreapMtEph (multiple reader/writer threads). |
| 7 | Medium | Design | Consider deprecating BSTSetTreapMtEph in favor of a set wrapper around BSTParaTreapMtEph, which would provide correct asymptotic costs for all operations. |
| 8 | Medium | Fidelity | Evaluate whether `priority_for`'s hash-based approach provides sufficient randomness properties for the O(lg n) height guarantee. Consider switching to a proper random hash or recording priorities per key. |
| 9 | Low | Fidelity | The prose defers size augmentation to Ch40. Document that Ch39 includes size augmentation early for practical reasons. |
| 10 | Low | Testing | Add structural property tests verifying BST ordering and max-heap property after operations. |
| 11 | Low | Testing | Add `test_priority_for_determinism` verifying the same key always maps to the same priority. |
| 12 | Low | Exercise | Exercise 39.1 (uniqueness of treap structure) could be a future Verus proof goal. |
