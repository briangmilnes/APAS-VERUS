<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 38 — Parametric BSTs: Review Against Prose

- **Date**: 2026-02-13
- **Reviewer**: Claude-Opus-4.6
- **Prose source**: `prompts/Chap38.txt`
- **Source files**: `src/Chap38/BSTParaStEph.rs`, `src/Chap38/BSTParaMtEph.rs`
- **Test files**: `tests/Chap38/TestBSTParaStEph.rs`, `tests/Chap38/TestBSTParaMtEph.rs`
- **PTT files**: None

## Phase 1: Structural Completeness

### Prose algorithms and data types

| # | Prose Item | Type | StEph | MtEph | Notes |
|---|-----------|------|:-----:|:-----:|-------|
| 1 | Data Type 38.1 — Parametric BST (K, T, E, size, expose, joinMid) | ADT | Yes | Yes | `Exposed` enum + `ParamBST` struct + `ParamBSTTrait` |
| 2 | Algorithm 38.2 — empty, singleton, joinM | Fn | Partial | Partial | `new()` = empty, `join_m` present; `singleton` missing as named function (available via `ParamBSTLit![k]` in StEph only) |
| 3 | Algorithm 38.3 — split | Fn | Yes | Yes | `split_inner` + trait `split` |
| 4 | Algorithm 38.4 — minKey, joinPair | Fn | Yes | Yes | `min_key` + `join_pair_inner` |
| 5 | Algorithm 38.5 — insert, delete | Fn | Yes | Yes | Use split + joinM / joinPair pattern from prose |
| 6 | Algorithm 38.6 — union (parallel) | Fn | Yes (seq) | Yes (parallel) | StEph sequential; MtEph uses `ParaPair!` |
| 7 | Algorithm 38.7 — intersect (parallel) | Fn | No | Yes (parallel) | StEph missing entirely |
| 8 | Algorithm 38.8 — difference (parallel) | Fn | No | Yes (parallel) | StEph missing entirely |
| 9 | Algorithm 38.9 — filter (parallel) | Fn | No | Yes (parallel) | StEph missing entirely |
| 10 | Algorithm 38.10 — reduce (parallel) | Fn | No | Yes (parallel) | StEph missing entirely |
| 11 | Cost Spec 38.11 | Spec | N/A | N/A | Cost annotations present on all functions |
| 12 | find (mentioned in Cost Spec 38.11) | Fn | Yes | Yes | Standard BST find traversal |

### Summary

- **StEph** implements the core ADT (expose, joinMid, size), split, insert, delete, find, union, joinPair, and in_order. Missing: `singleton` as a named function, `intersect`, `difference`, `filter`, `reduce`.
- **MtEph** implements all prose algorithms including the parallel set operations (union, intersect, difference, filter, reduce). Missing: `singleton` as a named function.
- Neither file has a `verus!` block — these are pure Rust implementations with no formal verification.

## Phase 2: Algorithmic Fidelity

| # | Algorithm | Fidelity | Notes |
|---|----------|----------|-------|
| 1 | expose | Faithful | Reads root, returns `Leaf` or `Node(L, k, R)` — matches prose exactly |
| 2 | joinMid | Faithful | Creates `None` for leaf or builds `NodeInner` with computed size — matches prose |
| 3 | split | Faithful | Recursive split with `Less`/`Greater`/`Equal` three-way comparison, rebuilds with `join_mid` — matches Algorithm 38.3 |
| 4 | joinM | Faithful | Wrapper around `join_mid(Node(L, k, R))` — matches Algorithm 38.2 |
| 5 | minKey | Faithful | Follows left spine — matches Algorithm 38.4 |
| 6 | joinPair | Faithful | Finds min key in right tree, splits, joins — matches Algorithm 38.4 |
| 7 | insert | Faithful | `split(T, k)` then `joinM(L, k, R)` — matches Algorithm 38.5 |
| 8 | delete | Faithful | `split(T, k)` then `joinPair(L, R)` — matches Algorithm 38.5 |
| 9 | union (MtEph) | Faithful | Expose root of `a`, split `b` by `ak`, parallel recursive union — matches Algorithm 38.6 |
| 10 | union (StEph) | Sequential variant | Same structure as Algorithm 38.6 but calls are sequential, not parallel |
| 11 | intersect | Faithful | Parallel with split + conditional joinM/joinPair — matches Algorithm 38.7 |
| 12 | difference | Faithful | Parallel with inverted joinM/joinPair condition — matches Algorithm 38.8 |
| 13 | filter | Faithful | Parallel with predicate test + joinM/joinPair — matches Algorithm 38.9 |
| 14 | reduce | Faithful | Parallel with `op(L', op(k, R'))` combination — matches Algorithm 38.10 |
| 15 | find | Standard BST | Not explicitly given as an algorithm in Ch38 prose, but follows standard BST find; correct |

## Phase 3: Specification Fidelity

**Not applicable.** Neither file contains `verus!` blocks. There are no `requires`/`ensures` specifications, no spec functions, and no proof functions. All 48 functions have spec strength **none**.

The implementations are executable Rust code without formal verification. This is consistent with the project status — Chapter 38 has not yet been verusified.

## Phase 4: Data Structure Fidelity

| # | Aspect | Prose | StEph | MtEph | Match? |
|---|--------|-------|-------|-------|--------|
| 1 | Key type | `K` (total order) | `T: StT + Ord` | `T: MtKey` | Yes — `Ord` provides total order |
| 2 | Tree type `T` | Abstract tree | `ParamBST<T>` with `Rc<RefCell<Option<Box<NodeInner>>>>` | `ParamBST<T>` with `Arc<RwLock<Option<Box<NodeInner>>>>` | Yes — interior mutability for in-place insert/delete |
| 3 | Exposed type `E` | `Leaf \| Node(T, K, T)` | `enum Exposed { Leaf, Node(ParamBST, T, ParamBST) }` | Same | Yes |
| 4 | Size stored in node | Yes (for O(1) access) | `NodeInner.size: N` | Same | Yes |
| 5 | No values (keys only) | Prose mentions optional values but uses keys-only | Keys only | Keys only | Yes |

### Notes

- The prose's `joinMid` is described as potentially performing rebalancing. The parametric implementation here does **not** rebalance — it simply constructs a node. This is intentional: the parametric BST is meant to be subclassed with a specific balancing scheme (Chapter 39 implements Treaps). The code faithfully captures the parametric interface.
- StEph uses `Rc<RefCell>` (single-threaded mutable reference counting), MtEph uses `Arc<RwLock>` (thread-safe) — standard pattern in the codebase.

## Phase 5: Cost Analysis

| # | Operation | APAS Work | APAS Span | Claude Work | Claude Span | Match? |
|---|----------|-----------|-----------|-------------|-------------|--------|
| 1 | new / empty | O(1) | O(1) | Θ(1) | Θ(1) | Yes |
| 2 | expose | O(1) | O(1) | Θ(1) | Θ(1) | Yes |
| 3 | joinMid | O(1) | O(1) | Θ(1) | Θ(1) | Yes — parametric version is O(1); balancing schemes may differ |
| 4 | size | O(1) | O(1) | Θ(1) | Θ(1) | Yes — stored in node |
| 5 | split | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes |
| 6 | find | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes |
| 7 | insert | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes |
| 8 | delete | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes |
| 9 | joinPair | O(lg(\|t1\|+\|t2\|)) | O(lg(\|t1\|+\|t2\|)) | Θ(log(\|t1\|+\|t2\|)) | Θ(log(\|t1\|+\|t2\|)) | Yes |
| 10 | union (MtEph) | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes |
| 11 | union (StEph) | O(m·lg(n/m)) | O(m·lg(n/m)) | Θ(m·lg(n/m)) | Θ(m·lg(n/m)) | Yes — sequential, span = work |
| 12 | intersect | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes |
| 13 | difference | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes |
| 14 | filter | O(\|t\|) | O(lg \|t\|) | Θ(n) | Θ(lg n) | Yes |
| 15 | reduce | O(\|t\|) | O(lg \|t\|) | Θ(n) | Θ(lg n) | Yes |
| 16 | in_order | O(\|t\|) | O(\|t\|) | Θ(n) | Θ(n) | Yes — sequential traversal |

All cost annotations agree with the APAS Cost Specification 38.11.

## Phase 6: Test Coverage

### StEph Tests (`tests/Chap38/TestBSTParaStEph.rs`)

| # | Test | Covers |
|---|------|--------|
| 1 | `test_parambstlit_macro_functionality` | Empty and populated BST via `ParamBSTLit!` macro |
| 2 | `para_basic_insert_find` | insert, find, size, is_empty, in_order |
| 3 | `para_split_and_join_pair` | split, join_pair, in_order verification |
| 4 | `para_union_and_delete` | union, delete, find, in_order |
| 5 | `para_join_mid_expose_roundtrip` | expose, join_mid round-trip |

### MtEph Tests (`tests/Chap38/TestBSTParaMtEph.rs`)

| # | Test | Covers |
|---|------|--------|
| 1 | `para_basic_insert_find` | insert, find, size, is_empty, in_order |
| 2 | `para_split_and_join_pair` | split, join_pair |
| 3 | `para_union_and_delete` | union, delete |
| 4 | `para_join_mid_expose_roundtrip` | expose, join_mid |
| 5 | `para_intersect_and_difference` | intersect, difference |
| 6 | `para_filter_and_reduce` | filter, reduce |
| 7 | `para_union_large_balanced` | union on 200+ element trees |
| 8 | `para_intersect_and_difference_large` | intersect, difference on 256+ element trees |
| 9 | `para_filter_and_reduce_edge_cases` | filter, reduce edge cases |
| 10 | `para_concurrent_insertions` | Concurrent multi-threaded insert + find |
| 11 | `para_concurrent_operations_stress` | 6-thread concurrent stress test |
| 12 | `para_concurrent_set_operations` | Concurrent union, intersect, difference |
| 13 | `para_concurrent_filter_reduce` | Concurrent filter, reduce |
| 14 | `para_concurrent_split_join` | Concurrent split operations |
| 15 | `para_concurrent_expose_join_mid` | Concurrent expose, join_mid |
| 16 | `para_concurrent_delete_operations` | Concurrent delete operations |

### Coverage Assessment

- MtEph has excellent test coverage including concurrency stress tests.
- StEph covers all implemented operations but does not test `intersect`, `difference`, `filter`, or `reduce` (since they are not implemented).
- No proof-time tests (PTTs) exist — expected since there are no `verus!` blocks.

## Phase 7: TOC Headers

Neither file has TOC section headers. Since these are pure Rust files without `verus!` blocks, the standard 13-section TOC structure does not directly apply. However, both files follow a logical organization:

| # | File | Structure |
|---|------|-----------|
| 1 | BSTParaStEph.rs | module → imports → type defs → trait → free functions → impl block → macro |
| 2 | BSTParaMtEph.rs | module → imports → type defs → trait → free functions → impl block |

## Phase 8: Gaps and Recommendations

| # | Priority | Gap | Recommendation |
|---|----------|-----|----------------|
| 1 | Medium | StEph missing `intersect`, `difference`, `filter`, `reduce` | Add sequential implementations to match the trait in MtEph |
| 2 | Low | No `singleton` named function | The `ParamBSTLit![k]` macro serves this role for StEph; MtEph has no equivalent. Consider adding `fn singleton(k: T) -> Self { let t = Self::new(); t.insert(k); t }` |
| 3 | High | No `verus!` blocks or formal specs | Chapter 38 has not been verusified. Future work: add View, spec functions, requires/ensures |
| 4 | Low | No TOC headers | Consider adding section comments for consistency, even in non-verus files |
| 5 | Low | `BSTParaStEph.rs.bak` and `BSTParaMtEph.rs.bak` in directory | Remove backup files |
| 6 | Medium | StEph and MtEph traits differ | StEph has 12 trait methods, MtEph has 16 — the trait interfaces should match for consistency |

## Proof Holes Summary

```
✓ BSTParaMtEph.rs
✓ BSTParaStEph.rs

Modules:   2 clean, 0 holed
Holes Found: 0 total
```

No proof holes. This is trivially true since neither file contains `verus!` blocks — there are no proofs to have holes in.

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 48 |

All 48 functions have **no** requires/ensures specifications. The implementations are pure executable Rust without formal verification.

## Overall Assessment

Chapter 38 provides a clean, faithful implementation of the Parametric BST interface from the APAS textbook. The algorithmic fidelity is high — all implemented functions match their prose descriptions closely. The MtEph variant correctly uses `ParaPair!` for parallel execution of union, intersect, difference, filter, and reduce. Cost annotations are accurate and match Cost Specification 38.11.

The main gaps are:
1. **No formal verification** — the chapter is entirely unverified. This is the most significant gap.
2. **StEph completeness** — the single-threaded variant is missing 4 of the parallel algorithms (intersect, difference, filter, reduce), even though sequential implementations would be straightforward.
3. **Trait parity** — the StEph and MtEph traits have different numbers of methods, which is inconsistent.

The test suite is strong, particularly for MtEph which includes both functional correctness tests and concurrent stress tests with barriers.
