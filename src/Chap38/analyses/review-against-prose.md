<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 38 — Parametric BSTs: Review Against Prose

- **Date**: 2026-02-17
- **Reviewer**: Claude-Opus-4.6
- **Prose source**: `prompts/Chap38.txt`
- **Source files**: `src/Chap38/BSTParaStEph.rs`, `src/Chap38/BSTParaMtEph.rs`
- **Test files**: `tests/Chap38/TestBSTParaStEph.rs`, `tests/Chap38/TestBSTParaMtEph.rs`
- **PTT files**: None

## Phase 2: Prose Inventory

| # | Prose Item | Reference | Type | StEph | MtEph | Notes |
|---|-----------|-----------|------|:-----:|:-----:|-------|
| 1 | Data Type 38.1 — K (key type with total order) | §1 | Type | `T: StT + Ord` | `T: MtKey` | Faithful — `Ord` provides total order |
| 2 | Data Type 38.1 — T (abstract tree type) | §1 | Type | `ParamBST<T>` | `ParamBST<T>` | Faithful — opaque via `Rc<RefCell>` / `Arc<RwLock>` |
| 3 | Data Type 38.1 — E (exposed tree: Leaf or Node) | §1 | Type | `Exposed<T>` | `Exposed<T>` | Faithful — `enum { Leaf, Node(ParamBST, T, ParamBST) }` |
| 4 | Data Type 38.1 — size : T → N | §1 | Fn | Yes | Yes | O(1) via stored `NodeInner.size` field |
| 5 | Data Type 38.1 — expose : T → E | §1 | Fn | Yes | Yes | `expose_internal` + trait `expose` |
| 6 | Data Type 38.1 — joinMid : E → T | §1 | Fn | Yes | Yes | `join_mid` free fn + trait method |
| 7 | Algorithm 38.2 — empty | §2 | Fn | Yes | Yes | `ParamBST::new()` = `joinMid(Leaf)` equivalent |
| 8 | Algorithm 38.2 — singleton k | §2 | Fn | No | No | No named `singleton` fn. Available indirectly via `ParamBSTLit![k]` macro in StEph only |
| 9 | Algorithm 38.2 — joinM(L, k, R) | §2 | Fn | Yes | Yes | `join_m` wrapper around `join_mid(Node(L, k, R))` |
| 10 | Algorithm 38.3 — split(T, k) | §2 | Fn | Yes | Yes | `split_inner` + trait `split`. Three-way compare with recursive rebuild |
| 11 | Algorithm 38.4 — minKey(T, k) | §2 | Fn | Yes | Yes | `min_key` — returns `Option<T>` instead of prose's two-argument form |
| 12 | Algorithm 38.4 — joinPair(T1, T2) | §2 | Fn | Yes | Yes | `join_pair_inner` — finds min of T2, splits, joins |
| 13 | Algorithm 38.5 — insert(T, k) | §2 | Fn | Yes | Yes | `split` then `joinM(L, k, R)` — matches prose |
| 14 | Algorithm 38.5 — delete(T, k) | §2 | Fn | Yes | Yes | `split` then `joinPair(L, R)` — matches prose |
| 15 | Algorithm 38.6 — union(T1, T2) (parallel) | §3 | Fn | Yes (seq) | Yes (par) | StEph sequential variant; MtEph uses `ParaPair!` |
| 16 | Algorithm 38.7 — intersect(T1, T2) (parallel) | §3 | Fn | **No** | Yes (par) | Missing from StEph entirely |
| 17 | Algorithm 38.8 — difference(T1, T2) (parallel) | §3 | Fn | **No** | Yes (par) | Missing from StEph entirely |
| 18 | Algorithm 38.9 — filter f T (parallel) | §3 | Fn | **No** | Yes (par) | Missing from StEph entirely |
| 19 | Algorithm 38.10 — reduce f I T (parallel) | §3 | Fn | **No** | Yes (par) | Missing from StEph entirely |
| 20 | Cost Specification 38.11 | §4 | Spec | Partial | Yes | Cost doc-comments present on all functions in both files |
| 21 | find (mentioned in Cost Spec) | §4 | Fn | Yes | Yes | Standard BST traversal, not given as a numbered algorithm in prose |
| 22 | in_order (not in prose) | — | Fn | Yes | Yes | Extra utility, not from the textbook |
| 23 | is_empty (not in prose) | — | Fn | Yes | Yes | Extra utility, not from the textbook |
| 24 | ParamBSTLit! macro (not in prose) | — | Macro | Yes | No | Convenience macro for StEph only |

### Inventory Summary

- **StEph**: 14 of 19 prose items implemented. Missing: `singleton`, `intersect`, `difference`, `filter`, `reduce`.
- **MtEph**: 18 of 19 prose items implemented. Missing: `singleton` only.
- Neither file contains `verus!` blocks — these are pure Rust implementations with no formal verification.
- Chap38 is gated with `not(verus_keep_ghost)` in `lib.rs`, meaning it is compiled only for cargo tests, never under Verus.

## Phase 3: Algorithmic Analysis

### Fidelity

| # | Algorithm | File | Fidelity | Detail |
|---|----------|------|----------|--------|
| 1 | expose | Both | **Faithful** | Reads root via borrow/read lock; returns `Leaf` or `Node(L, k, R)` — exact match to prose |
| 2 | joinMid | Both | **Faithful** | Creates `None` for Leaf or builds `NodeInner` with computed `size = 1 + left.size() + right.size()` — matches prose. Note: no rebalancing (this is the parametric version; rebalancing deferred to Ch39) |
| 3 | joinM | Both | **Faithful** | One-line wrapper `join_mid(Node(L, k, R))` — matches Algorithm 38.2 |
| 4 | split | Both | **Faithful** | Recursive three-way compare (`Less`/`Greater`/`Equal`), rebuilds with `join_mid` on the way back up — matches Algorithm 38.3 exactly |
| 5 | minKey | Both | **Minor deviation** | Prose takes two args `minKey(T, k)` where `k` is a fallback. Implementation uses `Option<T>` return instead. Functionally equivalent — when the tree is a leaf, `None` plays the same role as the fallback key at the call site |
| 6 | joinPair | Both | **Minor deviation** | Prose calls `minKey(L, k)` with the already-exposed left subtree and root key. Implementation calls `min_key(&right)` on the unexposed right tree, re-exposing it. Functionally identical but does one redundant expose |
| 7 | insert | Both | **Faithful** | `split(T, k)` then `joinM(L, k, R)` — matches Algorithm 38.5 verbatim |
| 8 | delete | Both | **Faithful** | `split(T, k)` then `joinPair(L, R)` — matches Algorithm 38.5 verbatim |
| 9 | union (StEph) | StEph | **Missing base case** | Prose checks `(Leaf, _) ⇒ T2` and `(_, Leaf) ⇒ T1`. StEph only checks the first: when `a` is `Node` but `b` is `Leaf`, the code recurses through all of `a`, doing O(\|a\|) work instead of O(1). Functionally correct but asymptotically wasteful |
| 10 | union (MtEph) | MtEph | **Missing base case** | Same issue as StEph: no check for `(_, Leaf) ⇒ T1`. The parallel recursion traverses all of `a` when `b` is empty |
| 11 | intersect | MtEph | **Faithful** | Both `(Leaf, _)` and `(_, Leaf)` base cases present. Parallel recursion with conditional `joinM`/`joinPair` based on `found` flag — matches Algorithm 38.7 |
| 12 | difference | MtEph | **Faithful** | Both base cases present: `(Leaf, _) ⇒ empty`, `(_, Leaf) ⇒ T1`. Inverted `joinM`/`joinPair` logic — matches Algorithm 38.8 |
| 13 | filter | MtEph | **Faithful** | Parallel left/right recursion, `joinM` when predicate holds, `joinPair` otherwise — matches Algorithm 38.9 |
| 14 | reduce | MtEph | **Faithful** | Parallel recursion producing `op(L', op(k, R'))` — matches Algorithm 38.10. Note: prose requires `f` to be associative and `I` to be its identity for correctness; this constraint is not documented in the code |
| 15 | find | Both | **Standard** | Not a numbered algorithm in Ch38 prose, but follows standard BST search. Correct |

### Cost Analysis

| # | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? | Notes |
|---|----------|-----------|-----------|-----------|-----------|--------|-------|
| 1 | new/empty | O(1) | O(1) | Θ(1) | Θ(1) | Yes | |
| 2 | expose | O(1) | O(1) | Θ(1) | Θ(1) | Yes | |
| 3 | joinMid | O(1) | O(1) | Θ(1) | Θ(1) | Yes | Parametric version; balancing cost deferred to Ch39 |
| 4 | size | O(1) | O(1) | Θ(1) | Θ(1) | Yes | Stored in node |
| 5 | split | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes | Depth-bounded by tree height |
| 6 | find | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes | |
| 7 | insert | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes | split + joinM |
| 8 | delete | O(lg \|t\|) | O(lg \|t\|) | Θ(log n) | Θ(log n) | Yes | split + joinPair |
| 9 | joinPair | O(lg(\|t1\|+\|t2\|)) | O(lg(\|t1\|+\|t2\|)) | Θ(log(\|t1\|+\|t2\|)) | Θ(log(\|t1\|+\|t2\|)) | Yes | minKey + split + joinM |
| 10 | union (MtEph) | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes | With the caveat that the missing `(_, Leaf)` base case causes O(\|a\|) work when b is empty |
| 11 | union (StEph) | O(m·lg(n/m)) | — | Θ(m·lg(n/m)) | Θ(m·lg(n/m)) | Yes | Sequential: span = work |
| 12 | intersect | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes | |
| 13 | difference | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes | |
| 14 | filter | O(\|t\|) | O(lg \|t\|) | Θ(n) | Θ(lg n) | Yes | |
| 15 | reduce | O(\|t\|) | O(lg \|t\|) | Θ(n) | Θ(lg n) | Yes | |
| 16 | in_order | O(\|t\|) | O(\|t\|) | Θ(n) | Θ(n) | Yes | Sequential traversal; not in prose |

All cost annotations agree with Cost Specification 38.11. The `Θ` notation used in the code comments is stronger than the `O` notation in the prose but consistent.

## Phase 4: Parallelism Review (Mt Modules)

### BSTParaMtEph.rs

| # | Function | Parallel? | Mechanism | Prose Match? | Notes |
|---|---------|:---------:|-----------|:------------:|-------|
| 1 | `union_inner` | Yes | `ParaPair!` on left/right recursive union | Yes | Matches `(union(L1, L2) \|\| union(R1, R2))` from Algorithm 38.6 |
| 2 | `intersect_inner` | Yes | `ParaPair!` on left/right recursive intersect | Yes | Matches `(intersect(L1, L2) \|\| intersect(R1, R2))` from Algorithm 38.7 |
| 3 | `difference_inner` | Yes | `ParaPair!` on left/right recursive difference | Yes | Matches `(difference(L1, L2) \|\| difference(R1, R2))` from Algorithm 38.8 |
| 4 | `filter_inner` | Yes | `ParaPair!` on left/right recursive filter | Yes | Matches `(filter f L) \|\| (filter f R)` from Algorithm 38.9. Uses `Arc<F>` to share predicate across threads |
| 5 | `reduce_inner` | Yes | `ParaPair!` on left/right recursive reduce | Yes | Matches `(reduce f I L) \|\| (reduce f I R)` from Algorithm 38.10. Uses `Arc<F>` for the operator |
| 6 | `collect_in_order` | No | Sequential traversal | N/A | Not in prose; sequential in_order is appropriate |
| 7 | All other functions | No | Sequential | N/A | expose, joinMid, split, insert, delete, find are inherently sequential per-tree operations |

### Thread Safety Mechanism

- `ParamBST<T>` uses `Arc<RwLock<Option<Box<NodeInner>>>>` for thread-safe interior mutability.
- `insert` and `delete` use `write()` locks to mutate in place.
- `expose`, `find`, `split` use `read()` locks.
- `ParaPair!` macro handles fork-join threading.
- All closures use `move` captures as required by `ParaPair!`.

### Parallel Correctness Concerns

1. **No grain control**: Every recursive call spawns threads regardless of tree size. For small subtrees, the threading overhead dominates. The prose abstracts over this, but a practical implementation should have a sequential cutoff.
2. **`Arc<F>` sharing for filter/reduce**: The predicate and operator closures are shared across threads via `Arc`. This is correct but adds reference-counting overhead at each recursive level.
3. **`insert` and `delete` are NOT concurrent-safe in their current form**: They do `read` + compute + `write` without holding the lock across the entire operation. Concurrent inserts can produce lost updates. The concurrent tests use loose assertions (e.g., `assert!(results[0] >= 1)`) to accommodate this.

## Phase 5: RTT Review

### TestBSTParaStEph.rs (5 tests)

| # | Test | Operations Covered | Edge Cases? | Correct? |
|---|------|--------------------|:-----------:|:--------:|
| 1 | `test_parambstlit_macro_functionality` | `ParamBSTLit![]`, `ParamBSTLit![2,1,3]`, size, find | Empty tree | Yes |
| 2 | `para_basic_insert_find` | insert, find, size, is_empty, in_order | 7-element tree | Yes |
| 3 | `para_split_and_join_pair` | split, join_pair, in_order | Split at existing key | Yes |
| 4 | `para_union_and_delete` | union, delete, find, in_order | Union of disjoint sets | Yes |
| 5 | `para_join_mid_expose_roundtrip` | expose, join_mid, size | Leaf and single-node | Yes |

### TestBSTParaMtEph.rs (16 tests)

| # | Test | Operations Covered | Edge Cases? | Correct? |
|---|------|--------------------|:-----------:|:--------:|
| 1 | `para_basic_insert_find` | insert, find, size, is_empty, in_order | 7-element tree | Yes |
| 2 | `para_split_and_join_pair` | split, join_pair, in_order | Split at existing key | Yes |
| 3 | `para_union_and_delete` | union, delete, find, in_order | Union of disjoint sets | Yes |
| 4 | `para_join_mid_expose_roundtrip` | expose, join_mid, size | Leaf and single-node | Yes |
| 5 | `para_intersect_and_difference` | intersect, difference, in_order | Overlapping sets | Yes |
| 6 | `para_filter_and_reduce` | filter, reduce | Even filter, sum, empty reduce | Yes |
| 7 | `para_union_large_balanced` | union on 200/200 element trees | Overlapping ranges | Yes |
| 8 | `para_intersect_and_difference_large` | intersect, difference on 256-element trees | Large overlapping ranges | Yes |
| 9 | `para_filter_and_reduce_edge_cases` | filter, reduce with edge cases | Single element, sum of squares | Yes |
| 10 | `para_concurrent_insertions` | Concurrent insert + find across 4 threads | Thread safety under contention | Weak — uses `assert!(size >= 1)` |
| 11 | `para_concurrent_operations_stress` | 6-thread concurrent insert + find | Stress test | Weak — loose assertions |
| 12 | `para_concurrent_set_operations` | Concurrent union, intersect, difference | Parallel set ops on read-only trees | Yes — exact assertions |
| 13 | `para_concurrent_filter_reduce` | Concurrent filter, reduce | Parallel read-only operations | Yes — exact assertions |
| 14 | `para_concurrent_split_join` | Concurrent split operations | Parallel read-only splits | Yes — exact assertions |
| 15 | `para_concurrent_expose_join_mid` | Concurrent expose, join_mid | Building trees in parallel | Yes |
| 16 | `para_concurrent_delete_operations` | Concurrent delete + find | Concurrent mutation | Weak — `assert!(size <= 100)` |

### RTT Coverage Assessment

- **StEph coverage**: Good for implemented operations. Missing tests for `intersect`, `difference`, `filter`, `reduce` (because these are not implemented in StEph).
- **MtEph coverage**: Excellent functional coverage. All 16 trait methods exercised. Large-tree tests provide confidence in algorithmic correctness.
- **Concurrency tests**: Tests 10, 11, 16 use concurrent mutation (insert/delete on a shared tree) with very loose assertions. This correctly acknowledges that the `insert`/`delete` pattern is not linearizable under concurrent mutation. Tests 12–15 use concurrent reads on shared immutable trees and have exact assertions — these are properly testing the parallel algorithms.
- **Missing**: No test for split at a non-existent key, no test for union with empty tree, no test for delete of non-existent key.

## Phase 6: PTT Review

No proof-time tests exist for Chapter 38. This is expected since:
1. Neither source file contains `verus!` blocks.
2. The module is gated with `not(verus_keep_ghost)` in `lib.rs`, meaning Verus never compiles it.
3. There are no specs to test.

## Phase 7: Gap Analysis

| # | Priority | Category | Gap | Detail | Recommendation |
|---|----------|----------|-----|--------|----------------|
| 1 | **High** | Fidelity | Union missing `(_, Leaf) ⇒ T1` base case | Both `union_inner` (StEph and MtEph) only check `expose(a)` for Leaf. When `a` is non-empty but `b` is empty, the code traverses all of `a` at O(\|a\|) cost instead of returning immediately at O(1). Functionally correct but asymptotically wasteful. | Add `match expose_internal(b) { Exposed::Leaf => return a.clone(), _ => {} }` before the main recursion, or restructure to match the prose's `case (expose T1, expose T2) of` pattern |
| 2 | **High** | Completeness | StEph missing 4 algorithms | `intersect`, `difference`, `filter`, `reduce` are not implemented in `BSTParaStEph.rs`. The `ParamBSTTrait` in StEph has only 12 methods vs MtEph's 16. | Add sequential implementations. These are trivially derived from the MtEph versions by removing `ParaPair!` and making sequential calls |
| 3 | **High** | Verification | No `verus!` blocks | No formal specifications or proofs. The chapter is entirely unverified. Gated with `not(verus_keep_ghost)` in `lib.rs`. | Future verusification work: add View, spec functions, requires/ensures, and proof functions. This is a large effort given the recursive tree structure |
| 4 | **Medium** | Fidelity | `singleton` not a named function | Algorithm 38.2 defines `singleton k = joinMid(Node(empty, k, empty))`. No named function exists. `ParamBSTLit![k]` serves this role for StEph but works via insert (different implementation). MtEph has no equivalent. | Add `fn singleton(k: T) -> Self` to both traits |
| 5 | **Medium** | Trait parity | StEph and MtEph traits differ | StEph trait has 12 methods, MtEph has 16. Different method sets break the abstraction that both implement the same ADT. | Unify: add `intersect`, `difference`, `filter`, `reduce` to StEph trait with sequential implementations |
| 6 | **Medium** | Documentation | `reduce` associativity requirement undocumented | Algorithm 38.10 requires `f` to be associative with identity `I` for correctness. The parallel evaluation order differs from left-to-right sequential, so non-associative operators produce wrong results. This constraint is not documented. | Add a doc comment on `reduce` noting the associativity requirement |
| 7 | **Low** | Testing | Concurrent mutation tests have weak assertions | Tests 10, 11, 16 in MtEph use assertions like `assert!(size >= 1)` that are tautologically true. They test that the code doesn't crash/deadlock but don't verify correctness. | This is acceptable given that `insert`/`delete` are not designed for concurrent mutation. Consider adding a doc comment noting thread-safety limitations |
| 8 | **Low** | Testing | Missing edge case tests | No tests for: split at non-existent key, union/intersect/difference with empty tree, delete of non-existent key, reduce with single element | Add targeted edge case tests |
| 9 | **Low** | Code quality | `ParamBSTLit!` macro missing from MtEph | StEph has the macro; MtEph does not. | Either add an equivalent macro for MtEph or note that this is intentional |
| 10 | **Low** | Fidelity | `minKey` signature deviation | Prose `minKey(T, k)` takes fallback key as argument; implementation returns `Option<T>`. Functionally equivalent via `unwrap_or` at call site. | No change needed; deviation is minor and idiomatic in Rust |

## Phase 8: TOC Review

Neither file contains `verus!` blocks, so the standard 13-section TOC structure does not directly apply. Both files are pure Rust modules.

### File Structure

| # | File | Structure (top to bottom) |
|---|------|---------------------------|
| 1 | `BSTParaStEph.rs` | copyright → module → imports → enum Exposed → struct NodeInner → struct ParamBST → trait ParamBSTTrait → free fns (expose_internal, join_mid, split_inner, join_m, min_key, join_pair_inner, union_inner, collect_in_order) → impl ParamBSTTrait for ParamBST → macro_rules! ParamBSTLit |
| 2 | `BSTParaMtEph.rs` | copyright → module → imports → enum Exposed → struct NodeInner → struct ParamBST → trait ParamBSTTrait → free fns (expose_internal, join_mid, split_inner, join_m, min_key, join_pair_inner, union_inner, intersect_inner, difference_inner, filter_inner, filter_parallel, reduce_inner, reduce_parallel, collect_in_order) → impl ParamBSTTrait for ParamBST |

### In/Out Table

Since neither file has `verus!` blocks, the in/out table is trivially "all outside":

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:---:|:------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BSTParaStEph.rs | `✅ out` (derive) | - | `✅ out` (derive on Exposed) | - | - | `✅ out` (derive) | - | `✅ out` (ParamBSTLit!) | No `verus!` block exists |
| 2 | BSTParaMtEph.rs | `✅ out` (derive) | - | - | - | - | `✅ out` (derive) | - | - | No `verus!` block exists |

Note: "✅ out" here means "correctly outside `verus!`" in the trivial sense — there is no `verus!` block to be inside of. When the chapter is verusified, these derive impls will need to be reviewed per the standard rules (Clone, PartialEq/Eq inside; Debug outside; etc.).

## Proof Holes Summary

```
✓ BSTParaMtEph.rs
✓ BSTParaStEph.rs

Modules:     2 clean, 0 holed
Holes Found: 0 total
```

No proof holes. This is trivially true since neither file contains `verus!` blocks — there are no proofs to have holes in.

## Review TODOs

| # | Priority | TODO | Effort |
|---|----------|------|--------|
| 1 | High | Fix union `(_, Leaf)` missing base case in both StEph and MtEph | Small — add 3 lines per file |
| 2 | High | Add `intersect`, `difference`, `filter`, `reduce` to StEph (sequential variants) | Medium — straightforward port from MtEph |
| 3 | High | Unify `ParamBSTTrait` across StEph and MtEph to have the same method set | Medium — requires TODO #2 first |
| 4 | Medium | Add `singleton` as a named function to both traits | Small |
| 5 | Medium | Document associativity requirement on `reduce` | Small — doc comment |
| 6 | Low | Add edge case tests (split non-existent key, ops with empty tree, delete non-existent) | Small |
| 7 | Low | Add `ParamBSTLit!` equivalent for MtEph or document why it's absent | Small |
| 8 | Future | Verusify Chapter 38 — add `verus!` blocks, View impls, specs, proofs | Large — recursive tree structure requires careful spec design |
