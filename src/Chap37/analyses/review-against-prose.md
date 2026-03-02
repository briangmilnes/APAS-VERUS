<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 37 — Introduction to Binary Search Trees: Review Against Prose

**Date:** 2026-03-02 (refresh; prior: 2026-02-28, initial: 2026-02-19)
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap37.txt`
**Source directory:** `src/Chap37/` (19 files)
**Test directory:** `tests/Chap37/` (24 files)
**PTT directory:** none (0 files)
**Verification status:** All 19 files inside `verus!`; 14 clean, 5 holed; 48 holes (11 assume, 37 external_body)

## Phase 1: Inventory

Module summary from `veracity-review-module-fn-impls -d src/Chap37` (424 entries across 19 files).

| # | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | AVLTreeSeq | 20 | 23 | 0 | 13 | 35 | 1 | 31 | 4 | 1 |
| 2 | AVLTreeSeqMtPer | 11 | 14 | 0 | 13 | 25 | 2 | 11 | 12 | 4 |
| 3 | AVLTreeSeqStEph | 18 | 21 | 0 | 12 | 30 | 3 | 17 | 13 | 3 |
| 4 | AVLTreeSeqStPer | 13 | 16 | 0 | 14 | 27 | 3 | 11 | 14 | 5 |
| 5 | BSTAVLMtEph | 0 | 0 | 6 | 8 | 14 | 0 | 6 | 0 | 8 |
| 6 | BSTAVLStEph | 0 | 0 | 0 | 17 | 17 | 0 | 15 | 0 | 2 |
| 7 | BSTBBAlphaMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 0 | 8 |
| 8 | BSTBBAlphaStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 9 | BSTPlainMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 0 | 8 |
| 10 | BSTPlainStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 11 | BSTRBMtEph | 14 | 16 | 0 | 20 | 1 | 35 | 0 | 1 | 35 |
| 12 | BSTRBStEph | 0 | 0 | 0 | 15 | 15 | 0 | 13 | 0 | 2 |
| 13 | BSTSetAVLMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 14 | BSTSetBBAlphaMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 15 | BSTSetPlainMtEph | 20 | 20 | 0 | 3 | 23 | 0 | 0 | 0 | 23 |
| 16 | BSTSetRBMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 0 | 0 | 22 |
| 17 | BSTSetSplayMtEph | 20 | 20 | 0 | 2 | 22 | 0 | 0 | 0 | 22 |
| 18 | BSTSplayMtEph | 14 | 16 | 0 | 17 | 1 | 32 | 0 | 1 | 32 |
| 19 | BSTSplayStEph | 11 | 12 | 0 | 12 | 24 | 0 | 3 | 1 | 20 |

**Key:** 8 verusified files (rows 5-10, 12, 19) total 116 V! functions; 70 with specs (Unk), 1 with holes. BSTSplayStEph newly partially verusified (24 V! functions, 3 with specs, 1 holed). 11 plain Rust files total 308 functions, mostly NoSpec.

## Phase 2: Prose Inventory

### Definitions

| # | Definition | Description | Implemented |
|---|-----------|-------------|-------------|
| 1 | Def 37.1 — Full Binary Tree | Recursive type: `Leaf | Node(tree x a x tree)` | Yes — `BalBinTree<T>` in Chap23 (verusified files); `Option<Box<Node<T>>>` in plain Rust |
| 2 | Def 37.2 — In-Order Traversal | `inOrder(L) ++ <k> ++ inOrder(R)` | Yes — `in_order_collect` / `in_order_parallel` in MtEph files; sequential in StEph |
| 3 | Def 37.3 — Binary Search Tree | Full binary tree where `inOrder(T)` is sorted by `<` | Yes — `tree_is_bst` spec fn in BSTPlainStEph, imported by all verusified files |
| 4 | Def 37.5 — Perfectly Balanced BST | Height exactly ceil(lg(n+1)) | Not formalized — no spec fn for perfect balance |
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
| 2 | empty | Th(1) | Th(1) | |
| 3 | singleton | Th(1) | Th(1) | |
| 4 | size | Th(1) | Th(1) | Cached at root in MtEph files |
| 5 | insert | O(lg n) | O(lg n) | For balanced BSTs |
| 6 | delete | O(lg n) | O(lg n) | For balanced BSTs |
| 7 | union / intersection / difference | O(m . lg(n/m)) | O(lg^2 n) | Chap 38 parametric |
| 8 | split | O(lg n) | O(lg n) | |
| 9 | joinPair / joinM | O(lg n) | O(lg n) | |
| 10 | filter | Th(n) | Th(lg n) | Parallel |
| 11 | reduce | Th(n) | Th(lg n) | Parallel |

### Balancing Schemes Discussed

| # | Scheme | Balance Guarantee | Implemented | Verusified | Balance Invariant Spec |
|---|--------|-------------------|-------------|------------|----------------------|
| 1 | AVL trees | h(T) = O(lg n) worst-case | Yes — `BSTAVLStEph`, `BSTAVLMtEph` | Yes | `avl_balanced` spec fn |
| 2 | Red-Black trees | h(T) = O(lg n) worst-case | Yes — `BSTRBStEph`, `BSTRBMtEph` | StEph only | No color spec (BalBinTree lacks color field) |
| 3 | Weight-balanced (BB[a]) | h(T) = O(lg n) worst-case | Yes — `BSTBBAlphaStEph`, `BSTBBAlphaMtEph` | Yes | `weight_balanced` spec fn |
| 4 | Treaps | h(T) = O(lg n) w.h.p. | No — deferred to Chap 38/39 | N/A | N/A |
| 5 | Splay trees | O(lg n) amortized | Yes — `BSTSplayStEph`, `BSTSplayMtEph` with full bottom-up splay (zig/zig-zig/zig-zag) | Partially (StEph in verus!, 2 holes) | N/A (amortized, no per-tree invariant) |

### Theorems / Properties

| # | Property | Proved in Code | Notes |
|---|----------|:--------------:|-------|
| 1 | BST property — in-order is sorted | Partial | `tree_is_bst` defined but equivalence with sorted in-order not formalized |
| 2 | Find visits at most h(T) nodes | No | Follows from structure but no formal cost proof |
| 3 | Near-balance implies h = O(lg n) | No | Balance specs exist but height bound not proven |
| 4 | Insert preserves BST property | Yes | All 7 verusified files prove `tree_is_bst(result)` |
| 5 | Insert preserves containment | Yes | `tree_contains(result, x) <==> (tree_contains(old, x) || x == value)` |
| 6 | Find correctness | Yes | `ensures result == tree_contains(node, target)` in 7 verusified files |
| 7 | Rotation preserves BST + containment | Yes | AVL, RB, BB[a] files prove rotations preserve BST and containment |
| 8 | Insert preserves AVL balance | Yes | `insert_node` in BSTAVLStEph ensures `tree_is_avl(result)` (BST + AVL balanced) |

## Phase 3: Algorithmic Analysis

### 3a. Verusification Status by Variant Family

| # | Family | File | verus! | Spec fns | Proof fns | Exec fns verified | Balance spec |
|---|--------|------|:------:|:--------:|:---------:|:-----------------:|:------------:|
| 1 | Plain | BSTPlainStEph.rs | Yes | `tree_contains`, `tree_is_bst` | — | insert, contains, find, new, size, is_empty, height | N/A (unbalanced) |
| 2 | Plain | BSTPlainMtEph.rs | Yes | (imports from StEph) | — | insert, contains, find, new, size, height, is_empty | N/A |
| 3 | AVL | BSTAVLStEph.rs | Yes | `avl_balanced`, `tree_is_avl` | `lemma_bst_deep`, `lemma_max_plus_one` | rotate_right, rotate_left, rebalance, insert, contains, find, new, size, is_empty, height | Yes — `insert_node` ensures `tree_is_avl` |
| 4 | AVL | BSTAVLMtEph.rs | Yes | (imports from StEph) | `lemma_bst_deep` | rotate_right, rotate_left, insert, contains, find, min, max + lock ops | Yes (imported) |
| 5 | RB | BSTRBStEph.rs | Yes | (uses AVL's `avl_balanced` import, unused) | `lemma_bst_deep` | rotate_right, rotate_left, insert, contains, find, new, size, is_empty, height | No (no color field) |
| 6 | RB | BSTRBMtEph.rs | **No** | — | — | — | Yes (exec RB rebalancing) |
| 7 | BB[a] | BSTBBAlphaStEph.rs | Yes | `weight_balanced`, `tree_is_bb` | — | insert, contains, find, new, size, is_empty, height | Yes |
| 8 | BB[a] | BSTBBAlphaMtEph.rs | Yes | (imports from StEph) | — | insert, contains, find, min, max + lock ops | Yes (imported) |
| 9 | Splay | BSTSplayStEph.rs | **Partial** | `spec_size_link`, `spec_height_link` | — | size_link, height_link, update, splay, bst_insert, insert_link, find_link, min_link, max_link, in_order_collect, pre_order_collect | N/A (amortized) |
| 10 | Splay | BSTSplayMtEph.rs | **No** | — | — | — | N/A |
| 11-15 | BSTSet* (5 files) | **No** | — | — | — | N/A |
| 16-19 | AVLTreeSeq* (4 files) | **No** | — | — | — | N/A |

### 3b. Implementation Fidelity — Algorithm 37.4 (Searching a BST)

All BST files implement `find` / `find_link` / `find_node` matching the prose exactly: compare key with root, recurse left if `<`, recurse right if `>`, return if `=`. Work O(h(T)) as stated.

Verusified files (Plain, AVL, RB, BB[a] StEph/MtEph) prove `ensures result.is_some() == tree_contains(node, target)` with an antisymmetric argument to rule out the wrong branch.

### 3c. Implementation Fidelity — Data Type 37.7 (BST ADT)

| # | ADT Operation | APAS Cost | Verusified? | BSTSet Cost | Notes |
|---|---------------|-----------|:-----------:|-------------|-------|
| 1 | empty | Th(1) | Yes | Th(1) | |
| 2 | singleton | Th(1) | — | Th(1) | |
| 3 | size | Th(1) | Yes | Th(1) cached | |
| 4 | find | O(h(T)) | Yes (proved) | O(h(T)) | |
| 5 | delete | O(lg n) | — | **O(n) rebuild** | Linear scan + rebuild |
| 6 | insert | O(lg n) | Yes (proved) | O(h(T)) | |
| 7 | union | O(m.lg(n/m)) | — | **O(n^2)** | Parallel structure (ParaPair!) in Plain/RB/Splay, but O(n) split/join bottleneck |
| 8 | intersection | O(m.lg(n/m)) | — | **O(n^2)** | Same — parallel recursion with O(n) primitives |
| 9 | difference | O(m.lg(n/m)) | — | **O(n^2)** | Same |
| 10 | split | O(lg n) | — | **O(n)** | Linear scan |
| 11 | joinPair | O(lg n) | — | **O(n)** | BTreeSet rebuild (RB/Splay use ParaPair! for extraction) |
| 12 | joinM | O(lg n) | — | **O(n)** | BTreeSet rebuild (RB/Splay use ParaPair! for extraction) |
| 13 | filter | Th(n), S Th(lg n) | — | Th(n) **sequential** | |
| 14 | reduce | Th(n), S Th(lg n) | — | Th(n) **sequential** | |

### 3d. Key Deviations from Prose

| # | Deviation | Severity |
|---|-----------|:--------:|
| 1 | `delete` is O(n) via linear rebuild instead of O(lg n) tree surgery | High |
| 2 | `split` is O(n) via traversal instead of O(lg n) recursive decomposition | High |
| 3 | `join_pair`/`join_m` are O(n) via BTreeSet rebuild instead of O(lg n) | High |
| 4 | RB color invariant not verified — `BalBinTree` lacks color field | Medium |
| 5 | BSTSet `filter`/`reduce` are sequential despite Mt naming | Medium |
| 6 | No in-order sorted spec equivalence | Low |

### 3e. Spec Strength — Verusified Functions

| # | Function | File | Spec Strength | Notes |
|---|----------|------|:-------------:|-------|
| 1 | `tree_contains` | BSTPlainStEph | strong | Recursive membership predicate |
| 2 | `tree_is_bst` | BSTPlainStEph | strong | BST ordering invariant |
| 3 | `avl_balanced` | BSTAVLStEph | strong | abs(h(L) - h(R)) <= 1 at every node |
| 4 | `tree_is_avl` | BSTAVLStEph | strong | Combined BST ordering + AVL balance |
| 5 | `weight_balanced` | BSTBBAlphaStEph | strong | 4.size(child) <= 3.total at every node |
| 6 | `insert_node` (AVL) | BSTAVLStEph | strong | Preserves `tree_is_avl`, containment, height bounded |
| 7 | `insert_node` (other variants) | 6 files | strong | BST preserved, containment iff old or new |
| 8 | `rebalance` (AVL) | BSTAVLStEph | strong | Preserves BST + AVL balance, height <= input, height >= input - 1 |
| 9 | `contains_node` (all variants) | 7 files | strong | result == tree_contains |
| 10 | `find_node` (all variants) | 7 files | strong | Some iff tree_contains, value matches |
| 11 | `rotate_right/left` | AVL/RB/BB[a] | strong | BST preserved, containment preserved, height + AVL balance conditional ensures |
| 12 | `bst_new/avl_new/rb_new/bb_new` | 4 StEph files | strong | BST property, empty tree |
| 13 | `size/height/is_empty` | All verusified | strong | Exact spec_size/spec_height match |
| 14 | `spec_size_link`/`spec_height_link` | BSTSplayStEph | strong | Recursive spec fns for splay tree size/height |
| 15 | `size_link`/`height_link` (Splay) | BSTSplayStEph | strong | `ensures result as nat == spec_...` |
| 16 | `update` (Splay) | BSTSplayStEph | partial | Preserves key/left/right, computes size; 1 overflow assume |
| 17 | `min_node/max_node` | 7 files | weak | Only `decreases`, no ensures |
| 18 | All plain Rust functions | 11 files | none | No specs |

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
| 6 | BSTSetRBMtEph | `union`/`intersection`/`difference` | **Yes** | `ParaPair!` recursive split-join |
| 7 | BSTSetRBMtEph | `join_pair`/`join_m` | **Partial** | `ParaPair!` for parallel value extraction, sequential BTreeSet merge |
| 8 | BSTSetSplayMtEph | `union`/`intersection`/`difference` | **Yes** | `ParaPair!` recursive split-join |
| 9 | BSTSetSplayMtEph | `join_pair`/`join_m` | **Partial** | `ParaPair!` for parallel value extraction, sequential BTreeSet merge |
| 10 | BSTSetAVLMtEph | `union`/`intersection`/`difference` | **Yes** | `ParaPair!` recursive split-join |

### BSTSet Span Audit

| # | Function | APAS Span | BSTSetBBAlpha Span | BSTSetPlain/RB/Splay/AVL Span | Root Cause |
|---|----------|-----------|--------------------|-----------------------------|------------|
| 1 | union | O(lg^2 n) | **O(n+m)** seq | **O(n)** parallel recursion, O(n) split | Parallel structure correct, but O(n) split/join primitives bottleneck |
| 2 | intersection | O(lg^2 n) | **O(n+m)** seq | **O(n)** parallel recursion, O(n) split | Same |
| 3 | difference | O(lg^2 n) | **O(n+m)** seq | **O(n)** parallel recursion, O(n) split | Same |
| 4 | filter | Th(lg n) | **Th(n)** seq | **Th(n)** seq | Sequential `iter().filter_map()` in all BSTSet files |
| 5 | reduce | Th(lg n) | **Th(n)** seq | **Th(n)** seq | Sequential `fold()` in all BSTSet files |
| 6 | delete | O(lg n) | **O(n)** seq | **O(n)** seq | Linear scan + rebuild in all BSTSet files |

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
| 5 | RB color invariant | Missing | `BalBinTree` has no color field |
| 6 | In-order traversal spec function | Missing | `tree_is_bst` via containment, not sorted in-order |
| 7 | Perfect balance spec (Def 37.5) | Missing | No spec fn for `height == ceil(lg(n+1))` |
| 8 | Treaps | Not in Chap37 | Deferred to Chap 38/39 |

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
| 8-19 | All plain Rust files | No | N/A — TOC standard applies to verusified files |

### In/Out Table (verusified files)

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BSTPlainStEph.rs | - | - | - | - | - | - | - | out | - |
| 2 | BSTPlainMtEph.rs | - | - | - | - | - | - | - | out | - |
| 3 | BSTAVLStEph.rs | - | - | - | - | - | - | - | out | - |
| 4 | BSTAVLMtEph.rs | - | - | - | - | - | - | - | out | - |
| 5 | BSTRBStEph.rs | - | - | - | - | - | - | - | out | - |
| 6 | BSTBBAlphaStEph.rs | - | - | - | - | - | - | - | out | - |
| 7 | BSTBBAlphaMtEph.rs | - | - | - | - | - | - | - | out | - |
| 8 | BSTSplayStEph.rs | in | - | in | - | - | out | - | - | - |

## Proof Holes Summary

From `veracity-review-proof-holes -d src/Chap37/` (2026-03-02):

```
 BSTAVLMtEph.rs — 1 clean proof fn
 BSTAVLStEph.rs — 2 clean proof fns
 BSTBBAlphaMtEph.rs — clean
 BSTBBAlphaStEph.rs — clean
 BSTPlainMtEph.rs — clean
 BSTPlainStEph.rs — clean
 BSTRBStEph.rs — 1 clean proof fn
 BSTSetAVLMtEph.rs — clean
 BSTSetBBAlphaMtEph.rs — clean
 BSTSetPlainMtEph.rs — clean
 BSTSetRBMtEph.rs — clean
 BSTSetSplayMtEph.rs — clean
 BSTRBMtEph.rs — 1 info (verus_rwlock_external_body)
 BSTSplayMtEph.rs — 1 info (verus_rwlock_external_body)

 AVLTreeSeq.rs — 5 holes (2 assume, 3 external_body), 6 eq/clone workaround errors
 AVLTreeSeqMtPer.rs — 12 holes (1 assume, 11 external_body), 1 eq/clone workaround error
 AVLTreeSeqStEph.rs — 15 holes (4 assume, 11 external_body), 1 eq/clone workaround error
 AVLTreeSeqStPer.rs — 14 holes (3 assume, 11 external_body), 1 eq/clone workaround error
 BSTSplayStEph.rs — 2 holes (1 assume, 1 external_body)

14 clean, 5 holed, 19 total
9 clean proof fns, 0 holed proof fns
48 holes total: 11 assume, 37 external_body
9 errors (eq/clone workaround), 2 info (2 rwlock)
```

### Hole Analysis

| # | File | Holes | assume | ext_body | Category | Notes |
|---|------|:-----:|:------:|:--------:|----------|-------|
| 1 | AVLTreeSeq | 5 | 2 | 3 | algorithmic | insert_at_link external_body; cached_size overflow assume; from_vec clone assume; iterator next external_body; clone external_body |
| 2 | AVLTreeSeqStEph | 15 | 4 | 11 | algorithmic | All tree ops external_body; singleton/from_vec spec_well_formed assumes |
| 3 | AVLTreeSeqStPer | 14 | 3 | 11 | algorithmic | Same pattern as StEph; subseq_copy/values_in_order/to_arrayseq assumes |
| 4 | AVLTreeSeqMtPer | 12 | 1 | 11 | algorithmic | Same tree ops external_body; values_in_order spec_well_formed assume |
| 5 | BSTSplayStEph | 2 | 1 | 1 | mixed | update overflow assume; Node::clone external_body |

**Changelog:**

**2026-03-02**: BSTSplayStEph reduced from 4 holes to 2 holes (2 eliminated since last review):
- `in_order_collect`: external_body removed — verified with `decreases *link`.
- `pre_order_collect`: external_body removed — verified with `decreases *link`.
- Splay rotations (zig, zig-zig, zig-zag) now fully implemented in both BSTSplayStEph and BSTSplayMtEph. Previously used plain BST insert without splaying.
- AVLTreeSeq.rs holes reclassified: 5 holes (was 10 in prior review due to different counting of eq/clone workarounds — 6 eq/clone assumes now errors, not holes).
- Total chapter holes: 48 (11 assume, 37 external_body). Zero style warnings.

**2026-02-28**: Proof holes refresh. BSTSplayStEph was at 4 holes. Total was 48 holes. Tool reclassification adjustments.

**2026-02-27**: BSTSplayStEph reduced from 11 holes to 4 holes (7 eliminated):
- `height_link`: 5 assumes removed via `spec_height_link` + `reveal_with_fuel`.
- `update`: external_body removed, 1 overflow assume remains.
- `bst_insert`: external_body removed — verified with `decreases old(link)`.
- `insert_link`: external_body removed — non-recursive wrapper.
- `size_link`: added `ensures result as nat == spec_size_link(link)`.

**2026-02-19**: Initial review. BSTPlainMtEph, BSTAVLMtEph, BSTBBAlphaMtEph cleaned (6 assumes eliminated).

## Spec Strength Summary

| Classification | Count |
|:--------------:|:-----:|
| strong | ~65 |
| partial | ~1 (update in BSTSplayStEph) |
| weak | ~4 (min_node/max_node) |
| none | ~354 (all plain Rust functions + unspecced splay/RB MtEph fns) |

The strong specs are concentrated in the 7 fully verusified files. BSTSplayStEph adds 3 new spec'd functions (size_link, height_link, update). All plain Rust files (11 of 19) have zero specs.

## Overall Assessment

### Architecture

The chapter has three layers:

1. **Verusified core** (7 files + 1 partial): Functional-style BST on `BalBinTree<T>`, with `tree_is_bst` spec and full insert/contains/find proofs. Uses the shared proof pattern from `BSTPlainStEph` (spec fns) imported by all variants. BSTAVLStEph additionally proves full AVL balance preservation through rebalance and insert. BSTSplayStEph is partially verusified: splay rotations are inside `verus!` with `decreases` proofs, `size_link`/`height_link` have ensures, but no BST preservation specs.

2. **Plain Rust MtEph** (3 files): Imperative `Option<Box<Node<T>>>` trees with actual RB rebalancing (BSTRBMtEph), real splay rotations (BSTSplayMtEph), parallel traversals via `ParaPair!`, and `Arc<RwLock>` concurrency.

3. **BSTSet wrappers** (5 files): Implement the full Data Type 37.7 ADT by wrapping an MtEph BST. Four of five (Plain, RB, Splay, AVL) now use parallel `ParaPair!` divide-and-conquer for `union`/`intersection`/`difference`, but still crippled by O(n) `split`/`join` primitives. BSTSetBBAlphaMtEph remains fully sequential.

### Strengths

| # | Strength |
|---|----------|
| 1 | Strong core verification: 7 files with clean insert/find proofs and `tree_is_bst` preservation |
| 2 | AVL balance fully proved: `insert_node` ensures `tree_is_avl` (BST + AVL balance), `rebalance` fully verified with no assumes |
| 3 | Verified lock pattern: BSTPlainMtEph demonstrates vstd::rwlock with verified `BstPred` predicate |
| 4 | Complete ADT coverage: All 14 operations from Data Type 37.7 implemented in BSTSet wrappers |
| 5 | Parallel set operations: 4 of 5 BSTSet files now use `ParaPair!` for union/intersection/difference |
| 6 | Genuine parallelism in MtEph BSTs: `in_order`, `filter`, `reduce`, `from_sorted_slice` use `ParaPair!` |
| 7 | Comprehensive test suite: 24 test files, ~568 tests total, all source modules covered |
| 8 | BST core is hole-free: all 7 BST StEph/MtEph files now clean (0 assumes, 0 external_body on algorithmic logic) |
| 9 | Splay rotations implemented: Both BSTSplayStEph and BSTSplayMtEph now perform full bottom-up splay with zig/zig-zig/zig-zag |

### Weaknesses

| # | Weakness | Severity |
|---|----------|:--------:|
| 1 | Asymptotically wrong split/join/delete — O(n) instead of O(lg n) | High |
| 2 | AVLTreeSeq* files: 46 holes (10 assume, 36 external_body) across 4 files — all tree mutation ops unverified; 9 eq/clone workaround errors | High |
| 3 | BSTSplayStEph: splay rotations lack BST preservation specs (no `tree_is_bst` or `tree_contains`) | Medium |
| 4 | RB color invariant not verified — BalBinTree lacks color field | Medium |
| 5 | BSTSetBBAlphaMtEph still fully sequential (no ParaPair!) | Medium |
| 6 | BSTSet `filter`/`reduce` sequential in all 5 wrappers | Medium |
| 7 | BSTSplayStEph: 2 remaining holes (1 assume overflow, 1 Node::clone external_body) | Low |
| 8 | No in-order sorted spec equivalence with `tree_is_bst` | Low |
| 9 | AVLTreeSeq files are misplaced (sequence ops, not BSTs) | Low |
| 10 | `min_node`/`max_node` specs are weak (decreases only, no ensures) across 7 files | Low |

### Proposed Fixes

| # | Sev | Action | Files | Est Difficulty |
|---|:---:|--------|-------|:--------------:|
| 1 | High | Prove AVLTreeSeq rotate/rebalance/insert_at_link (remove external_body) | AVLTreeSeq.rs, AVLTreeSeqStEph.rs | Hard — recursive tree mutation through Box, similar to BSTAVLStEph but on sequence-indexed trees |
| 2 | High | Prove AVLTreeSeqStPer/MtPer tree ops (remove external_body on Arc-based ops) | AVLTreeSeqStPer.rs, AVLTreeSeqMtPer.rs | Hard — Arc ownership + persistent tree structure |
| 3 | High | Implement proper O(lg n) split/join/delete or wire to Chap38 parametric BST | 5 BSTSet*MtEph files | Hard — fundamental algorithm change; Chap38 dependency |
| 4 | Med | Add `tree_is_bst` / `tree_contains` specs to BSTSplayStEph splay/bst_insert/insert_link | BSTSplayStEph.rs | Medium — splay uses `Option<Box<Node>>` not `BalBinTree`; need new spec fns or bridge |
| 5 | Med | Eliminate `update` overflow assume in BSTSplayStEph | BSTSplayStEph.rs | Easy — add `requires spec_size_link < usize::MAX - 1` to callers, propagate up |
| 6 | Med | Verify Node::clone in BSTSplayStEph (remove external_body) | BSTSplayStEph.rs | Easy — standard clone pattern with `ensures result == *self` |
| 7 | Med | Add color field to BalBinTree or create RBBalBinTree for RB invariant verification | Chap23, BSTRBStEph.rs | Medium — type change affects all BST files |
| 8 | Med | Add `ParaPair!` to BSTSetBBAlphaMtEph union/intersection/difference | BSTSetBBAlphaMtEph.rs | Easy — follow pattern from BSTSetPlainMtEph |
| 9 | Med | Add `ensures` to `min_node`/`max_node` across all verusified files | 7 files | Easy — `ensures result.is_some() ==> tree_contains(..., result.unwrap())` |
| 10 | Low | Formalize in-order sorted spec equivalence with `tree_is_bst` | BSTPlainStEph.rs | Medium — need `Seq::is_sorted` and a proof of equivalence |
| 11 | Low | Move AVLTreeSeq files to Chap18/Chap19 | 4 AVLTreeSeq* files + lib.rs | Easy — rename, update lib.rs, fix imports |
| 12 | Low | Create PTTs for verusified BST files | New files | Easy |
