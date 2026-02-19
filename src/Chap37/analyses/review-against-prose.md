<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 37 — Introduction to Binary Search Trees: Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap37.txt`
**Source directory:** `src/Chap37/` (19 files)
**Test directory:** `tests/Chap37/` (24 files)
**PTT directory:** none (0 files)
**Verification status:** Partially verusified — 7 files have `verus!` blocks with specs/proofs; 12 files remain plain Rust

## Phase 1: Inventory

Module summary from `veracity-review-module-fn-impls -d src/Chap37` (415 entries across 19 files).

| # | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | AVLTreeSeq | 20 | 23 | 2 | 10 | 0 | 34 | 0 | 0 | 34 |
| 2 | AVLTreeSeqMtPer | 11 | 14 | 0 | 11 | 0 | 25 | 0 | 0 | 25 |
| 3 | AVLTreeSeqStEph | 18 | 21 | 2 | 9 | 0 | 31 | 0 | 0 | 31 |
| 4 | AVLTreeSeqStPer | 13 | 15 | 1 | 11 | 0 | 27 | 0 | 0 | 27 |
| 5 | BSTAVLMtEph | 0 | 0 | 6 | 8 | 14 | 0 | 6 | 2 | 6 |
| 6 | BSTAVLStEph | 0 | 0 | 0 | 17 | 17 | 0 | 14 | 1 | 2 |
| 7 | BSTBBAlphaMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 2 | 6 |
| 8 | BSTBBAlphaStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 9 | BSTPlainMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 2 | 6 |
| 10 | BSTPlainStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 11 | BSTRBMtEph | 14 | 16 | 0 | 19 | 0 | 35 | 0 | 0 | 35 |
| 12 | BSTRBStEph | 0 | 0 | 0 | 15 | 15 | 0 | 13 | 0 | 2 |
| 13 | BSTSetAVLMtEph | 20 | 20 | 0 | 3 | 0 | 23 | 0 | 0 | 23 |
| 14 | BSTSetBBAlphaMtEph | 20 | 20 | 3 | 0 | 0 | 23 | 0 | 0 | 23 |
| 15 | BSTSetPlainMtEph | 20 | 20 | 3 | 0 | 0 | 23 | 0 | 0 | 23 |
| 16 | BSTSetRBMtEph | 20 | 20 | 3 | 0 | 0 | 23 | 0 | 0 | 23 |
| 17 | BSTSetSplayMtEph | 20 | 20 | 3 | 0 | 0 | 23 | 0 | 0 | 23 |
| 18 | BSTSplayMtEph | 14 | 16 | 0 | 16 | 0 | 32 | 0 | 0 | 32 |
| 19 | BSTSplayStEph | 11 | 13 | 0 | 11 | 0 | 24 | 0 | 0 | 24 |

**Key:** 7 verusified files (rows 5–10, 12) total 92 V! functions with 59 having specs (Unk) and 7 having holes. 12 plain Rust files total 323 functions, all NoSpec.

## Phase 2: Prose Inventory

### Definitions

| # | Definition | Description | Implemented |
|---|-----------|-------------|-------------|
| 1 | Def 37.1 — Full Binary Tree | Recursive type: `Leaf | Node(tree × α × tree)` | Yes — `BalBinTree<T>` in Chap23 (verusified files); `Option<Box<Node<T>>>` in plain Rust |
| 2 | Def 37.2 — In-Order Traversal | `inOrder(L) ++ ⟨k⟩ ++ inOrder(R)` | Yes — `in_order_collect` / `in_order_parallel` in MtEph files; sequential in StEph |
| 3 | Def 37.3 — Binary Search Tree | Full binary tree where `inOrder(T)` is sorted by `<` | Yes — `tree_is_bst` spec fn in BSTPlainStEph, imported by all verusified files |
| 4 | Def 37.5 — Perfectly Balanced BST | Height exactly ⌈lg(n+1)⌉ | Not formalized — no spec fn for perfect balance |
| 5 | Def 37.6 — Nearly Balanced BST | Height O(lg n) for all trees satisfying balancing invariants | Partially — `avl_balanced`, `weight_balanced` spec fns exist but O(lg n) height bound not proven |

### Algorithms

| # | Algorithm | Description | Implemented | Verified |
|---|-----------|-------------|-------------|----------|
| 1 | Algorithm 37.4 — Searching a BST | `find T k`: recursive search comparing key with root | Yes — all BST files | Yes — 7 verusified files with `ensures result == tree_contains(...)` |

### Data Types / ADT

| # | ADT | Functions | Implemented |
|---|-----|----------|-------------|
| 1 | Data Type 37.7 — BST ADT | `empty`, `singleton`, `size`, `find`, `delete`, `insert`, `union`, `intersection`, `difference`, `split`, `joinPair`, `joinM`, `filter`, `reduce` | All 14 operations in BSTSet* wrappers; base BST types implement subset (`new`, `insert`, `find`, `contains`, `size`) |

### Cost Specs from Prose

| # | Operation | APAS Work | APAS Span | Notes |
|---|-----------|-----------|-----------|-------|
| 1 | find | O(h(T)) | O(h(T)) | |
| 2 | empty | Θ(1) | Θ(1) | |
| 3 | singleton | Θ(1) | Θ(1) | |
| 4 | size | Θ(1) | Θ(1) | Cached at root in MtEph files |
| 5 | insert | O(lg n) | O(lg n) | For balanced BSTs |
| 6 | delete | O(lg n) | O(lg n) | For balanced BSTs |
| 7 | union / intersection / difference | O(m · lg(n/m)) | O(lg² n) | Chap 38 parametric |
| 8 | split | O(lg n) | O(lg n) | |
| 9 | joinPair / joinM | O(lg n) | O(lg n) | |
| 10 | filter | Θ(n) | Θ(lg n) | Parallel |
| 11 | reduce | Θ(n) | Θ(lg n) | Parallel |

### Balancing Schemes Discussed

| # | Scheme | Balance Guarantee | Implemented | Verusified | Balance Invariant Spec |
|---|--------|-------------------|-------------|------------|----------------------|
| 1 | AVL trees | h(T) = O(lg n) worst-case | Yes — `BSTAVLStEph`, `BSTAVLMtEph` | Yes | `avl_balanced` spec fn |
| 2 | Red-Black trees | h(T) = O(lg n) worst-case | Yes — `BSTRBStEph`, `BSTRBMtEph` | StEph only | No color spec (BalBinTree lacks color field) |
| 3 | Weight-balanced (BB[α]) | h(T) = O(lg n) worst-case | Yes — `BSTBBAlphaStEph`, `BSTBBAlphaMtEph` | Yes | `weight_balanced` spec fn |
| 4 | Treaps | h(T) = O(lg n) w.h.p. | No — deferred to Chap 38/39 | N/A | N/A |
| 5 | Splay trees | O(lg n) amortized | Naming only — no splay rotations | No | N/A |

### Theorems / Properties

| # | Property | Proved in Code | Notes |
|---|----------|:--------------:|-------|
| 1 | BST property — in-order is sorted | Partial | `tree_is_bst` defined but equivalence with sorted in-order not formalized |
| 2 | Find visits at most h(T) nodes | No | Follows from structure but no formal cost proof |
| 3 | Near-balance implies h = O(lg n) | No | Balance specs exist but height bound not proven |
| 4 | Insert preserves BST property | Yes | All 7 verusified files prove `tree_is_bst(result)` |
| 5 | Insert preserves containment | Yes | `tree_contains(result, x) <==> (tree_contains(old, x) || x == value)` |
| 6 | Find correctness | Yes | `ensures result == tree_contains(node, target)` in 7 verusified files |
| 7 | Rotation preserves BST + containment | Yes | AVL, RB, BB[α] files prove rotations preserve BST and containment |

## Phase 3: Algorithmic Analysis

### 3a. Verusification Status by Variant Family

| # | Family | File | verus! | Spec fns | Proof fns | Exec fns verified | Balance spec |
|---|--------|------|:------:|:--------:|:---------:|:-----------------:|:------------:|
| 1 | Plain | BSTPlainStEph.rs | Yes | `tree_contains`, `tree_is_bst` | — | insert, contains, find, new, size, is_empty, height | N/A (unbalanced) |
| 2 | Plain | BSTPlainMtEph.rs | Yes | (imports from StEph) | — | insert, contains, find, new, size, height, is_empty | N/A |
| 3 | AVL | BSTAVLStEph.rs | Yes | `avl_balanced`, `tree_is_avl` | `lemma_bst_deep`, `lemma_max_plus_one` | rotate_right, rotate_left, rebalance, insert, contains, find, new, size, is_empty, height | Yes |
| 4 | AVL | BSTAVLMtEph.rs | Yes | (imports from StEph) | `lemma_bst_deep` | rotate_right, rotate_left, insert, contains, find, min, max + lock ops | Yes (imported) |
| 5 | RB | BSTRBStEph.rs | Yes | (uses AVL's `avl_balanced` import, unused) | `lemma_bst_deep` | rotate_right, rotate_left, insert, contains, find, new, size, is_empty, height | No (no color field) |
| 6 | RB | BSTRBMtEph.rs | **No** | — | — | — | Yes (exec RB rebalancing) |
| 7 | BB[α] | BSTBBAlphaStEph.rs | Yes | `weight_balanced`, `tree_is_bb` | — | insert, contains, find, new, size, is_empty, height | Yes |
| 8 | BB[α] | BSTBBAlphaMtEph.rs | Yes | (imports from StEph) | — | insert, contains, find, min, max + lock ops | Yes (imported) |
| 9 | Splay | BSTSplayStEph.rs | **No** | — | — | — | N/A |
| 10 | Splay | BSTSplayMtEph.rs | **No** | — | — | — | N/A |
| 11–15 | BSTSet* (5 files) | **No** | — | — | — | N/A |
| 16–19 | AVLTreeSeq* (4 files) | **No** | — | — | — | N/A |

### 3b. Implementation Fidelity — Algorithm 37.4 (Searching a BST)

All BST files implement `find` / `find_link` / `find_node` matching the prose exactly: compare key with root, recurse left if `<`, recurse right if `>`, return if `=`. Work O(h(T)) as stated.

Verusified files (Plain, AVL, RB, BB[α] StEph/MtEph) prove `ensures result.is_some() == tree_contains(node, target)` with an antisymmetric argument to rule out the wrong branch.

### 3c. Implementation Fidelity — Data Type 37.7 (BST ADT)

| # | ADT Operation | APAS Cost | Verusified? | BSTSet Cost | Notes |
|---|---------------|-----------|:-----------:|-------------|-------|
| 1 | empty | Θ(1) | Yes | Θ(1) | |
| 2 | singleton | Θ(1) | — | Θ(1) | |
| 3 | size | Θ(1) | Yes | Θ(1) cached | |
| 4 | find | O(h(T)) | Yes (proved) | O(h(T)) | |
| 5 | delete | O(lg n) | — | **O(n) rebuild** | Linear scan + rebuild |
| 6 | insert | O(lg n) | Yes (proved) | O(h(T)) | |
| 7 | union | O(m·lg(n/m)) | — | **O(n²)** | O(n) split |
| 8 | intersection | O(m·lg(n/m)) | — | **O(n²)** | O(n) split |
| 9 | difference | O(m·lg(n/m)) | — | **O(n²)** | O(n) split |
| 10 | split | O(lg n) | — | **O(n)** | Linear scan |
| 11 | joinPair | O(lg n) | — | **O(n)** | BTreeSet rebuild |
| 12 | joinM | O(lg n) | — | **O(n)** | BTreeSet rebuild |
| 13 | filter | Θ(n), S Θ(lg n) | — | Θ(n) **sequential** | |
| 14 | reduce | Θ(n), S Θ(lg n) | — | Θ(n) **sequential** | |

### 3d. Key Deviations from Prose

| # | Deviation | Severity |
|---|-----------|:--------:|
| 1 | `delete` is O(n) via linear rebuild instead of O(lg n) tree surgery | High |
| 2 | `split` is O(n) via traversal instead of O(lg n) recursive decomposition | High |
| 3 | `join_pair`/`join_m` are O(n) via BTreeSet rebuild instead of O(lg n) | High |
| 4 | Splay trees don't splay — use plain unbalanced BST insert | High |
| 5 | RB color invariant not verified — `BalBinTree` lacks color field | Medium |
| 6 | AVL `avl_insert` only ensures `tree_is_bst`, not `tree_is_avl` | Medium |
| 7 | BSTSet `filter`/`reduce` are sequential despite Mt naming | Medium |
| 8 | No in-order sorted spec equivalence | Low |

### 3e. Spec Strength — Verusified Functions

| # | Function | File | Spec Strength | Notes |
|---|----------|------|:-------------:|-------|
| 1 | `tree_contains` | BSTPlainStEph | strong | Recursive membership predicate |
| 2 | `tree_is_bst` | BSTPlainStEph | strong | BST ordering invariant |
| 3 | `avl_balanced` | BSTAVLStEph | strong | |h(L) - h(R)| ≤ 1 at every node |
| 4 | `weight_balanced` | BSTBBAlphaStEph | strong | 4·size(child) ≤ 3·total at every node |
| 5 | `insert_node` (all variants) | 7 files | strong | BST preserved, containment iff old or new |
| 6 | `contains_node` (all variants) | 7 files | strong | result == tree_contains |
| 7 | `find_node` (all variants) | 7 files | strong | Some iff tree_contains, value matches |
| 8 | `rotate_right/left` | AVL/RB/BB[α] | strong | BST preserved, containment preserved |
| 9 | `bst_new/avl_new/rb_new/bb_new` | 4 StEph files | strong | BST property, empty tree |
| 10 | `size/height/is_empty` | All verusified | strong | Exact spec_size/spec_height match |
| 11 | `min_node/max_node` | 7 files | weak | Only `decreases`, no ensures |
| 12 | All plain Rust functions | 12 files | none | No specs |

## Phase 4: Parallelism Review

### Verusified Mt Files

| # | Module | Classification | Mechanism | Notes |
|---|--------|:--------------:|-----------|-------|
| 1 | BSTPlainMtEph | Sequential | vstd::rwlock (verified predicate) | All ops sequential under lock |
| 2 | BSTAVLMtEph | Sequential | std::sync::RwLock (external_body) | All ops sequential under lock |
| 3 | BSTBBAlphaMtEph | Sequential | std::sync::RwLock (external_body) | All ops sequential under lock |

### Plain Rust Mt Files — Genuine Parallel Operations

| # | Module | Function | Parallel? | Mechanism |
|---|--------|----------|:---------:|-----------|
| 1 | BSTRBMtEph | `in_order` / `pre_order` | **Yes** | `ParaPair!` fork-join on subtrees |
| 2 | BSTRBMtEph | `from_sorted_slice` | **Yes** | `ParaPair!` recursive construction |
| 3 | BSTRBMtEph | `filter` / `reduce` | **Yes** | `ParaPair!` + `Arc<F>` |
| 4 | BSTSplayMtEph | Same set | **Yes** | Same mechanism |
| 5 | BSTSetPlainMtEph | `union`/`intersection`/`difference` | **Yes** | `ParaPair!` recursive split-join |
| 6 | BSTSetAVLMtEph | Same | **Yes** | Same |
| 7 | BSTSetRBMtEph | Same | **Yes** | Same |
| 8 | BSTSetSplayMtEph | Same | **Yes** | Same |

### BSTSet Span Audit

| # | Function | APAS Span | Actual Span | Match? | Root Cause |
|---|----------|-----------|-------------|:------:|------------|
| 1 | union | O(lg² n) | **O(n²)** | **No** | O(n) split/join |
| 2 | intersection | O(lg² n) | **O(n²)** | **No** | O(n) split/join |
| 3 | difference | O(lg² n) | **O(n²)** | **No** | O(n) split/join |
| 4 | filter | Θ(lg n) | **Θ(n)** | **No** | Sequential `iter().filter_map()` |
| 5 | reduce | Θ(lg n) | **Θ(n)** | **No** | Sequential `fold()` |
| 6 | delete | O(lg n) | **O(n)** | **No** | Linear scan + rebuild |

## Phase 5: Runtime Test Review

| # | Source Module | Test File(s) | Tests |
|---|-------------|-------------|:-----:|
| 1 | AVLTreeSeq.rs | TestAVLTreeSeq.rs | 55 |
| 2 | AVLTreeSeqStEph.rs | TestAVLTreeSeqStEph.rs, TestAVLTreeSeqStEphChap37.rs, TestAVLTreeSeqStEph18.rs | 50 |
| 3 | AVLTreeSeqStPer.rs | TestAVLTreeSeqStPer.rs, TestAVLTreeSeqStPer18.rs, TestAVLTreeSeqStPer19.rs | 40 |
| 4 | AVLTreeSeqMtPer.rs | TestAVLTreeSeqMtPer.rs | 44 |
| 5 | BSTPlainStEph.rs | TestBSTPlainStEph.rs | 9 |
| 6 | BSTPlainMtEph.rs | TestBSTPlainMtEph.rs | 10 |
| 7 | BSTAVLStEph.rs | TestBSTAVLStEph.rs | 9 |
| 8 | BSTAVLMtEph.rs | TestBSTAVLMtEph.rs | 25 |
| 9 | BSTRBStEph.rs | TestBSTRBStEph.rs | 9 |
| 10 | BSTRBMtEph.rs | TestBSTRBMtEph.rs | 25 |
| 11 | BSTBBAlphaStEph.rs | TestBSTBBAlphaStEph.rs | 9 |
| 12 | BSTBBAlphaMtEph.rs | TestBSTBBAlphaMtEph.rs | 13 |
| 13 | BSTSplayStEph.rs | TestBSTSplayStEph.rs | 10 |
| 14 | BSTSplayMtEph.rs | TestBSTSplayMtEph.rs | 23 |
| 15 | BSTSetPlainMtEph.rs | TestBSTSetPlainMtEph.rs | 49 |
| 16 | BSTSetAVLMtEph.rs | TestBSTSetAVLMtEph.rs | 42 |
| 17 | BSTSetRBMtEph.rs | TestBSTSetRBMtEph.rs | 42 |
| 18 | BSTSetBBAlphaMtEph.rs | TestBSTSetBBAlphaMtEph.rs | 42 |
| 19 | BSTSetSplayMtEph.rs | TestBSTSetSplayMtEph.rs | 42 |

**Cross-cutting test**: `TestBSTMtEph.rs` (19 tests) exercises all 5 Mt BST variants uniformly.

**Total tests**: ~568 across 24 test files.

**Coverage quality**: All 19 source modules have RTT files. Tests cover basic ops (insert, find, contains, min, max, size), ordering (in_order), and set operations (union, intersection, difference, split, join, filter, reduce). Missing: concurrent insert+find stress tests, property-based tests.

## Phase 6: PTT Review

**No PTTs exist** for Chapter 37. `rust_verify_test/tests/Chap37/` does not exist.

PTTs would be valuable for:
- `tree_is_bst` preservation across sequences of operations
- `tree_contains` completeness (all inserted values found)
- Balance invariant preservation
- Rotation correctness under various tree shapes

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Efficient O(lg n) split | Missing | BSTSet split is O(n) via traversal |
| 2 | Efficient O(lg n) joinPair | Missing | BSTSet join is O(n) via BTreeSet |
| 3 | Efficient O(lg n) joinM (joinMid) | Missing | BSTSet joinM is O(n) via BTreeSet |
| 4 | Efficient O(lg n) delete | Missing | BSTSet delete is O(n) via rebuild |
| 5 | Splay rotations (zig, zig-zig, zig-zag) | Missing | BSTSplay* uses plain BST insert |
| 6 | RB color invariant | Missing | `BalBinTree` has no color field |
| 7 | AVL balance preservation on insert | Missing | `avl_insert` ensures BST but not `tree_is_avl` |
| 8 | In-order traversal spec function | Missing | `tree_is_bst` via containment, not sorted in-order |
| 9 | Perfect balance spec (Def 37.5) | Missing | No spec fn for `height == ceil(lg(n+1))` |
| 10 | Treaps | Not in Chap37 | Deferred to Chap 38/39 |

### Code with No Direct Prose Counterpart

| # | Code Item | Notes |
|---|----------|-------|
| 1 | AVLTreeSeq, AVLTreeSeqStEph, AVLTreeSeqStPer, AVLTreeSeqMtPer | Implicit-order sequence trees, not BSTs — supports nth, set, subseq. Better fit for Chap 18-19. |
| 2 | `from_sorted_slice`, `build_balanced` | Construction helpers for parallel tree building |
| 3 | `pre_order` / `pre_order_parallel` | Traversal — defined in prose (Def 37.2) but not part of the ADT |
| 4 | `rotate_right`, `rotate_left` | Rotation primitives — implied by balancing discussion but no explicit algorithm |
| 5 | `lemma_bst_deep` | Proof helper — decomposes BST invariant two levels deep for rotation proofs |
| 6 | `BstPred<T>` / `RwLockPredicate` | Lock invariant infrastructure for verified Mt access |

## Phase 8: TOC Review

### TOC Standard Compliance

| # | File | Has TOC | Compliant |
|---|------|:-------:|:---------:|
| 1 | BSTPlainStEph.rs | Yes | Yes |
| 2 | BSTPlainMtEph.rs | Yes | Yes |
| 3 | BSTAVLStEph.rs | Yes | Yes |
| 4 | BSTAVLMtEph.rs | Yes | Yes |
| 5 | BSTRBStEph.rs | Yes | Yes |
| 6 | BSTBBAlphaStEph.rs | Yes | Yes |
| 7 | BSTBBAlphaMtEph.rs | Yes | Yes |
| 8–19 | All plain Rust files | No | N/A — TOC standard applies to verusified files |

### In/Out Table (verusified files)

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BSTPlainStEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 2 | BSTPlainMtEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 3 | BSTAVLStEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 4 | BSTAVLMtEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 5 | BSTRBStEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 6 | BSTBBAlphaStEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 7 | BSTBBAlphaMtEph.rs | - | - | - | - | - | - | - | ✅ out | - |

## Proof Holes Summary

From `veracity-review-verus-proof-holes -d src/Chap37/`:

```
✓ BSTPlainStEph.rs — clean
✓ BSTBBAlphaStEph.rs — clean
✓ BSTRBStEph.rs — 1 clean proof fn
✓ BSTRBMtEph.rs — clean (plain Rust)
✓ BSTSplayStEph.rs — clean (plain Rust)
✓ BSTSplayMtEph.rs — clean (plain Rust)
✓ BSTSetAVLMtEph.rs — clean (plain Rust)

❌ BSTAVLStEph.rs — 9 × assume() in rebalance
❌ BSTAVLMtEph.rs — 2 × assume() (spec_size/spec_height <= usize::MAX)
❌ BSTBBAlphaMtEph.rs — 2 × assume() (spec_size/spec_height <= usize::MAX)
❌ BSTPlainMtEph.rs — 2 × assume() (spec_size/spec_height <= usize::MAX)

+ 9 bare_impl errors across AVLTreeSeq*, BSTSet* files

Holes Found: 15 total (15 × assume)
```

### Hole Analysis

| # | File | Holes | Type | Justification | Avoidable? |
|---|------|:-----:|------|---------------|:----------:|
| 1 | BSTAVLStEph | 9 | assume | `avl_balanced(result)` and `height <= old_height` after rotations in rebalance | Yes — need rotation height lemmas |
| 2 | BSTAVLMtEph | 2 | assume | `spec_size() <= usize::MAX` | Tolerable — practical overflow guard |
| 3 | BSTBBAlphaMtEph | 2 | assume | `spec_size() <= usize::MAX`, `spec_height() <= usize::MAX` | Tolerable |
| 4 | BSTPlainMtEph | 2 | assume | Same | Tolerable |

**Key observation**: BSTPlainMtEph uses vstd::rwlock with verified predicate (zero external_body on lock ops). BSTAVLMtEph and BSTBBAlphaMtEph use std::sync::RwLock, requiring external_body on all lock operations — those external_body holes are counted in the veracity output but not in the assume-only count above.

## Spec Strength Summary

| Classification | Count |
|:--------------:|:-----:|
| strong | ~59 |
| partial | 0 |
| weak | ~4 (min_node/max_node) |
| none | ~352 (all plain Rust functions) |

The strong specs are concentrated in the 7 verusified files. All plain Rust files (12 of 19) have zero specs.

## Overall Assessment

### Architecture

The chapter has three layers:

1. **Verusified core** (7 files): Functional-style BST on `BalBinTree<T>`, with `tree_is_bst` spec and full insert/contains/find proofs. Uses the shared proof pattern from `BSTPlainStEph` (spec fns) imported by all variants.

2. **Plain Rust MtEph** (4 files): Imperative `Option<Box<Node<T>>>` trees with actual RB rebalancing (BSTRBMtEph), parallel traversals via `ParaPair!`, and `Arc<RwLock>` concurrency. Splay-style naming but no splay rotations (BSTSplayMtEph).

3. **BSTSet wrappers** (5 files): Implement the full Data Type 37.7 ADT by wrapping an MtEph BST. Parallel `union`/`intersection`/`difference` via `ParaPair!` split-join, but crippled by O(n) `split`/`join` primitives.

### Strengths

| # | Strength |
|---|----------|
| 1 | Strong core verification: 7 files with clean insert/find proofs and `tree_is_bst` preservation |
| 2 | Verified lock pattern: BSTPlainMtEph demonstrates vstd::rwlock with verified `BstPred` predicate |
| 3 | Complete ADT coverage: All 14 operations from Data Type 37.7 implemented in BSTSet wrappers |
| 4 | Genuine parallelism in MtEph BSTs: `in_order`, `filter`, `reduce`, `from_sorted_slice` use `ParaPair!` |
| 5 | Comprehensive test suite: 24 test files, ~568 tests total, all source modules covered |

### Weaknesses

| # | Weakness | Severity |
|---|----------|:--------:|
| 1 | Asymptotically wrong split/join/delete — O(n) instead of O(lg n) | High |
| 2 | Splay trees don't splay — plain unbalanced BST insert | High |
| 3 | RB color invariant not verified — BalBinTree lacks color field | Medium |
| 4 | AVL insert doesn't preserve balance — 9 assumes in rebalance | Medium |
| 5 | 13 avoidable holes in MtEph files using std::sync::RwLock instead of vstd::rwlock | Medium |
| 6 | No in-order sorted spec equivalence with `tree_is_bst` | Low |
| 7 | AVLTreeSeq files are misplaced (sequence ops, not BSTs) | Low |
| 8 | 9 bare_impl errors across AVLTreeSeq*, BSTSet* files | Low |

### Review TODOs

| # | Priority | Action | Files Affected |
|---|:--------:|--------|---------------|
| 1 | High | Implement proper O(lg n) split/join/delete (or wire to Chap38 parametric BST) | 5 BSTSet*MtEph files |
| 2 | High | Fix Splay implementations to actually perform splay rotations | BSTSplayStEph.rs, BSTSplayMtEph.rs |
| 3 | High | Migrate BSTAVLMtEph and BSTBBAlphaMtEph from std::sync::RwLock to vstd::rwlock | BSTAVLMtEph.rs, BSTBBAlphaMtEph.rs |
| 4 | Medium | Close 9 assumes in BSTAVLStEph rebalance — prove rotation height lemmas | BSTAVLStEph.rs |
| 5 | Medium | Add color field to BalBinTree or create RBBalBinTree for RB invariant verification | Chap23, BSTRBStEph.rs |
| 6 | Medium | Formalize in-order sorted spec | BSTPlainStEph.rs |
| 7 | Medium | Add `ensures` to `min_node`/`max_node` | 7 verusified files |
| 8 | Low | Move AVLTreeSeq files to Chap18/Chap19 | 4 AVLTreeSeq* files |
| 9 | Low | Fix bare_impl errors | AVLTreeSeq*, BSTSet* files |
| 10 | Low | Create PTTs for verusified BST files | New files |
