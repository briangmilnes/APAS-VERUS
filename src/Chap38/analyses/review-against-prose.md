<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 38 — Parametric BSTs: Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap38.txt`
**Source files:** `src/Chap38/BSTParaStEph.rs`, `src/Chap38/BSTParaMtEph.rs`
**Test files:** `tests/Chap38/TestBSTParaStEph.rs` (5 tests), `tests/Chap38/TestBSTParaMtEph.rs` (16 tests)
**PTT files:** None
**Verification status:** No `verus!` blocks — entire chapter is plain Rust, gated with `#[cfg(not(verus_keep_ghost))]`

## Phase 1: Inventory

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap38 | BSTParaMtEph | 16 | 16 | 0 | 14 | 0 | 29 | 0 | 0 | 29 |
| 2 | Chap38 | BSTParaStEph | 12 | 12 | 0 | 8 | 0 | 19 | 0 | 0 | 19 |

**Total:** 48 functions, 0 inside `verus!`, 48 outside, 0 with specs, 0 proof holes.

## Phase 2: Prose Inventory

### Definitions

| # | Prose Item | Reference | Type | Description |
|---|-----------|-----------|------|-------------|
| 1 | Data Type 38.1 — K (key type with total order) | §1 | Type | Key type supporting `<` |
| 2 | Data Type 38.1 — T (abstract tree type) | §1 | Type | Opaque tree type |
| 3 | Data Type 38.1 — E (Exposed: Leaf or Node(T×K×T)) | §1 | Type | Exposed tree variant |
| 4 | Data Type 38.1 — size : T → N | §1 | Fn | Number of keys in T |
| 5 | Data Type 38.1 — expose : T → E | §1 | Fn | Expose the root |
| 6 | Data Type 38.1 — joinMid : E → T | §1 | Fn | Inverse of expose; rebalances |

### Algorithms

| # | Prose Item | Reference | Description |
|---|-----------|-----------|-------------|
| 7 | Algorithm 38.2 — empty | §2 | `joinMid(Leaf)` |
| 8 | Algorithm 38.2 — singleton k | §2 | `joinMid(Node(empty, k, empty))` |
| 9 | Algorithm 38.2 — joinM(L, k, R) | §2 | `joinMid(Node(L, k, R))` |
| 10 | Algorithm 38.3 — split(T, k) | §2 | Three-way recursive split |
| 11 | Algorithm 38.4 — minKey(T, k) | §2 | Find minimum key |
| 12 | Algorithm 38.4 — joinPair(T1, T2) | §2 | Join two trees without a middle key |
| 13 | Algorithm 38.5 — insert(T, k) | §2 | `split` then `joinM(L, k, R)` |
| 14 | Algorithm 38.5 — delete(T, k) | §2 | `split` then `joinPair(L, R)` |
| 15 | Algorithm 38.6 — union(T1, T2) | §3 | Parallel divide-and-conquer via split |
| 16 | Algorithm 38.7 — intersect(T1, T2) | §3 | Parallel, includes key if found in both |
| 17 | Algorithm 38.8 — difference(T1, T2) | §3 | Parallel, excludes key if found |
| 18 | Algorithm 38.9 — filter f T | §3 | Parallel, joinM/joinPair based on predicate |
| 19 | Algorithm 38.10 — reduce f I T | §3 | Parallel, `f(L', f(k, R'))` |

### Cost Specifications

| # | Prose Item | Reference | Description |
|---|-----------|-----------|-------------|
| 20 | Cost Specification 38.11 | §4 | Full cost table for all BST operations |

### Exercises

| # | Prose Item | Reference | Description |
|---|-----------|-----------|-------------|
| 21 | Exercise 38.1 | §3 | Prove correct intersection, difference, and union |

### Implementation Status

| # | Prose Item | StEph | MtEph | Notes |
|---|-----------|:-----:|:-----:|-------|
| 1 | K (key type) | `T: StT + Ord` | `T: MtKey` | Faithful |
| 2 | T (tree type) | `ParamBST<T>` (Rc/RefCell) | `ParamBST<T>` (Arc/RwLock) | Faithful |
| 3 | E (Exposed) | `Exposed<T>` enum | `Exposed<T>` enum | Faithful |
| 4 | size | Yes | Yes | O(1) via stored field |
| 5 | expose | Yes | Yes | `expose_internal` + trait |
| 6 | joinMid | Yes | Yes | `join_mid` free fn + trait |
| 7 | empty | Yes | Yes | `ParamBST::new()` |
| 8 | singleton | **No** | **No** | Available indirectly via `ParamBSTLit![k]` in StEph only |
| 9 | joinM | Yes | Yes | `join_m` wrapper |
| 10 | split | Yes | Yes | `split_inner` — exact match to prose |
| 11 | minKey | Yes | Yes | Returns `Option<T>` instead of prose's two-arg form |
| 12 | joinPair | Yes | Yes | `join_pair_inner` |
| 13 | insert | Yes | Yes | `split` + `joinM` — matches prose |
| 14 | delete | Yes | Yes | `split` + `joinPair` — matches prose |
| 15 | union | Yes (seq) | Yes (par) | MtEph uses `ParaPair!`; StEph sequential |
| 16 | intersect | **No** | Yes (par) | Missing from StEph |
| 17 | difference | **No** | Yes (par) | Missing from StEph |
| 18 | filter | **No** | Yes (par) | Missing from StEph |
| 19 | reduce | **No** | Yes (par) | Missing from StEph |
| 20 | Cost Spec 38.11 | Partial | Yes | Doc-comments on all functions |
| 21 | find | Yes | Yes | Standard BST traversal (not numbered in Ch38 prose) |
| 22 | in_order | Yes | Yes | Extra utility, not in prose |
| 23 | is_empty | Yes | Yes | Extra utility, not in prose |
| 24 | ParamBSTLit! | Yes | No | Convenience macro for StEph |

**StEph:** 14 of 19 prose items implemented. Missing: singleton, intersect, difference, filter, reduce.
**MtEph:** 18 of 19 prose items implemented. Missing: singleton only.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? | Notes |
|---|----------|-----------|-----------|-----------|-----------|:------:|-------|
| 1 | new/empty | O(1) | O(1) | Θ(1) | Θ(1) | Yes | |
| 2 | expose | O(1) | O(1) | Θ(1) | Θ(1) | Yes | |
| 3 | joinMid | O(1) | O(1) | Θ(1) | Θ(1) | Yes | Parametric; balancing deferred to Ch39 |
| 4 | size | O(1) | O(1) | Θ(1) | Θ(1) | Yes | Stored in node |
| 5 | split | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes | |
| 6 | find | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes | |
| 7 | insert | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes | split + joinM |
| 8 | delete | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes | split + joinPair |
| 9 | joinPair | O(lg(\|t1\|+\|t2\|)) | O(lg(\|t1\|+\|t2\|)) | Θ(log(\|t1\|+\|t2\|)) | Θ(log(\|t1\|+\|t2\|)) | Yes | |
| 10 | union (MtEph) | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes | Missing `(_, Leaf)` base case adds O(\|a\|) when b empty |
| 11 | union (StEph) | O(m·lg(n/m)) | — | Θ(m·lg(n/m)) | Θ(m·lg(n/m)) | Yes | Sequential; span = work |
| 12 | intersect | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes | MtEph only |
| 13 | difference | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes | MtEph only |
| 14 | filter | O(\|t\|) | O(lg \|t\|) | Θ(n) | Θ(lg n) | Yes | MtEph only |
| 15 | reduce | O(\|t\|) | O(lg \|t\|) | Θ(n) | Θ(lg n) | Yes | MtEph only |
| 16 | in_order | O(\|t\|) | O(\|t\|) | Θ(n) | Θ(n) | Yes | Not in prose |

### 3b. Implementation Fidelity

| # | Algorithm | File | Fidelity | Detail |
|---|----------|------|:--------:|--------|
| 1 | expose | Both | Faithful | Reads root via borrow/read lock; returns `Leaf` or `Node(L, k, R)` |
| 2 | joinMid | Both | Faithful | Creates `NodeInner` with `size = 1 + left.size() + right.size()` |
| 3 | joinM | Both | Faithful | One-line wrapper `join_mid(Node(L, k, R))` |
| 4 | split | Both | Faithful | Recursive three-way compare, rebuilds with `join_mid` on way back |
| 5 | minKey | Both | Minor deviation | Prose takes `(T, k)` with fallback key; impl returns `Option<T>` — equivalent via `unwrap_or` |
| 6 | joinPair | Both | Minor deviation | Calls `min_key(&right)` on unexposed right tree, re-exposing it; functionally identical |
| 7 | insert | Both | Faithful | `split(T, k)` then `joinM(L, k, R)` |
| 8 | delete | Both | Faithful | `split(T, k)` then `joinPair(L, R)` |
| 9 | union (both) | Both | Missing base case | No check for `(_, Leaf) ⇒ T1`. Traverses all of `a` when `b` is empty — O(\|a\|) instead of O(1). Functionally correct but asymptotically wasteful |
| 10 | intersect | MtEph | Faithful | Both `(Leaf, _)` and `(_, Leaf)` base cases present. Conditional `joinM`/`joinPair` on `found` |
| 11 | difference | MtEph | Faithful | Both base cases present. Inverted join logic |
| 12 | filter | MtEph | Faithful | Parallel left/right, `joinM` when predicate holds, `joinPair` otherwise |
| 13 | reduce | MtEph | Faithful | Parallel `op(L', op(k, R'))`. Associativity requirement undocumented |
| 14 | find | Both | Standard | Not a numbered algorithm in Ch38; standard BST search |

### 3c. Spec Fidelity

Not applicable — no `verus!` blocks, no `requires`/`ensures`.

## Phase 4: Parallelism Review

### BSTParaMtEph.rs

| # | Function | Parallel? | Mechanism | Prose Match? | Notes |
|---|---------|:---------:|-----------|:------------:|-------|
| 1 | `union_inner` | Yes | `ParaPair!` on left/right recursive union | Yes | Matches `(union(L1, L2) \|\| union(R1, R2))` |
| 2 | `intersect_inner` | Yes | `ParaPair!` on left/right recursive intersect | Yes | Matches Algorithm 38.7 |
| 3 | `difference_inner` | Yes | `ParaPair!` on left/right recursive difference | Yes | Matches Algorithm 38.8 |
| 4 | `filter_inner` | Yes | `ParaPair!` on left/right recursive filter | Yes | Matches Algorithm 38.9. Uses `Arc<F>` to share predicate |
| 5 | `reduce_inner` | Yes | `ParaPair!` on left/right recursive reduce | Yes | Matches Algorithm 38.10. Uses `Arc<F>` for operator |
| 6 | `collect_in_order` | No | Sequential traversal | N/A | Not in prose |
| 7 | All other fns | No | Sequential | N/A | expose, joinMid, split, insert, delete, find are sequential per-tree operations |

### Thread Safety

- `ParamBST<T>` uses `Arc<RwLock<Option<Box<NodeInner>>>>` for thread-safe interior mutability.
- `insert`/`delete` use `write()` locks; `expose`/`find`/`split` use `read()` locks.
- `ParaPair!` macro handles fork-join threading.
- No grain control: every recursive call spawns threads regardless of subtree size.
- `insert`/`delete` are NOT concurrent-safe: read + compute + write without holding lock across entire operation. Concurrent tests use loose assertions.

## Phase 5: Runtime Test Review

### TestBSTParaStEph.rs (5 tests)

| # | Test | Operations Covered | Edge Cases? |
|---|------|--------------------|:-----------:|
| 1 | `test_parambstlit_macro_functionality` | `ParamBSTLit![]`, `ParamBSTLit![2,1,3]`, size, find | Empty tree |
| 2 | `para_basic_insert_find` | insert, find, size, is_empty, in_order | 7-element tree |
| 3 | `para_split_and_join_pair` | split, join_pair, in_order | Split at existing key |
| 4 | `para_union_and_delete` | union, delete, find, in_order | Union of disjoint sets |
| 5 | `para_join_mid_expose_roundtrip` | expose, join_mid, size | Leaf and single-node |

### TestBSTParaMtEph.rs (16 tests)

| # | Test | Operations Covered | Edge Cases? |
|---|------|--------------------|:-----------:|
| 1 | `para_basic_insert_find` | insert, find, size, is_empty, in_order | 7-element tree |
| 2 | `para_split_and_join_pair` | split, join_pair, in_order | Split at existing key |
| 3 | `para_union_and_delete` | union, delete, find, in_order | Union of disjoint sets |
| 4 | `para_join_mid_expose_roundtrip` | expose, join_mid, size | Leaf and single-node |
| 5 | `para_intersect_and_difference` | intersect, difference, in_order | Overlapping sets |
| 6 | `para_filter_and_reduce` | filter, reduce | Even filter, sum, empty reduce |
| 7 | `para_union_large_balanced` | union on 200+200 element trees | Overlapping ranges |
| 8 | `para_intersect_and_difference_large` | intersect, difference on 256-element trees | Large overlapping ranges |
| 9 | `para_filter_and_reduce_edge_cases` | filter, reduce with edge cases | Single element, sum of squares |
| 10 | `para_concurrent_insertions` | Concurrent insert + find across 4 threads | Thread safety | Weak assertions |
| 11 | `para_concurrent_operations_stress` | 6-thread concurrent insert + find | Stress test | Weak assertions |
| 12 | `para_concurrent_set_operations` | Concurrent union, intersect, difference | Parallel set ops on read-only trees | Exact assertions |
| 13 | `para_concurrent_filter_reduce` | Concurrent filter, reduce | Parallel read-only operations | Exact assertions |
| 14 | `para_concurrent_split_join` | Concurrent split operations | Parallel read-only splits | Exact assertions |
| 15 | `para_concurrent_expose_join_mid` | Concurrent expose, join_mid | Building trees in parallel |
| 16 | `para_concurrent_delete_operations` | Concurrent delete + find | Concurrent mutation | Weak assertions |

### Coverage Assessment

- **StEph:** Good for implemented operations. Missing tests for intersect, difference, filter, reduce (not implemented in StEph).
- **MtEph:** Excellent functional coverage. All 16 trait methods exercised. Large-tree tests provide confidence.
- **Concurrency tests:** Tests 10, 11, 16 use concurrent mutation with loose assertions (correct given non-linearizable insert/delete). Tests 12–15 use concurrent reads with exact assertions.
- **Missing:** No test for split at non-existent key, union with empty tree, or delete of non-existent key.

## Phase 6: PTT Review

No PTTs exist. Expected since neither file contains `verus!` blocks. The module is gated with `not(verus_keep_ghost)`.

## Phase 7: Gap Analysis

### Prose Items Not Implemented

| # | Priority | Gap | Detail | Recommendation |
|---|----------|-----|--------|----------------|
| 1 | High | Union missing `(_, Leaf) ⇒ T1` base case | Both `union_inner` (StEph and MtEph) only check `expose(a)` for Leaf. When `a` is non-empty but `b` is empty, code traverses all of `a` at O(\|a\|) instead of O(1). | Add base case check |
| 2 | High | StEph missing 4 algorithms | `intersect`, `difference`, `filter`, `reduce` not in StEph. Trait has 12 vs MtEph's 16 methods. | Add sequential implementations |
| 3 | High | No `verus!` blocks | No formal specs or proofs. Entirely unverified. | Future verusification |
| 4 | Medium | `singleton` not a named function | Algorithm 38.2 defines it. `ParamBSTLit![k]` serves as workaround for StEph only. | Add `fn singleton(k: T) -> Self` |
| 5 | Medium | `reduce` associativity undocumented | Prose requires `f` to be associative with identity `I`. Not documented in code. | Add doc comment |
| 6 | Low | Missing edge case tests | No test for split at non-existent key, ops with empty tree, delete of non-existent key | Add tests |
| 7 | Low | `ParamBSTLit!` missing from MtEph | StEph has the macro; MtEph does not. | Add or document omission |

### Code With No Prose Counterpart

| # | Item | Purpose |
|---|------|---------|
| 1 | `find` | Standard BST search — not numbered in Ch38 but mentioned in Cost Spec |
| 2 | `in_order` | Utility for testing and traversal |
| 3 | `is_empty` | Convenience wrapper |
| 4 | `ParamBSTLit!` | Test convenience macro |

## Phase 8: TOC Review

Neither file contains `verus!` blocks, so the 13-section TOC standard does not apply.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:---:|:------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BSTParaStEph.rs | ✅ out (derive) | - | ✅ out (derive on Exposed) | - | - | ✅ out (derive) | - | ✅ out (ParamBSTLit!) | No `verus!` block |
| 2 | BSTParaMtEph.rs | ✅ out (derive) | - | - | - | - | ✅ out (derive) | - | - | No `verus!` block |

## Proof Holes Summary

```
✓ BSTParaMtEph.rs
✓ BSTParaStEph.rs

Modules:     2 clean, 0 holed
Holes Found: 0 total
```

Trivially clean — no `verus!` code exists.

## Spec Strength Summary

| Classification | Count |
|---------------|------:|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 48 |

## Overall Assessment

Chapter 38 faithfully implements the parametric BST abstraction from the APAS prose. The MtEph variant implements 18 of 19 prose items with genuine `ParaPair!` parallelism for union, intersect, difference, filter, and reduce. The StEph variant is incomplete (missing 4 parallel algorithms as sequential variants) and both files have a union base case omission.

**Strengths:**
- All implemented algorithms match the prose pseudocode with high fidelity
- MtEph provides genuine fork-join parallelism via `ParaPair!` for all five parallel algorithms
- 21 runtime tests with good functional and concurrency coverage
- Cost annotations on all functions agree with Cost Specification 38.11

**Weaknesses:**
- No formal verification — entirely plain Rust with zero `verus!` code
- StEph missing intersect, difference, filter, reduce (sequential variants)
- Union missing `(_, Leaf) ⇒ T1` base case in both files — wasteful O(|a|) traversal
- No grain control for thread spawning in MtEph
- `singleton` not exposed as a named function
