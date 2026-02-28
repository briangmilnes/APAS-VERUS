<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 38 — Parametric BSTs: Review Against Prose

**Date:** 2026-02-27
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap38.txt`
**Source files:** `src/Chap38/BSTParaStEph.rs`, `src/Chap38/BSTParaMtEph.rs`
**Test files:** `tests/Chap38/TestBSTParaStEph.rs` (13 tests), `tests/Chap38/TestBSTParaMtEph.rs` (20 tests)
**PTT files:** None
**Verification status:** 2879 verified, 0 errors. BSTParaStEph.rs trait+impl inside `verus!` with specs on 11 of 17 trait methods; all impl methods `external_body`. BSTParaMtEph.rs trait+impl still outside `verus!` with no specs.

## Phase 1: Inventory

| # | Chap | Module | Tr | IT | IBI | V! | -V! | Holes | NoSpec |
|---|------|--------|:--:|:--:|:---:|:--:|:---:|:-----:|:------:|
| 1 | Chap38 | BSTParaStEph.rs | 17 | 17 | 0 | 17 | 11 | 19 eb | 6 |
| 2 | Chap38 | BSTParaMtEph.rs | 17 | 17 | 0 | 1 | 33 | 1 eb | 33 |

Column key: Tr = trait fns, IT = impl fns, IBI = impl bare fns, V! = fns inside verus!, -V! = fns outside verus!, Holes = external_body count, NoSpec = fns without requires/ensures.

**Totals:** 68 functions across both files. BSTParaStEph.rs: trait+impl inside `verus!`, 19 `external_body` holes, 11 trait methods with specs, 6 without. BSTParaMtEph.rs: trait+impl outside `verus!`, 1 `external_body` (RwLock), no specs.

## Phase 2: Prose Inventory

### Definitions

| # | Chap | Prose Item | Reference | Type | Description |
|---|------|-----------|-----------|------|-------------|
| 1 | Chap38 | Data Type 38.1 — K (key type) | §1 | Type | Key type supporting `<` |
| 2 | Chap38 | Data Type 38.1 — T (tree type) | §1 | Type | Opaque tree type |
| 3 | Chap38 | Data Type 38.1 — E (Exposed) | §1 | Type | `Leaf` or `Node(T × K × T)` |
| 4 | Chap38 | Data Type 38.1 — size | §1 | Fn | Number of keys in T |
| 5 | Chap38 | Data Type 38.1 — expose | §1 | Fn | Expose the root |
| 6 | Chap38 | Data Type 38.1 — joinMid | §1 | Fn | Inverse of expose; rebalances |

### Algorithms

| # | Chap | Prose Item | Reference | Description |
|---|------|-----------|-----------|-------------|
| 7 | Chap38 | Algorithm 38.2 — empty | §2 | `joinMid(Leaf)` |
| 8 | Chap38 | Algorithm 38.2 — singleton k | §2 | `joinMid(Node(empty, k, empty))` |
| 9 | Chap38 | Algorithm 38.2 — joinM(L, k, R) | §2 | `joinMid(Node(L, k, R))` |
| 10 | Chap38 | Algorithm 38.3 — split(T, k) | §2 | Three-way recursive split |
| 11 | Chap38 | Algorithm 38.4 — minKey(T, k) | §2 | Find minimum key |
| 12 | Chap38 | Algorithm 38.4 — joinPair(T1, T2) | §2 | Join two trees without middle key |
| 13 | Chap38 | Algorithm 38.5 — insert(T, k) | §2 | `split` then `joinM(L, k, R)` |
| 14 | Chap38 | Algorithm 38.5 — delete(T, k) | §2 | `split` then `joinPair(L, R)` |
| 15 | Chap38 | Algorithm 38.6 — union(T1, T2) | §3 | Parallel divide-and-conquer via split |
| 16 | Chap38 | Algorithm 38.7 — intersect(T1, T2) | §3 | Parallel, include if found in both |
| 17 | Chap38 | Algorithm 38.8 — difference(T1, T2) | §3 | Parallel, exclude if found |
| 18 | Chap38 | Algorithm 38.9 — filter f T | §3 | Parallel, joinM/joinPair on predicate |
| 19 | Chap38 | Algorithm 38.10 — reduce f I T | §3 | Parallel, `f(L', f(k, R'))` |

### Cost Specifications

| # | Chap | Prose Item | Reference | Description |
|---|------|-----------|-----------|-------------|
| 20 | Chap38 | Cost Specification 38.11 | §4 | Full cost table for all BST operations |

### Exercises

| # | Chap | Prose Item | Reference | Description |
|---|------|-----------|-----------|-------------|
| 21 | Chap38 | Exercise 38.1 | §3 | Prove correct intersection, difference, union |

### Implementation Status

| # | Chap | Prose Item | BSTParaStEph.rs | BSTParaMtEph.rs | Notes |
|---|------|-----------|:---------------:|:---------------:|-------|
| 1 | Chap38 | K (key type) | `T: StT + Ord` | `T: MtKey` | Faithful |
| 2 | Chap38 | T (tree type) | `ParamBST<T>` | `ParamBST<T>` | Arc/RwLock |
| 3 | Chap38 | E (Exposed) | `Exposed<T>` | `Exposed<T>` | Faithful |
| 4 | Chap38 | size | Yes | Yes | O(1) via stored field |
| 5 | Chap38 | expose | Yes | Yes | `expose_internal` + trait |
| 6 | Chap38 | joinMid | Yes | Yes | `join_mid` free fn + trait |
| 7 | Chap38 | empty | Yes | Yes | `ParamBST::new()` |
| 8 | Chap38 | singleton | Yes | Yes | `joinMid(Node(empty, k, empty))` |
| 9 | Chap38 | joinM | Yes | Yes | `join_m` wrapper |
| 10 | Chap38 | split | Yes | Yes | `split_inner` — exact match |
| 11 | Chap38 | minKey | Yes | Yes | Returns `Option<T>` |
| 12 | Chap38 | joinPair | Yes | Yes | `join_pair_inner` |
| 13 | Chap38 | insert | Yes | Yes | `split` + `joinM` |
| 14 | Chap38 | delete | Yes | Yes | `split` + `joinPair` |
| 15 | Chap38 | union | Yes (seq) | Yes (par) | MtEph uses `ParaPair!` |
| 16 | Chap38 | intersect | Yes (seq) | Yes (par) | Faithful |
| 17 | Chap38 | difference | Yes (seq) | Yes (par) | Faithful |
| 18 | Chap38 | filter | Yes (seq) | Yes (par) | Faithful |
| 19 | Chap38 | reduce | Yes (seq) | Yes (par) | Faithful |
| 20 | Chap38 | Cost Spec 38.11 | Yes | Yes | Doc-comments on trait |
| 21 | Chap38 | find | Yes | Yes | Not numbered in Ch38 |
| 22 | Chap38 | in_order | Yes | Yes | Extra utility |
| 23 | Chap38 | is_empty | Yes | Yes | Extra utility |
| 24 | Chap38 | ParamBSTLit! | Yes | No | Convenience macro |

**BSTParaStEph.rs:** 19 of 19 prose items implemented.
**BSTParaMtEph.rs:** 19 of 19 prose items implemented.

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Chap | Operation | APAS Work | APAS Span | Code Work | Code Span | Match? |
|---|------|----------|-----------|-----------|-----------|-----------|:------:|
| 1 | Chap38 | new/empty | O(1) | O(1) | Θ(1) | Θ(1) | Yes |
| 2 | Chap38 | expose | O(1) | O(1) | Θ(1) | Θ(1) | Yes |
| 3 | Chap38 | joinMid | O(1) | O(1) | Θ(1) | Θ(1) | Yes |
| 4 | Chap38 | size | O(1) | O(1) | Θ(1) | Θ(1) | Yes |
| 5 | Chap38 | split | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes |
| 6 | Chap38 | find | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes |
| 7 | Chap38 | insert | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes |
| 8 | Chap38 | delete | O(lg n) | O(lg n) | Θ(lg n) | Θ(lg n) | Yes |
| 9 | Chap38 | joinPair | O(lg(n1+n2)) | O(lg(n1+n2)) | Θ(lg(n1+n2)) | Θ(lg(n1+n2)) | Yes |
| 10 | Chap38 | union (Mt) | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes |
| 11 | Chap38 | union (St) | O(m·lg(n/m)) | — | Θ(m·lg(n/m)) | seq | Yes |
| 12 | Chap38 | intersect (Mt) | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes |
| 13 | Chap38 | intersect (St) | O(m·lg(n/m)) | — | Θ(m·lg(n/m)) | seq | Yes |
| 14 | Chap38 | difference (Mt) | O(m·lg(n/m)) | O(lg n) | Θ(m·lg(n/m)) | Θ(lg n) | Yes |
| 15 | Chap38 | difference (St) | O(m·lg(n/m)) | — | Θ(m·lg(n/m)) | seq | Yes |
| 16 | Chap38 | filter (Mt) | O(n) | O(lg n) | Θ(n) | Θ(lg n) | Yes |
| 17 | Chap38 | filter (St) | O(n) | — | Θ(n) | seq | Yes |
| 18 | Chap38 | reduce (Mt) | O(n) | O(lg n) | Θ(n) | Θ(lg n) | Yes |
| 19 | Chap38 | reduce (St) | O(n) | — | Θ(n) | seq | Yes |
| 20 | Chap38 | in_order | O(n) | O(n) | Θ(n) | Θ(n) | Yes |

### 3b. Implementation Fidelity

| # | Chap | Algorithm | File | Fidelity | Detail |
|---|------|----------|------|:--------:|--------|
| 1 | Chap38 | expose | Both | Faithful | Reads root via read lock |
| 2 | Chap38 | joinMid | Both | Faithful | Creates NodeInner with computed size |
| 3 | Chap38 | joinM | Both | Faithful | One-line wrapper |
| 4 | Chap38 | split | Both | Faithful | Recursive three-way compare |
| 5 | Chap38 | minKey | Both | Minor | Returns `Option<T>` vs prose `(T, k)` |
| 6 | Chap38 | joinPair | Both | Minor | Calls `min_key(&right)` then split |
| 7 | Chap38 | insert | Both | Faithful | `split` + `joinM` |
| 8 | Chap38 | delete | Both | Faithful | `split` + `joinPair` |
| 9 | Chap38 | union | Both | Faithful | Both base cases present |
| 10 | Chap38 | intersect | Both | Faithful | Conditional joinM/joinPair |
| 11 | Chap38 | difference | Both | Faithful | Inverted join logic |
| 12 | Chap38 | filter | Both | Faithful | Predicate-based join/joinPair |
| 13 | Chap38 | reduce | Both | Faithful | `op(L', op(k, R'))` |
| 14 | Chap38 | find | Both | Standard | Not numbered in Ch38 |

### 3c. Spec Fidelity

**BSTParaStEph.rs** — 11 of 17 trait methods have specs:

| # | Chap | Function | Spec | Strength | Detail |
|---|------|---------|------|:--------:|--------|
| 1 | Chap38 | new | `result@ == Set::empty()` | strong | |
| 2 | Chap38 | singleton | `result@ == Set::empty().insert(key@)` | strong | |
| 3 | Chap38 | size | `count == self@.len()` | strong | |
| 4 | Chap38 | is_empty | `empty == (self@.len() == 0)` | strong | |
| 5 | Chap38 | find | `found.is_some() <==> self@.contains(key@)` | partial | Missing: `found == Some(key)` |
| 6 | Chap38 | split | `parts.1 == self@.contains(key@)`, finite | partial | Missing: L < k, R > k |
| 7 | Chap38 | join_pair | `joined@.finite()` | weak | Missing: result = L ∪ R |
| 8 | Chap38 | union | `result@ == self@.union(other@)` | strong | |
| 9 | Chap38 | intersect | `result@ == self@.intersect(other@)` | strong | |
| 10 | Chap38 | difference | `result@ == self@.difference(other@)` | strong | |
| 11 | Chap38 | filter | `filtered@.subset_of(self@)` | partial | Missing: exact predicate spec |
| 12 | Chap38 | expose | none | none | |
| 13 | Chap38 | join_mid | none | none | |
| 14 | Chap38 | insert | none | none | Missing: mutates self |
| 15 | Chap38 | delete | none | none | Missing: mutates self |
| 16 | Chap38 | reduce | none | none | |
| 17 | Chap38 | in_order | none | none | |

**BSTParaMtEph.rs** — 0 of 17 trait methods have specs.

**Note:** The `spec_set_view` function on `ParamBST` in BSTParaStEph.rs is `external_body` returning `Set::empty()`. This is a placeholder; the View cannot track actual tree contents until the implementation connects RwLock contents to the ghost state. All `ensures` clauses in BSTParaStEph.rs are currently vacuously trusted via `external_body` on every impl method.

## Phase 4: Parallelism Review

### BSTParaMtEph.rs

| # | Chap | Function | Parallel? | Mechanism | Prose Match? |
|---|------|---------|:---------:|-----------|:------------:|
| 1 | Chap38 | `union_inner` | Yes | `ParaPair!` | Yes |
| 2 | Chap38 | `intersect_inner` | Yes | `ParaPair!` | Yes |
| 3 | Chap38 | `difference_inner` | Yes | `ParaPair!` | Yes |
| 4 | Chap38 | `filter_inner` | Yes | `ParaPair!` + `Arc<F>` | Yes |
| 5 | Chap38 | `reduce_inner` | Yes | `ParaPair!` + `Arc<F>` | Yes |
| 6 | Chap38 | `collect_in_order` | No | Sequential traversal | N/A |
| 7 | Chap38 | All others | No | Sequential per-tree ops | N/A |

### Thread Safety

- `ParamBST<T>` uses `Arc<RwLock<Option<Box<NodeInner>>>>`.
- `insert`/`delete` use `write()` locks; `expose`/`find`/`split` use `read()` locks.
- `ParaPair!` macro handles fork-join threading.
- `insert`/`delete` are NOT linearizable: read-compute-write without holding lock across full operation.

## Phase 5: Runtime Test Review

### TestBSTParaStEph.rs (13 tests)

| # | Chap | Test | Operations Covered |
|---|------|----|-----|
| 1 | Chap38 | `test_parambstlit_macro_functionality` | ParamBSTLit!, size, find |
| 2 | Chap38 | `para_basic_insert_find` | insert, find, size, is_empty, in_order |
| 3 | Chap38 | `para_split_and_join_pair` | split, join_pair, in_order |
| 4 | Chap38 | `para_union_and_delete` | union, delete, find, in_order |
| 5 | Chap38 | `para_join_mid_expose_roundtrip` | expose, join_mid, size |
| 6 | Chap38 | `para_singleton` | singleton, size, find, in_order |
| 7 | Chap38 | `para_intersect_and_difference` | intersect, difference, in_order |
| 8 | Chap38 | `para_filter_and_reduce` | filter, reduce |
| 9 | Chap38 | `para_intersect_and_difference_large` | 256-element intersect, difference |
| 10 | Chap38 | `para_filter_and_reduce_edge_cases` | filter, reduce edge cases |
| 11 | Chap38 | `para_split_nonexistent_key` | split at missing key |
| 12 | Chap38 | `para_ops_on_empty_tree` | empty tree operations |
| 13 | Chap38 | `para_delete_nonexistent_key` | delete non-existent key |

### TestBSTParaMtEph.rs (20 tests)

| # | Chap | Test | Operations Covered |
|---|------|----|-----|
| 1 | Chap38 | `para_basic_insert_find` | insert, find, size, is_empty, in_order |
| 2 | Chap38 | `para_split_and_join_pair` | split, join_pair, in_order |
| 3 | Chap38 | `para_union_and_delete` | union, delete, find, in_order |
| 4 | Chap38 | `para_join_mid_expose_roundtrip` | expose, join_mid, size |
| 5 | Chap38 | `para_intersect_and_difference` | intersect, difference, in_order |
| 6 | Chap38 | `para_filter_and_reduce` | filter, reduce |
| 7 | Chap38 | `para_union_large_balanced` | 200+200 element union |
| 8 | Chap38 | `para_intersect_and_difference_large` | 256-element intersect, difference |
| 9 | Chap38 | `para_filter_and_reduce_edge_cases` | filter, reduce edge cases |
| 10 | Chap38 | `para_singleton` | singleton, size, find, in_order |
| 11 | Chap38 | `para_split_nonexistent_key` | split at missing key |
| 12 | Chap38 | `para_ops_on_empty_tree` | empty tree operations |
| 13 | Chap38 | `para_delete_nonexistent_key` | delete non-existent key |
| 14 | Chap38 | `para_concurrent_insertions` | 4-thread concurrent insert+find |
| 15 | Chap38 | `para_concurrent_operations_stress` | 6-thread stress test |
| 16 | Chap38 | `para_concurrent_set_operations` | Concurrent union/intersect/diff |
| 17 | Chap38 | `para_concurrent_filter_reduce` | Concurrent filter/reduce |
| 18 | Chap38 | `para_concurrent_split_join` | Concurrent split at 25/50/75 |
| 19 | Chap38 | `para_concurrent_expose_join_mid` | Concurrent tree construction |
| 20 | Chap38 | `para_concurrent_delete_operations` | Concurrent delete+find |

All 33 tests pass.

## Phase 6: PTT Review

No PTTs exist. Neither file contains verified loops or iterators inside `verus!`. No PTTs needed.

## Phase 7: Gap Analysis

### Proof Holes

| # | Chap | File | Line | Type | Function | Removable? |
|---|------|------|-----:|------|----------|:----------:|
| 1 | Chap38 | BSTParaStEph.rs | 70 | eb (RwLock) | `new_bst_para_lock` | No — vstd req |
| 2 | Chap38 | BSTParaStEph.rs | 78 | eb | `spec_set_view` | Yes — needs recursive spec |
| 3 | Chap38 | BSTParaStEph.rs | 158 | eb | `new` | Yes — straightforward |
| 4 | Chap38 | BSTParaStEph.rs | 163 | eb | `singleton` | Yes — straightforward |
| 5 | Chap38 | BSTParaStEph.rs | 172 | eb | `expose` | Yes — needs Exposed View |
| 6 | Chap38 | BSTParaStEph.rs | 175 | eb | `join_mid` | Yes — needs Exposed View |
| 7 | Chap38 | BSTParaStEph.rs | 178 | eb | `size` | Yes — needs spec_set_view |
| 8 | Chap38 | BSTParaStEph.rs | 188 | eb | `is_empty` | Yes — needs spec_set_view |
| 9 | Chap38 | BSTParaStEph.rs | 193 | eb | `insert` | Medium — mutation + RwLock |
| 10 | Chap38 | BSTParaStEph.rs | 204 | eb | `delete` | Medium — mutation + RwLock |
| 11 | Chap38 | BSTParaStEph.rs | 215 | eb | `find` | Yes — recursive read-only |
| 12 | Chap38 | BSTParaStEph.rs | 229 | eb | `split` | Yes — recursive read-only |
| 13 | Chap38 | BSTParaStEph.rs | 237 | eb | `join_pair` | Yes — delegates to split+joinM |
| 14 | Chap38 | BSTParaStEph.rs | 242 | eb | `union` | Medium — recursive + set spec |
| 15 | Chap38 | BSTParaStEph.rs | 247 | eb | `intersect` | Medium — recursive + set spec |
| 16 | Chap38 | BSTParaStEph.rs | 252 | eb | `difference` | Medium — recursive + set spec |
| 17 | Chap38 | BSTParaStEph.rs | 257 | eb | `filter` | Hard — higher-order fn spec |
| 18 | Chap38 | BSTParaStEph.rs | 264 | eb | `reduce` | Hard — needs associativity |
| 19 | Chap38 | BSTParaStEph.rs | 269 | eb | `in_order` | Medium — needs Seq view |
| 20 | Chap38 | BSTParaStEph.rs | 308 | eb | `clone` (ParamBST) | No — Arc::clone |
| 21 | Chap38 | BSTParaStEph.rs | 287 | accept | Clone Exposed | Removable with real View |
| 22 | Chap38 | BSTParaStEph.rs | 302 | accept | Clone NodeInner | Removable with real View |
| 23 | Chap38 | BSTParaMtEph.rs | 46 | eb (RwLock) | `new_bst_para_lock` | No — vstd req |

### Missing Specs

| # | Chap | File | Function | Spec Needed |
|---|------|------|----------|-------------|
| 1 | Chap38 | BSTParaStEph.rs | expose | ensures on Exposed variant |
| 2 | Chap38 | BSTParaStEph.rs | join_mid | ensures on result set view |
| 3 | Chap38 | BSTParaStEph.rs | insert | ensures self@ == old(self@).insert(key@) |
| 4 | Chap38 | BSTParaStEph.rs | delete | ensures self@ == old(self@).remove(key@) |
| 5 | Chap38 | BSTParaStEph.rs | reduce | ensures on reduction value |
| 6 | Chap38 | BSTParaStEph.rs | in_order | ensures sorted, same elements |
| 7 | Chap38 | BSTParaMtEph.rs | all 17 | No specs at all |

### Style Warnings

| # | Chap | File | Warning | Detail |
|---|------|------|---------|--------|
| 1 | Chap38 | BSTParaStEph.rs | [11] | Set usage — `group_set_axioms` present (false positive) |
| 2 | Chap38 | BSTParaStEph.rs | [12] | 6 trait fns missing requires/ensures |
| 3 | Chap38 | BSTParaStEph.rs | [18] | Type defs appear after RwLockPredicate impl |
| 4 | Chap38 | BSTParaStEph.rs | [19] | 6 return names use generic `result` |
| 5 | Chap38 | BSTParaMtEph.rs | [13] | `impl ParamBSTTrait` outside verus! |
| 6 | Chap38 | BSTParaMtEph.rs | [15] | 3 Clone impls outside verus! |
| 7 | Chap38 | BSTParaMtEph.rs | [18] | Type defs appear after RwLockPredicate impl |

## Phase 8: TOC Review

### In/Out Table

| # | Chap | Item | BSTParaStEph.rs | BSTParaMtEph.rs |
|---|------|------|:---------------:|:---------------:|
| 1 | Chap38 | Type definitions | ✅ in | ✅ in |
| 2 | Chap38 | View impls | ✅ in | - |
| 3 | Chap38 | RwLockPredicate | ✅ in | ✅ in |
| 4 | Chap38 | Trait | ✅ in | ❌ out |
| 5 | Chap38 | Impl | ✅ in (eb) | ❌ out |
| 6 | Chap38 | Clone (Exposed) | ✅ in (accept) | ❌ out |
| 7 | Chap38 | Clone (NodeInner) | ✅ in (accept) | ❌ out |
| 8 | Chap38 | Clone (ParamBST) | ✅ in (eb) | ❌ out |
| 9 | Chap38 | Debug | ✅ out | ✅ out |
| 10 | Chap38 | Macro | ✅ out | - |
| 11 | Chap38 | Free fns | ❌ out | ❌ out |
| 12 | Chap38 | TOC header | ✅ present | ❌ missing |
| 13 | Chap38 | broadcast use | ✅ present | ❌ missing |

## Spec Strength Summary

| # | Chap | Classification | BSTParaStEph.rs | BSTParaMtEph.rs | Total |
|---|------|---------------|----------------:|----------------:|------:|
| 1 | Chap38 | strong | 5 | 0 | 5 |
| 2 | Chap38 | partial | 3 | 0 | 3 |
| 3 | Chap38 | weak | 1 | 0 | 1 |
| 4 | Chap38 | none | 6 | 17 | 23 |
| 5 | Chap38 | hole (external_body) | 19 | 1 | 20 |
| 6 | Chap38 | accept | 2 | 0 | 2 |

## Overall Assessment

Chapter 38 faithfully implements all 19 prose items in both BSTParaStEph.rs (sequential) and BSTParaMtEph.rs (parallel) variants. BSTParaStEph.rs has been partially verusified: trait+impl are inside `verus!` with specs on 11 of 17 methods, View type upgraded to `Set<T::V>`, and broadcast use added. BSTParaMtEph.rs remains entirely un-verusified.

**Strengths:**
- All 19 prose algorithms implemented in both files
- High algorithmic fidelity to prose pseudocode
- BSTParaMtEph.rs provides genuine `ParaPair!` parallelism for all five parallel algorithms
- 33 runtime tests with comprehensive functional and concurrency coverage
- BSTParaStEph.rs trait+impl now inside `verus!` with meaningful specs on set operations
- Cost annotations on all trait methods match Cost Specification 38.11

**Weaknesses:**
- All 19 BSTParaStEph.rs impl methods are `external_body` — specs are trusted, not proved
- `spec_set_view` is `external_body` returning `Set::empty()` — a placeholder
- 6 BSTParaStEph.rs trait methods still have no specs (expose, join_mid, insert, delete, reduce, in_order)
- BSTParaMtEph.rs completely un-verusified: trait+impl outside `verus!`, no specs, no View
- Free functions (expose_internal, join_mid, split_inner, etc.) live outside `verus!` in both files
- Generic `result` return names on 6 functions
- BSTParaMtEph.rs missing TOC and broadcast use

**Progress since prior review (2026-02-19):**
- BSTParaStEph.rs trait+impl moved inside `verus!`
- BSTParaStEph.rs View type upgraded from `()` to `Set<T::V>`
- 11 trait methods now have `ensures` clauses
- `broadcast use vstd::set::group_set_axioms` added
- TOC added to BSTParaStEph.rs
- `accept` import added for Clone holes
