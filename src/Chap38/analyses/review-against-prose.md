<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 38 — Parametric BSTs: Review Against Prose

**Date:** 2026-02-28
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap38.txt`
**Source files:** `src/Chap38/BSTParaStEph.rs`, `src/Chap38/BSTParaMtEph.rs`
**Test files:** `tests/Chap38/TestBSTParaStEph.rs` (13 tests), `tests/Chap38/TestBSTParaMtEph.rs` (20 tests)
**PTT files:** None
**Verification status:** Both files compile with 0 errors. Both have trait+impl inside `verus!` with specs on 14 of 17 trait methods; all impl methods `external_body`. 51 total external_body holes (32 StEph, 19 MtEph).

## Phase 1: Inventory

| # | Chap | Module | Tr | IT | IBI | ML | V! | -V! | Holes | NoSpec |
|---|------|--------|:--:|:--:|:---:|:--:|:--:|:---:|:-----:|:------:|
| 1 | 38 | BSTParaStEph.rs | 17 | 17 | 0 | 14 | 30 | 0 | 30 eb | 0 |
| 2 | 38 | BSTParaMtEph.rs | 17 | 17 | 0 | 16 | 18 | 14 | 18 eb | 14 |

Column key: Tr = trait fns, IT = impl-trait fns, IBI = bare-impl fns, ML = module-level free fns, V! = exec fns inside verus!, -V! = exec fns outside verus!, Holes = external_body count (exec fns only), NoSpec = exec fns without requires/ensures.

**veracity-review-proof-holes totals** (includes spec fns and clone): StEph 32 eb + 2 accept + 1 RwLock info; MtEph 19 eb + 2 accept + 1 RwLock info. Grand total: 51 external_body holes.

**Summary:** BSTParaStEph.rs has all functions inside verus! (all external_body). BSTParaMtEph.rs has trait+impl inside verus! (all external_body) but 14 free functions outside verus! entirely. Both files share identical trait specs (14 of 17 methods with ensures).

## Phase 2: Prose Inventory

### Definitions

| # | Chap | Prose Item | Reference | Type | Description |
|---|------|-----------|-----------|------|-------------|
| 1 | 38 | Data Type 38.1 — K (key type) | §1 | Type | Key type supporting `<` |
| 2 | 38 | Data Type 38.1 — T (tree type) | §1 | Type | Opaque tree type |
| 3 | 38 | Data Type 38.1 — E (Exposed) | §1 | Type | `Leaf` or `Node(T × K × T)` |
| 4 | 38 | Data Type 38.1 — size | §1 | Fn | Number of keys in T |
| 5 | 38 | Data Type 38.1 — expose | §1 | Fn | Expose the root |
| 6 | 38 | Data Type 38.1 — joinMid | §1 | Fn | Inverse of expose; rebalances |

### Algorithms

| # | Chap | Prose Item | Reference | Description |
|---|------|-----------|-----------|-------------|
| 7 | 38 | Algorithm 38.2 — empty | §2 | `joinMid(Leaf)` |
| 8 | 38 | Algorithm 38.2 — singleton k | §2 | `joinMid(Node(empty, k, empty))` |
| 9 | 38 | Algorithm 38.2 — joinM(L, k, R) | §2 | `joinMid(Node(L, k, R))` |
| 10 | 38 | Algorithm 38.3 — split(T, k) | §2 | Three-way recursive split |
| 11 | 38 | Algorithm 38.4 — minKey(T, k) | §2 | Find minimum key |
| 12 | 38 | Algorithm 38.4 — joinPair(T1, T2) | §2 | Join two trees w/o middle key |
| 13 | 38 | Algorithm 38.5 — insert(T, k) | §2 | `split` then `joinM(L, k, R)` |
| 14 | 38 | Algorithm 38.5 — delete(T, k) | §2 | `split` then `joinPair(L, R)` |
| 15 | 38 | Algorithm 38.6 — union(T1, T2) | §3 | Parallel divide-and-conquer |
| 16 | 38 | Algorithm 38.7 — intersect(T1, T2) | §3 | Parallel, include if found |
| 17 | 38 | Algorithm 38.8 — difference(T1, T2) | §3 | Parallel, exclude if found |
| 18 | 38 | Algorithm 38.9 — filter f T | §3 | Parallel, joinM/joinPair |
| 19 | 38 | Algorithm 38.10 — reduce f I T | §3 | Parallel, `f(L', f(k, R'))` |

### Cost Specifications

| # | Chap | Prose Item | Reference | Description |
|---|------|-----------|-----------|-------------|
| 20 | 38 | Cost Specification 38.11 | §4 | Full cost table for all BST ops |

### Exercises

| # | Chap | Prose Item | Reference | Description |
|---|------|-----------|-----------|-------------|
| 21 | 38 | Exercise 38.1 | §3 | Prove correct intersection, difference, union |

### Implementation Status

| # | Chap | Prose Item | BSTParaStEph.rs | BSTParaMtEph.rs | Notes |
|---|------|-----------|:---------------:|:---------------:|-------|
| 1 | 38 | K (key type) | `T: StT + Ord` | `T: MtKey` | Faithful |
| 2 | 38 | T (tree type) | `ParamBST<T>` | `ParamBST<T>` | Arc/RwLock |
| 3 | 38 | E (Exposed) | `Exposed<T>` | `Exposed<T>` | Faithful |
| 4 | 38 | size | Yes | Yes | O(1) via stored field |
| 5 | 38 | expose | Yes | Yes | `expose_internal` + trait |
| 6 | 38 | joinMid | Yes | Yes | `join_mid` free fn + trait |
| 7 | 38 | empty | Yes | Yes | `ParamBST::new()` |
| 8 | 38 | singleton | Yes | Yes | via joinMid |
| 9 | 38 | joinM | Yes | Yes | `join_m` wrapper |
| 10 | 38 | split | Yes | Yes | `split_inner` — exact match |
| 11 | 38 | minKey | Yes | Yes | Returns `Option<T>` |
| 12 | 38 | joinPair | Yes | Yes | `join_pair_inner` |
| 13 | 38 | insert | Yes | Yes | `split` + `joinM` |
| 14 | 38 | delete | Yes | Yes | `split` + `joinPair` |
| 15 | 38 | union | Yes (seq) | Yes (par) | MtEph uses `ParaPair!` |
| 16 | 38 | intersect | Yes (seq) | Yes (par) | Faithful |
| 17 | 38 | difference | Yes (seq) | Yes (par) | Faithful |
| 18 | 38 | filter | Yes (seq) | Yes (par) | Faithful |
| 19 | 38 | reduce | Yes (seq) | Yes (par) | Faithful |
| 20 | 38 | Cost Spec 38.11 | Yes | Yes | Doc-comments on trait |
| 21 | 38 | find | Yes | Yes | Not numbered in Ch38 |
| 22 | 38 | in_order | Yes | Yes | Extra utility |
| 23 | 38 | is_empty | Yes | Yes | Extra utility |
| 24 | 38 | ParamBSTLit! | Yes | No | Convenience macro |

**BSTParaStEph.rs:** 19 of 19 prose items implemented.
**BSTParaMtEph.rs:** 19 of 19 prose items implemented.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Chap | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? |
|---|------|----------|-----------|-----------|-----------|-----------|:------:|
| 1 | 38 | new/empty | O(1) | O(1) | Theta(1) | Theta(1) | Yes |
| 2 | 38 | expose | O(1) | O(1) | Theta(1) | Theta(1) | Yes |
| 3 | 38 | joinMid | O(1) | O(1) | Theta(1) | Theta(1) | Yes |
| 4 | 38 | size | O(1) | O(1) | Theta(1) | Theta(1) | Yes |
| 5 | 38 | split | O(lg n) | O(lg n) | Theta(lg n) | Theta(lg n) | Yes |
| 6 | 38 | find | O(lg n) | O(lg n) | Theta(lg n) | Theta(lg n) | Yes |
| 7 | 38 | insert | O(lg n) | O(lg n) | Theta(lg n) | Theta(lg n) | Yes |
| 8 | 38 | delete | O(lg n) | O(lg n) | Theta(lg n) | Theta(lg n) | Yes |
| 9 | 38 | joinPair | O(lg(n1+n2)) | O(lg(n1+n2)) | Theta(lg(n1+n2)) | Theta(lg(n1+n2)) | Yes |
| 10 | 38 | union (Mt) | O(m lg(n/m)) | O(lg n) | Theta(m lg(n/m)) | Theta(lg n) | Yes |
| 11 | 38 | union (St) | O(m lg(n/m)) | — | Theta(m lg(n/m)) | seq | Yes |
| 12 | 38 | intersect (Mt) | O(m lg(n/m)) | O(lg n) | Theta(m lg(n/m)) | Theta(lg n) | Yes |
| 13 | 38 | intersect (St) | O(m lg(n/m)) | — | Theta(m lg(n/m)) | seq | Yes |
| 14 | 38 | difference (Mt) | O(m lg(n/m)) | O(lg n) | Theta(m lg(n/m)) | Theta(lg n) | Yes |
| 15 | 38 | difference (St) | O(m lg(n/m)) | — | Theta(m lg(n/m)) | seq | Yes |
| 16 | 38 | filter (Mt) | O(n) | O(lg n) | Theta(n) | Theta(lg n) | Yes |
| 17 | 38 | filter (St) | O(n) | — | Theta(n) | seq | Yes |
| 18 | 38 | reduce (Mt) | O(n) | O(lg n) | Theta(n) | Theta(lg n) | Yes |
| 19 | 38 | reduce (St) | O(n) | — | Theta(n) | seq | Yes |
| 20 | 38 | in_order | O(n) | O(n) | Theta(n) | Theta(n) | Yes |

### 3b. Implementation Fidelity

| # | Chap | Algorithm | File | Fidelity | Detail |
|---|------|----------|------|:--------:|--------|
| 1 | 38 | expose | Both | Faithful | Reads root via read lock |
| 2 | 38 | joinMid | Both | Faithful | Creates NodeInner, computes size |
| 3 | 38 | joinM | Both | Faithful | One-line wrapper on joinMid |
| 4 | 38 | split | Both | Faithful | Recursive three-way compare |
| 5 | 38 | minKey | Both | Minor | Returns `Option<T>` vs prose `(T, k)` |
| 6 | 38 | joinPair | Both | Minor | Calls `min_key(&right)` then split |
| 7 | 38 | insert | Both | Faithful | `split` + `joinM` |
| 8 | 38 | delete | Both | Faithful | `split` + `joinPair` |
| 9 | 38 | union | Both | Faithful | Both base cases present |
| 10 | 38 | intersect | Both | Faithful | Conditional joinM/joinPair |
| 11 | 38 | difference | Both | Faithful | Inverted join logic |
| 12 | 38 | filter | Both | Faithful | Predicate-based join/joinPair |
| 13 | 38 | reduce | Both | Faithful | `op(L', op(k, R'))` |
| 14 | 38 | find | Both | Standard | Not numbered in Ch38 |

### 3c. Spec Fidelity

Both files now share identical trait specs. 14 of 17 trait methods have `ensures`:

| # | Chap | Function | Spec | Strength | Detail |
|---|------|---------|------|:--------:|--------|
| 1 | 38 | new | `result@ == Set::empty()` | strong | |
| 2 | 38 | singleton | `result@ == Set::empty().insert(key@)` | strong | Includes finite |
| 3 | 38 | expose | `self@.len() == 0 ==> exposed is Leaf` | partial | Missing: Node variant spec |
| 4 | 38 | join_mid | `exposed is Leaf ==> result@ empty` | partial | Missing: Node case result set |
| 5 | 38 | size | `count == self@.len()` | strong | Includes finite |
| 6 | 38 | is_empty | `empty == (self@.len() == 0)` | strong | Includes finite |
| 7 | 38 | insert | none | none | Interior mutability blocks old() |
| 8 | 38 | delete | none | none | Interior mutability blocks old() |
| 9 | 38 | find | `found.is_some() <==> self@.contains(key@)` | partial | Missing: found == Some(key) |
| 10 | 38 | split | `parts.1 == self@.contains(key@)`, finite | partial | Missing: L < k, R > k |
| 11 | 38 | join_pair | `joined@.finite()` | weak | Missing: result = L union R |
| 12 | 38 | union | `result@ == self@.union(other@)` | strong | |
| 13 | 38 | intersect | `result@ == self@.intersect(other@)` | strong | |
| 14 | 38 | difference | `result@ == self@.difference(other@)` | strong | |
| 15 | 38 | filter | `filtered@.subset_of(self@)` | partial | Missing: exact predicate spec |
| 16 | 38 | reduce | none | none | Needs op spec (higher-order) |
| 17 | 38 | in_order | `seq@.len() == self@.len()` | partial | Missing: sorted + same elements |

**Critical note:** `spec_set_view` is `external_body` returning `Set::empty()` in both files. This is a placeholder — the View does not track actual tree contents. All `ensures` clauses are vacuously trusted via `external_body` on every impl method.

## Phase 4: Parallelism Review

### BSTParaMtEph.rs

| # | Chap | Function | Parallel? | Mechanism | Prose Match? |
|---|------|---------|:---------:|-----------|:------------:|
| 1 | 38 | `union_inner` | Yes | `ParaPair!` | Yes |
| 2 | 38 | `intersect_inner` | Yes | `ParaPair!` | Yes |
| 3 | 38 | `difference_inner` | Yes | `ParaPair!` | Yes |
| 4 | 38 | `filter_inner` | Yes | `ParaPair!` + `Arc<F>` | Yes |
| 5 | 38 | `reduce_inner` | Yes | `ParaPair!` + `Arc<F>` | Yes |
| 6 | 38 | `collect_in_order` | No | Sequential traversal | N/A |
| 7 | 38 | All others | No | Sequential per-tree ops | N/A |

### Thread Safety

- `ParamBST<T>` uses `Arc<RwLock<Option<Box<NodeInner>>>>`.
- `insert`/`delete` use `write()` locks; `expose`/`find`/`split` use `read()` locks.
- `ParaPair!` macro handles fork-join threading.
- `insert`/`delete` are NOT linearizable: read-compute-write without holding lock across full operation.

## Phase 5: Runtime Test Review

### TestBSTParaStEph.rs (13 tests)

| # | Chap | Test | Operations Covered |
|---|------|----|-----|
| 1 | 38 | `test_parambstlit_macro_functionality` | ParamBSTLit!, size, find |
| 2 | 38 | `para_basic_insert_find` | insert, find, size, is_empty, in_order |
| 3 | 38 | `para_split_and_join_pair` | split, join_pair, in_order |
| 4 | 38 | `para_union_and_delete` | union, delete, find, in_order |
| 5 | 38 | `para_join_mid_expose_roundtrip` | expose, join_mid, size |
| 6 | 38 | `para_singleton` | singleton, size, find, in_order |
| 7 | 38 | `para_intersect_and_difference` | intersect, difference, in_order |
| 8 | 38 | `para_filter_and_reduce` | filter, reduce |
| 9 | 38 | `para_intersect_and_difference_large` | 256-element intersect, difference |
| 10 | 38 | `para_filter_and_reduce_edge_cases` | filter, reduce edge cases |
| 11 | 38 | `para_split_nonexistent_key` | split at missing key |
| 12 | 38 | `para_ops_on_empty_tree` | empty tree operations |
| 13 | 38 | `para_delete_nonexistent_key` | delete non-existent key |

### TestBSTParaMtEph.rs (20 tests)

| # | Chap | Test | Operations Covered |
|---|------|----|-----|
| 1 | 38 | `para_basic_insert_find` | insert, find, size, is_empty, in_order |
| 2 | 38 | `para_split_and_join_pair` | split, join_pair, in_order |
| 3 | 38 | `para_union_and_delete` | union, delete, find, in_order |
| 4 | 38 | `para_join_mid_expose_roundtrip` | expose, join_mid, size |
| 5 | 38 | `para_intersect_and_difference` | intersect, difference, in_order |
| 6 | 38 | `para_filter_and_reduce` | filter, reduce |
| 7 | 38 | `para_union_large_balanced` | 200+200 element union |
| 8 | 38 | `para_intersect_and_difference_large` | 256-element intersect, difference |
| 9 | 38 | `para_filter_and_reduce_edge_cases` | filter, reduce edge cases |
| 10 | 38 | `para_singleton` | singleton, size, find, in_order |
| 11 | 38 | `para_split_nonexistent_key` | split at missing key |
| 12 | 38 | `para_ops_on_empty_tree` | empty tree operations |
| 13 | 38 | `para_delete_nonexistent_key` | delete non-existent key |
| 14 | 38 | `para_concurrent_insertions` | 4-thread concurrent insert+find |
| 15 | 38 | `para_concurrent_operations_stress` | 6-thread stress test |
| 16 | 38 | `para_concurrent_set_operations` | Concurrent union/intersect/diff |
| 17 | 38 | `para_concurrent_filter_reduce` | Concurrent filter/reduce |
| 18 | 38 | `para_concurrent_split_join` | Concurrent split at 25/50/75 |
| 19 | 38 | `para_concurrent_expose_join_mid` | Concurrent tree construction |
| 20 | 38 | `para_concurrent_delete_operations` | Concurrent delete+find |

All 33 tests pass.

## Phase 6: PTT Review

No PTTs exist. Neither file contains verified loops or iterators inside `verus!`. No PTTs needed.

## Phase 7: Gap Analysis

### Proof Holes

| # | Chap | File | Line | Type | Function | Removable? |
|---|------|------|-----:|------|----------|:----------:|
| 1 | 38 | BSTParaStEph.rs | 72 | info (RwLock) | `new_bst_para_lock` | No — vstd req |
| 2 | 38 | BSTParaStEph.rs | 80 | eb | `spec_set_view` | Blocker — needs recursive View |
| 3 | 38 | BSTParaStEph.rs | 165 | eb | `new` | Yes — straightforward |
| 4 | 38 | BSTParaStEph.rs | 170 | eb | `singleton` | Yes — straightforward |
| 5 | 38 | BSTParaStEph.rs | 179 | eb | `expose` | Yes — needs Exposed View |
| 6 | 38 | BSTParaStEph.rs | 184 | eb | `join_mid` | Yes — needs Exposed View |
| 7 | 38 | BSTParaStEph.rs | 189 | eb | `size` | Yes — needs spec_set_view fixed |
| 8 | 38 | BSTParaStEph.rs | 199 | eb | `is_empty` | Yes — needs spec_set_view fixed |
| 9 | 38 | BSTParaStEph.rs | 204 | eb | `insert` | Medium — mutation + RwLock |
| 10 | 38 | BSTParaStEph.rs | 215 | eb | `delete` | Medium — mutation + RwLock |
| 11 | 38 | BSTParaStEph.rs | 226 | eb | `find` | Yes — recursive read-only |
| 12 | 38 | BSTParaStEph.rs | 240 | eb | `split` | Yes — recursive read-only |
| 13 | 38 | BSTParaStEph.rs | 248 | eb | `join_pair` | Yes — delegates to split+joinM |
| 14 | 38 | BSTParaStEph.rs | 253 | eb | `union` | Medium — recursive + set spec |
| 15 | 38 | BSTParaStEph.rs | 258 | eb | `intersect` | Medium — recursive + set spec |
| 16 | 38 | BSTParaStEph.rs | 263 | eb | `difference` | Medium — recursive + set spec |
| 17 | 38 | BSTParaStEph.rs | 268 | eb | `filter` | Hard — higher-order fn spec |
| 18 | 38 | BSTParaStEph.rs | 275 | eb | `reduce` | Hard — needs associativity |
| 19 | 38 | BSTParaStEph.rs | 280 | eb | `in_order` | Medium — needs Seq view |
| 20 | 38 | BSTParaStEph.rs | 292 | eb | `new_leaf` | Yes — trivial |
| 21 | 38 | BSTParaStEph.rs | 297 | eb | `expose_internal` | Yes — read lock + match |
| 22 | 38 | BSTParaStEph.rs | 308 | eb | `join_mid` (free) | Yes — match + construct |
| 23 | 38 | BSTParaStEph.rs | 321 | eb | `split_inner` | Yes — recursive, read-only |
| 24 | 38 | BSTParaStEph.rs | 341 | eb | `join_m` | Yes — one-line wrapper |
| 25 | 38 | BSTParaStEph.rs | 346 | eb | `min_key` | Yes — recursive read-only |
| 26 | 38 | BSTParaStEph.rs | 357 | eb | `join_pair_inner` | Yes — delegates to split+joinM |
| 27 | 38 | BSTParaStEph.rs | 370 | eb | `union_inner` | Medium — recursive set proof |
| 28 | 38 | BSTParaStEph.rs | 385 | eb | `intersect_inner` | Medium — recursive set proof |
| 29 | 38 | BSTParaStEph.rs | 403 | eb | `difference_inner` | Medium — recursive set proof |
| 30 | 38 | BSTParaStEph.rs | 422 | eb | `filter_inner` | Hard — higher-order fn spec |
| 31 | 38 | BSTParaStEph.rs | 442 | eb | `reduce_inner` | Hard — needs associativity |
| 32 | 38 | BSTParaStEph.rs | 459 | eb | `collect_in_order` | Medium — needs Seq view |
| 33 | 38 | BSTParaStEph.rs | 502 | eb | `clone` (ParamBST) | No — Arc::clone |
| 34 | 38 | BSTParaStEph.rs | 481 | accept | Clone Exposed | Removable with real View |
| 35 | 38 | BSTParaStEph.rs | 496 | accept | Clone NodeInner | Removable with real View |
| 36 | 38 | BSTParaMtEph.rs | 66 | info (RwLock) | `new_bst_para_lock` | No — vstd req |
| 37 | 38 | BSTParaMtEph.rs | 74 | eb | `spec_set_view` | Blocker — same as StEph |
| 38 | 38 | BSTParaMtEph.rs | 159-274 | eb ×17 | all 17 impl methods | Blocked by free fns outside V! |
| 39 | 38 | BSTParaMtEph.rs | 315 | eb | `clone` (ParamBST) | No — Arc::clone |
| 40 | 38 | BSTParaMtEph.rs | 294 | accept | Clone Exposed | Removable with real View |
| 41 | 38 | BSTParaMtEph.rs | 309 | accept | Clone NodeInner | Removable with real View |

### MtEph Free Functions Outside verus!

These 14 free functions live outside `verus!` and have no specs or proof obligations. They are invisible to the verifier:

| # | Chap | File | Function | Lines | Notes |
|---|------|------|----------|------:|-------|
| 1 | 38 | BSTParaMtEph.rs | `new_leaf` | 325 | Trivial |
| 2 | 38 | BSTParaMtEph.rs | `expose_internal` | 329 | Read lock |
| 3 | 38 | BSTParaMtEph.rs | `join_mid` | 339 | Construct |
| 4 | 38 | BSTParaMtEph.rs | `split_inner` | 351 | Recursive |
| 5 | 38 | BSTParaMtEph.rs | `join_m` | 370 | Wrapper |
| 6 | 38 | BSTParaMtEph.rs | `min_key` | 374 | Recursive |
| 7 | 38 | BSTParaMtEph.rs | `join_pair_inner` | 384 | Uses min_key+split |
| 8 | 38 | BSTParaMtEph.rs | `union_inner` | 395 | ParaPair! parallel |
| 9 | 38 | BSTParaMtEph.rs | `intersect_inner` | 408 | ParaPair! parallel |
| 10 | 38 | BSTParaMtEph.rs | `difference_inner` | 424 | ParaPair! parallel |
| 11 | 38 | BSTParaMtEph.rs | `filter_inner` | 442 | ParaPair! + Arc |
| 12 | 38 | BSTParaMtEph.rs | `filter_parallel` | 464 | Arc wrapper |
| 13 | 38 | BSTParaMtEph.rs | `reduce_inner` | 472 | ParaPair! + Arc |
| 14 | 38 | BSTParaMtEph.rs | `reduce_parallel` | 495 | Arc wrapper |

### Missing Specs

| # | Chap | File | Function | Spec Needed |
|---|------|------|----------|-------------|
| 1 | 38 | Both | insert | ensures self@ == old(self@).insert(key@) |
| 2 | 38 | Both | delete | ensures self@ == old(self@).remove(key@) |
| 3 | 38 | Both | reduce | ensures on reduction value |

## Phase 8: TOC Review

### In/Out Table

| # | Chap | Item | BSTParaStEph.rs | BSTParaMtEph.rs |
|---|------|------|:---------------:|:---------------:|
| 1 | 38 | Type definitions | ✅ in | ✅ in |
| 2 | 38 | View impls | ✅ in | ✅ in |
| 3 | 38 | RwLockPredicate | ✅ in | ✅ in |
| 4 | 38 | Trait | ✅ in | ✅ in |
| 5 | 38 | Impl | ✅ in (eb) | ✅ in (eb) |
| 6 | 38 | Free fns | ✅ in (eb) | ❌ out |
| 7 | 38 | Clone (Exposed) | ✅ in (accept) | ✅ in (accept) |
| 8 | 38 | Clone (NodeInner) | ✅ in (accept) | ✅ in (accept) |
| 9 | 38 | Clone (ParamBST) | ✅ in (eb) | ✅ in (eb) |
| 10 | 38 | Debug | ✅ out | ✅ out |
| 11 | 38 | Macro | ✅ out | - |
| 12 | 38 | TOC header | ✅ present | ✅ present |
| 13 | 38 | broadcast use | ✅ present | ✅ present |

**Action item:** MtEph free functions (row 6) should be moved inside `verus!` with external_body to match StEph pattern.

## Spec Strength Summary

| # | Chap | Classification | BSTParaStEph.rs | BSTParaMtEph.rs | Total |
|---|------|---------------|----------------:|----------------:|------:|
| 1 | 38 | strong | 6 | 6 | 12 |
| 2 | 38 | partial | 5 | 5 | 10 |
| 3 | 38 | weak | 1 | 1 | 2 |
| 4 | 38 | none | 3 | 3 | 6 |
| 5 | 38 | external_body holes | 32 | 19 | 51 |
| 6 | 38 | accept holes | 2 | 2 | 4 |
| 7 | 38 | free fns outside V! | 0 | 14 | 14 |

## Proposed Fixes Table

| # | Sev | File | Fix | Holes Cleared | Blocked By | Effort |
|---|:---:|------|-----|:-------------:|------------|:------:|
| 1 | critical | Both | Implement real `spec_set_view` — recursive spec connecting RwLock contents to `Set<T::V>`. | 2 (spec_set_view) | — | high |
| 2 | high | MtEph | Move 14 free fns inside `verus!` with `external_body`, matching StEph pattern. | 0 (enables future) | — | low |
| 3 | high | StEph | Remove eb from `new_leaf`, `join_m` (trivial one-liners). | 2 | #1 | low |
| 4 | high | StEph | Remove eb from `expose_internal`, `join_mid` (free fn). Define Exposed View. | 2 | #1 | medium |
| 5 | high | StEph | Remove eb from `new`, `singleton` (impl — delegate to proved free fns). | 2 | #3, #4 | low |
| 6 | high | StEph | Remove eb from `size`, `is_empty` (impl — read lock + stored field). | 2 | #1 | low |
| 7 | medium | StEph | Remove eb from `find` (recursive read-only). | 2 (impl+free) | #1, #4 | medium |
| 8 | medium | StEph | Remove eb from `split_inner`, `split` (recursive three-way). | 2 (impl+free) | #1, #4 | medium |
| 9 | medium | StEph | Remove eb from `min_key`, `join_pair_inner`, `join_pair`. | 3 | #1, #8 | medium |
| 10 | medium | StEph | Remove eb from `union_inner`/`intersect_inner`/`difference_inner` + impls. | 6 | #1, #8, #9 | medium |
| 11 | medium | StEph | Strengthen `split` spec: add L < k, R > k ordering invariant. | 0 (spec quality) | #1 | medium |
| 12 | medium | StEph | Strengthen `join_pair` spec: `result = L union R`. | 0 (spec quality) | #1 | low |
| 13 | medium | StEph | Strengthen `find` spec: `found == Some(key)` when found. | 0 (spec quality) | — | low |
| 14 | medium | StEph | Remove eb from `collect_in_order`, `in_order`. Add sorted spec. | 2 | #1 | medium |
| 15 | low | StEph | Remove eb from `filter_inner`, `filter`. Spec for higher-order fn. | 2 | #1, #8, #9 | hard |
| 16 | low | StEph | Remove eb from `reduce_inner`, `reduce`. Needs associativity spec. | 2 | #1 | hard |
| 17 | low | Both | Add `insert`/`delete` specs. Requires interior-mutability ghost state. | 0 (spec quality) | #1 | hard |
| 18 | low | Both | Remove accept holes on Clone Exposed/NodeInner with real View. | 4 | #1, #4 | low |

**Total removable holes:** 31 of 51 external_body holes are architecturally removable. 2 are vstd requirements (RwLock new), 2 are Arc::clone. The remaining 16 (MtEph impl methods) become removable after fix #2 + proving the corresponding StEph free fns.

**Root blocker:** Fix #1 (`spec_set_view`) gates nearly all other fixes. Without a real View that maps RwLock contents to `Set<T::V>`, no ensures clause can be proved.

## Overall Assessment

Chapter 38 faithfully implements all 19 prose items in both BSTParaStEph.rs (sequential) and BSTParaMtEph.rs (parallel). Both files now have trait+impl inside `verus!` with identical specs on 14 of 17 methods and all impl methods marked `external_body`.

**Strengths:**
- All 19 prose algorithms implemented in both files
- High algorithmic fidelity to prose pseudocode
- BSTParaMtEph.rs provides genuine `ParaPair!` parallelism for all five parallel algorithms
- 33 runtime tests with comprehensive functional and concurrency coverage
- Both files now share identical trait specs with View type `Set<T::V>`
- Cost annotations on all trait methods match Cost Specification 38.11
- TOC, broadcast use, and structural patterns present in both files

**Weaknesses:**
- All impl methods in both files are `external_body` — specs are trusted, not proved
- `spec_set_view` is `external_body` returning `Set::empty()` — a placeholder that makes all set-based specs vacuous
- 3 trait methods still have no specs (insert, delete, reduce)
- MtEph free functions (14) live outside `verus!` — invisible to the verifier
- 51 total external_body holes across both files

**Progress since prior review (2026-02-27):**
- BSTParaMtEph.rs trait+impl moved inside `verus!`
- BSTParaMtEph.rs View type upgraded from `()` to `Set<T::V>`
- BSTParaMtEph.rs now has specs on 14 of 17 trait methods (was 0)
- BSTParaMtEph.rs TOC header and broadcast use added
- BSTParaMtEph.rs Clone impls moved inside `verus!`
- Both files now structurally aligned (except free fn placement)
