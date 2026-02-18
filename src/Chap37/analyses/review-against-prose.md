<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 37 — Review Against Prose

- **Date**: 2026-02-17
- **Reviewer**: Claude-Opus-4.6
- **Prose source**: `prompts/Chap37.txt`
- **Source directory**: `src/Chap37/` (19 files)
- **Test directory**: `tests/Chap37/` (24 files)
- **PTT directory**: none (0 files)
- **Verification status**: Partially verusified — 7 files have `verus!` blocks with specs/proofs; 12 files remain plain Rust

---

## Phase 2: Prose Inventory

### Definitions

| # | Definition | Description | Implemented |
|---|-----------|-------------|-------------|
| 1 | Def 37.1 — Full Binary Tree | Recursive type: `Leaf \| Node(tree × α × tree)` | Yes — `BalBinTree<T>` in Chap23 (used by verusified files); `Option<Box<Node<T>>>` in plain Rust files |
| 2 | Def 37.2 — In-Order Traversal | `inOrder T = case T of Leaf ⇒ ⟨⟩ \| Node(L,k,R) ⇒ inOrder(L) ++ ⟨k⟩ ++ inOrder(R)` | Yes — `in_order_collect` / `in_order_parallel` in all MtEph files; sequential in StEph files |
| 3 | Def 37.3 — Binary Search Tree (BST) | Full binary tree where `inOrder(T)` is sorted by `<` | Yes — `tree_is_bst` spec fn in BSTPlainStEph, verified and imported by all verusified files |
| 4 | Def 37.5 — Perfectly Balanced BST | Height exactly ⌈lg(n+1)⌉ | Not formalized — no spec fn for perfect balance |
| 5 | Def 37.6 — Nearly Balanced BST | Height O(lg n) for all trees with n elements satisfying balancing invariants | Partially — `avl_balanced`, `weight_balanced` spec fns exist but O(lg n) height bound not proven as a lemma |

### Algorithms

| # | Algorithm | Description | Implemented | Verified |
|---|-----------|-------------|-------------|----------|
| 1 | Algorithm 37.4 — Searching a BST | `find T k`: recursive search comparing key with root | Yes — all BST files | Yes — `contains_node`/`find_node` in 7 verusified files with `ensures result == tree_contains(...)` |

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
| 2 | Red-Black trees | h(T) = O(lg n) worst-case | Yes — `BSTRBStEph`, `BSTRBMtEph` | StEph only (BST ordering); MtEph plain Rust | No color spec (BalBinTree lacks color field) |
| 3 | Weight-balanced (BB[α]) | h(T) = O(lg n) worst-case | Yes — `BSTBBAlphaStEph`, `BSTBBAlphaMtEph` | Yes | `weight_balanced` spec fn |
| 4 | Treaps | h(T) = O(lg n) w.h.p. | No — deferred to Chap 38/39 | N/A | N/A |
| 5 | Splay trees | O(lg n) amortized | Naming only — no splay rotations | No | N/A |

### Theorems / Properties

| # | Property | Proved in Code | Notes |
|---|----------|:-------------:|-------|
| 1 | BST property — in-order is sorted | Partial | `tree_is_bst` defined but equivalence with sorted in-order not formalized |
| 2 | Find visits at most h(T) nodes | No | Follows from structure but no formal cost proof |
| 3 | Near-balance implies h = O(lg n) | No | Balance specs exist but height bound not proven |
| 4 | Insert preserves BST property | Yes | All 7 verusified files prove `tree_is_bst(result)` |
| 5 | Insert preserves containment | Yes | All 7 verusified files prove `tree_contains(result, x) <==> (tree_contains(old, x) \|\| x == value)` |
| 6 | Find correctness | Yes | `ensures result == tree_contains(node, target)` in 7 verusified files |
| 7 | Rotation preserves BST + containment | Yes | AVL, RB, and AVL-Mt files prove rotations preserve BST and containment |

---

## Phase 3: Algorithmic Analysis

### 3a. Verusification Status by Variant Family

| # | Family | File | verus! | Spec fns | Proof fns | Exec fns verified | Balance spec |
|---|--------|------|:------:|:--------:|:---------:|:-----------------:|:------------:|
| 1 | Plain | BSTPlainStEph.rs | Yes | `tree_contains`, `tree_is_bst` | — | insert, contains, find, new, size, is_empty, height | N/A (unbalanced) |
| 2 | Plain | BSTPlainMtEph.rs | Yes | (imports from StEph) | — | insert, contains, find, new, size, height, is_empty | N/A |
| 3 | AVL | BSTAVLStEph.rs | Yes | `avl_balanced`, `tree_is_avl` | `lemma_bst_deep` | rotate_right, rotate_left, insert, contains, find, new, size, is_empty, height | Yes |
| 4 | AVL | BSTAVLMtEph.rs | Yes | (imports from StEph) | `lemma_bst_deep` | rotate_right, rotate_left, insert, contains, find, min, max + external_body lock ops | Yes (imported) |
| 5 | RB | BSTRBStEph.rs | Yes | (uses AVL's `avl_balanced` import, unused) | `lemma_bst_deep` | rotate_right, rotate_left, insert, contains, find, new, size, is_empty, height | No (no color field) |
| 6 | RB | BSTRBMtEph.rs | **No** | — | — | — | Yes (actual RB rebalancing in exec code) |
| 7 | BB[α] | BSTBBAlphaStEph.rs | Yes | `weight_balanced`, `tree_is_bb` | — | insert, contains, find, new, size, is_empty, height | Yes |
| 8 | BB[α] | BSTBBAlphaMtEph.rs | Yes | (imports from StEph) | — | insert, contains, find, min, max + external_body lock ops | Yes (imported) |
| 9 | Splay | BSTSplayStEph.rs | **No** | — | — | — | N/A (no splay rotations) |
| 10 | Splay | BSTSplayMtEph.rs | **No** | — | — | — | N/A (no splay rotations) |
| 11 | Set | BSTSetPlainMtEph.rs | **No** | — | — | — | N/A |
| 12 | Set | BSTSetAVLMtEph.rs | **No** | — | — | — | N/A |
| 13 | Set | BSTSetRBMtEph.rs | **No** | — | — | — | N/A |
| 14 | Set | BSTSetBBAlphaMtEph.rs | **No** | — | — | — | N/A |
| 15 | Set | BSTSetSplayMtEph.rs | **No** | — | — | — | N/A |
| 16 | AVLTreeSeq | AVLTreeSeq.rs | **No** | — | — | — | N/A |
| 17 | AVLTreeSeq | AVLTreeSeqStEph.rs | **No** | — | — | — | N/A |
| 18 | AVLTreeSeq | AVLTreeSeqStPer.rs | **No** | — | — | — | N/A |
| 19 | AVLTreeSeq | AVLTreeSeqMtPer.rs | **No** | — | — | — | N/A |

### 3b. Implementation Fidelity — Algorithm 37.4 (Searching a BST)

All BST files implement `find` / `find_link` / `find_node` matching the prose exactly: compare key with root, recurse left if `<`, recurse right if `>`, return if `=`. Work O(h(T)) as stated.

**Verusified files** (Plain, AVL, RB, BB[α] StEph/MtEph) prove `ensures result.is_some() == tree_contains(node, target)` with an antisymmetric argument to rule out the wrong branch. This is a clean proof of Algorithm 37.4's correctness.

### 3c. Implementation Fidelity — Data Type 37.7 (BST ADT)

| # | ADT Operation | APAS Cost | Verusified StEph | Verusified MtEph | Plain MtEph | BSTSet | Actual Cost (BSTSet) |
|---|---------------|-----------|:----------------:|:----------------:|:-----------:|:------:|---------------------|
| 1 | empty | Θ(1) | Yes | Yes | Yes | Yes | Θ(1) |
| 2 | singleton | Θ(1) | — | — | — | Yes | Θ(1) |
| 3 | size | Θ(1) | Yes | Yes | Yes | Yes | Θ(1) cached / Θ(n) BalBinTree |
| 4 | find | O(h(T)) | Yes (proved) | Yes (proved) | Yes | Yes (delegates) | O(h(T)) |
| 5 | delete | O(lg n) | — | — | — | Yes | **O(n) rebuild** |
| 6 | insert | O(lg n) | Yes (proved) | Yes (proved) | Yes | Yes (delegates) | O(h(T)) |
| 7 | union | O(m·lg(n/m)) | — | — | Yes (RB,Splay parallel) | Yes (parallel split-join) | **O(n²)** — O(n) split |
| 8 | intersection | O(m·lg(n/m)) | — | — | — | Yes (parallel split-join) | **O(n²)** — O(n) split |
| 9 | difference | O(m·lg(n/m)) | — | — | — | Yes (parallel split-join) | **O(n²)** — O(n) split |
| 10 | split | O(lg n) | — | — | — | Yes | **O(n)** linear scan |
| 11 | joinPair | O(lg n) | — | — | — | Yes | **O(n)** BTreeSet rebuild |
| 12 | joinM | O(lg n) | — | — | — | Yes | **O(n)** BTreeSet rebuild |
| 13 | filter | Θ(n), Span Θ(lg n) | — | — | Yes (RB,Splay parallel) | Yes | Θ(n) but **sequential** in BSTSet |
| 14 | reduce | Θ(n), Span Θ(lg n) | — | — | Yes (RB,Splay parallel) | Yes | Θ(n) but **sequential** in BSTSet |

### 3d. Key Deviations from Prose

1. **`delete` is O(n)**: BSTSet modules implement delete via linear scan + rebuild from Vec. The prose expects O(lg n) deletion via tree surgery (successor swap + rebalance).

2. **`split` is O(n)**: BSTSet modules implement `split` via `in_order()` traversal (O(n)) followed by partition. The prose expects O(lg n) split via recursive tree decomposition. Since `union`/`intersection`/`difference` call `split` recursively, the cost cascades to O(n²).

3. **`join_pair` / `join_m` are O(n)**: These collect all values into a `BTreeSet` and rebuild. The prose expects O(lg n) via balanced tree join.

4. **BSTSet `filter`/`reduce` are sequential**: Despite the `Send` bound, BSTSet implementations use sequential `iter().filter_map()` and `fold()`. The underlying MtEph BSTs (RB, Splay) have parallel implementations.

5. **Splay trees don't splay**: `BSTSplayStEph` and `BSTSplayMtEph` use plain unbalanced BST insert — no zig, zig-zig, or zig-zag rotations. The amortized O(lg n) guarantee does not hold.

6. **RB tree color invariant not verified**: `BSTRBStEph` verifies BST ordering and rotation containment, but the red-black color invariant is not expressed because `BalBinTree` lacks a per-node color field. The RB-specific balance guarantee is not proven.

7. **AVL insert does not prove balance preservation**: `BSTAVLStEph` defines `avl_balanced` but `avl_insert` only ensures `tree_is_bst(result)`, not `tree_is_avl(result)`. The balance-preserving rebalance after insert is not performed.

### 3e. Spec Strength — Verusified Functions

| # | Function | File | Spec Strength | Notes |
|---|----------|------|:-------------:|-------|
| 1 | `tree_contains` | BSTPlainStEph | **strong** | Recursive membership predicate |
| 2 | `tree_is_bst` | BSTPlainStEph | **strong** | BST ordering invariant |
| 3 | `avl_balanced` | BSTAVLStEph | **strong** | \|h(L) - h(R)\| ≤ 1 at every node |
| 4 | `weight_balanced` | BSTBBAlphaStEph | **strong** | 4·size(child) ≤ 3·total at every node |
| 5 | `insert_node` (all variants) | 7 files | **strong** | BST preserved, containment iff old or new |
| 6 | `contains_node` (all variants) | 7 files | **strong** | result == tree_contains |
| 7 | `find_node` (all variants) | 7 files | **strong** | Some iff tree_contains, value matches |
| 8 | `rotate_right/left` | AVL/RB/AVL-Mt | **strong** | BST preserved, containment preserved |
| 9 | `bst_new/avl_new/rb_new/bb_new` | 4 StEph files | **strong** | BST property, empty tree |
| 10 | `size/height/is_empty` | All verusified | **strong** | Exact spec_size/spec_height match |
| 11 | `min_node/max_node` | 7 files | **weak** | Only `decreases`, no ensures |
| 12 | All plain Rust functions | 12 files | **none** | No specs |

---

## Phase 4: Parallelism Review

### 4a. Classification of Verusified Mt Files

**BSTPlainMtEph** (vstd::rwlock — fully verified lock):
- All operations are **sequential** under the lock. No parallel tree operations.
- Uses `RwLock<BalBinTree<T>, BstPred<T>>` with verified predicate.

**BSTAVLMtEph, BSTBBAlphaMtEph** (std::sync::RwLock — external_body lock):
- All operations are **sequential** under the lock. No parallel tree operations.
- Lock acquisition is `external_body`; pure tree operations are verified.

### 4b. Classification of Plain Rust Mt Files

**BSTRBMtEph, BSTSplayMtEph** — genuine parallel operations:

| # | Function | Parallel? | Mechanism | Span |
|---|----------|:---------:|-----------|------|
| 1 | insert | Sequential | Lock + recursive descent | O(lg n) |
| 2 | find/contains | Sequential | Lock + recursive descent | O(lg n) |
| 3 | size/height | Sequential | Lock + cached/walk | O(1)/O(n) |
| 4 | in_order | **Parallel** | `ParaPair!` fork-join on subtrees | O(lg n) |
| 5 | pre_order | **Parallel** | `ParaPair!` fork-join on subtrees | O(lg n) |
| 6 | from_sorted_slice | **Parallel** | `ParaPair!` recursive construction | O(lg n) |
| 7 | filter | **Parallel** | `ParaPair!` + `Arc<F>` | O(lg n) |
| 8 | reduce | **Parallel** | `ParaPair!` + `Arc<F>` | O(lg n) |

### 4c. BSTSet Module Parallelism

| # | Function | APAS Span | Actual Span | Parallel? | Gap |
|---|----------|-----------|-------------|:---------:|-----|
| 1 | union | O(lg² n) | **O(n²)** | **Parallel** (ParaPair! recursion) | O(n) split/join kills cost |
| 2 | intersection | O(lg² n) | **O(n²)** | **Parallel** (ParaPair! recursion) | O(n) split/join kills cost |
| 3 | difference | O(lg² n) | **O(n²)** | **Parallel** (ParaPair! recursion) | O(n) split/join kills cost |
| 4 | split | O(lg n) | **O(n)** | Sequential | Linear traversal |
| 5 | join_pair | O(lg n) | **O(n)** | Sequential* | BTreeSet rebuild (*some variants parallel extract) |
| 6 | join_m | O(lg n) | **O(n)** | Sequential* | BTreeSet rebuild |
| 7 | filter | Θ(lg n) | **Θ(n)** | Sequential | `iter().filter_map()` |
| 8 | reduce | Θ(lg n) | **Θ(n)** | Sequential | `fold()` |
| 9 | delete | O(lg n) | **O(n)** | Sequential | Linear scan + rebuild |

### 4d. BSTSet Variant Differences

Not all BSTSet variants are identical. Two distinct patterns:

**Pattern A — Parallel split-join** (BSTSetPlain, BSTSetAVL, BSTSetSplay):
- `union`/`intersection`/`difference` use `ParaPair!` recursive divide-and-conquer.
- `join_pair`/`join_m` in AVL and Splay variants use parallel value extraction via `ParaPair!`.

**Pattern B — Sequential BTreeSet** (BSTSetBBAlpha, BSTSetRB):
- `union`/`intersection`/`difference` use sequential BTreeSet merge (BBAlpha) or parallel split-join (RB after first review correction — RB actually uses ParaPair for join but not for union/intersection/difference which use sequential BTreeSet).
- BSTSetBBAlpha: `union` = sequential BTreeSet merge; `intersection`/`difference` = sequential filter.

---

## Phase 5: Runtime Test Review

### 5a. Coverage Table

| # | Source Module | RTT File(s) | Tests |
|---|-------------|-------------|-------|
| 1 | AVLTreeSeq.rs | TestAVLTreeSeq.rs | Basic ops, insert, nth, set, subseq |
| 2 | AVLTreeSeqStEph.rs | TestAVLTreeSeqStEph.rs, TestAVLTreeSeqStEphChap37.rs, TestAVLTreeSeqStEph18.rs | Comprehensive: empty, insert, nth, set, subseq, iter |
| 3 | AVLTreeSeqStPer.rs | TestAVLTreeSeqStPer.rs, TestAVLTreeSeqStPer18.rs, TestAVLTreeSeqStPer19.rs | Persistent variants |
| 4 | AVLTreeSeqMtPer.rs | TestAVLTreeSeqMtPer.rs | Thread-safe persistent |
| 5 | BSTPlainStEph.rs | TestBSTPlainStEph.rs | Insert, find, contains, bounds, in-order |
| 6 | BSTPlainMtEph.rs | TestBSTPlainMtEph.rs | Lock-based ops |
| 7 | BSTAVLStEph.rs | TestBSTAVLStEph.rs | Insert, find, bounds, duplicate, in-order, pre-order |
| 8 | BSTAVLMtEph.rs | TestBSTAVLMtEph.rs | Lock-based ops |
| 9 | BSTRBStEph.rs | TestBSTRBStEph.rs | Insert, find, bounds, RB height |
| 10 | BSTRBMtEph.rs | TestBSTRBMtEph.rs | Lock-based ops, parallel traversal |
| 11 | BSTBBAlphaStEph.rs | TestBSTBBAlphaStEph.rs | Insert, find, bounds |
| 12 | BSTBBAlphaMtEph.rs | TestBSTBBAlphaMtEph.rs | Lock-based ops |
| 13 | BSTSplayStEph.rs | TestBSTSplayStEph.rs | Insert, find (unbalanced) |
| 14 | BSTSplayMtEph.rs | TestBSTSplayMtEph.rs | Lock-based ops, parallel traversal |
| 15 | BSTSetPlainMtEph.rs | TestBSTSetPlainMtEph.rs | Set ops: union, intersection, difference, split, join |
| 16 | BSTSetAVLMtEph.rs | TestBSTSetAVLMtEph.rs | Set ops |
| 17 | BSTSetRBMtEph.rs | TestBSTSetRBMtEph.rs | Set ops |
| 18 | BSTSetBBAlphaMtEph.rs | TestBSTSetBBAlphaMtEph.rs | Set ops |
| 19 | BSTSetSplayMtEph.rs | TestBSTSetSplayMtEph.rs | Set ops |

**Cross-cutting test**: `TestBSTMtEph.rs` exercises all 5 Mt BST variants uniformly (Plain, AVL, RB, BBAlpha, Splay, plus Treap from Chap39).

### 5b. Test Quality

- **Coverage**: All 19 source modules have RTT files. Excellent.
- **Depth**: Tests cover basic ops (insert, find, contains, min, max, size), ordering (in_order), and set operations (union, intersection, difference, split, join, filter, reduce).
- **Missing**: No stress tests for thread safety (concurrent insert + find), no property-based tests, no large-scale performance tests.

---

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs exist** for Chapter 37. `rust_verify_test/tests/Chap37/` does not exist.

**PTTs would be valuable** for the verusified files to test:
- `tree_is_bst` preservation across sequences of operations
- `tree_contains` completeness (all inserted values found, no spurious values)
- Balance invariant preservation (currently only spec'd, not ensured by insert)
- Rotation correctness under various tree shapes

---

## Phase 7: Gap Analysis

### 7a. Prose Items with No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Treaps | Not in Chap37 | Implemented in Chap38/39 (`BSTTreapStEph`) |
| 2 | Efficient O(lg n) split | Missing | BSTSet split is O(n) via traversal |
| 3 | Efficient O(lg n) joinPair | Missing | BSTSet join is O(n) via BTreeSet |
| 4 | Efficient O(lg n) joinM (joinMid) | Missing | BSTSet joinM is O(n) via BTreeSet |
| 5 | Efficient O(lg n) delete | Missing | BSTSet delete is O(n) via rebuild |
| 6 | Splay rotations (zig, zig-zig, zig-zag) | Missing | BSTSplay* uses plain BST insert |
| 7 | RB color invariant | Missing | `BalBinTree` has no color field; BST ordering verified but not RB-specific balance |
| 8 | AVL balance preservation on insert | Missing | `avl_insert` ensures BST but not `tree_is_avl` — no rebalancing step |
| 9 | In-order traversal spec function | Missing | `tree_is_bst` defined via containment quantifiers, not via sorted in-order sequence |
| 10 | Perfect balance spec (Def 37.5) | Missing | No spec fn for `height == ceil(lg(n+1))` |

### 7b. Code with No Direct Prose Counterpart

| # | Code Item | Notes |
|---|----------|-------|
| 1 | AVLTreeSeq, AVLTreeSeqStEph, AVLTreeSeqStPer, AVLTreeSeqMtPer | Implicit-order sequence trees, not BSTs — supports nth, set, subseq. Better fit for Chap 18-19. |
| 2 | `from_sorted_slice`, `build_balanced` | Construction helpers for parallel tree building |
| 3 | `pre_order` / `pre_order_parallel` | Traversal — defined in prose (Def 37.2) but not part of the ADT |
| 4 | `rotate_right`, `rotate_left` | Rotation primitives — implied by balancing discussion but no explicit algorithm in Chap 37 prose |
| 5 | `lemma_bst_deep` | Proof helper — decomposes BST invariant two levels deep for rotation proofs |
| 6 | `BstPred<T>` / `RwLockPredicate` | Lock invariant infrastructure for verified Mt access |
| 7 | `PartialEq`, `Eq`, `Clone`, `Debug`, `Default` derive impls | Standard Rust infrastructure |
| 8 | All macros (`BSTPlainStEphLit!`, etc.) | Test convenience macros |

---

## Phase 8: TOC Review

### 8a. TOC Standard Compliance

| # | File | Has TOC | Sections Present | Compliant |
|---|------|:-------:|-----------------|:---------:|
| 1 | BSTPlainStEph.rs | Yes | 1,2,6,9,12 | Yes |
| 2 | BSTPlainMtEph.rs | Yes | 1,2,4,8,9,12 | Yes |
| 3 | BSTAVLStEph.rs | Yes | 1,2,6,7,9,12 | Yes |
| 4 | BSTAVLMtEph.rs | Yes | 1,2,6,7,9,12 | Yes |
| 5 | BSTRBStEph.rs | Yes | 1,2,6,7,9,12 | Yes |
| 6 | BSTBBAlphaStEph.rs | Yes | 1,2,6,9,12 | Yes |
| 7 | BSTBBAlphaMtEph.rs | Yes | 1,2,6,9,12 | Yes |
| 8-19 | All plain Rust files | No | N/A | N/A — TOC standard applies to verusified files |

### 8b. In/Out Table

Files with `verus!` blocks:

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | BSTPlainStEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 2 | BSTPlainMtEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 3 | BSTAVLStEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 4 | BSTAVLMtEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 5 | BSTRBStEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 6 | BSTBBAlphaStEph.rs | - | - | - | - | - | - | - | ✅ out | - |
| 7 | BSTBBAlphaMtEph.rs | - | - | - | - | - | - | - | ✅ out | - |

Plain Rust files (all impls correctly outside verus! since there is no verus!):

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|
| 8 | BSTRBMtEph.rs | derive | - | ✅ out | - | - | derive | - | ✅ out |
| 9 | BSTSplayStEph.rs | derive | - | ✅ out | - | - | derive | - | ✅ out |
| 10 | BSTSplayMtEph.rs | derive | - | ✅ out | - | - | derive | - | ✅ out |
| 11 | BSTSetPlainMtEph.rs | derive | - | - | - | - | derive | - | ✅ out |
| 12 | BSTSetAVLMtEph.rs | derive | - | - | - | - | derive | - | ✅ out |
| 13 | BSTSetRBMtEph.rs | derive | - | - | - | - | derive | - | ✅ out |
| 14 | BSTSetBBAlphaMtEph.rs | derive | - | - | - | - | derive | - | ✅ out |
| 15 | BSTSetSplayMtEph.rs | derive | - | - | - | - | derive | - | ✅ out |
| 16 | AVLTreeSeq.rs | - | ✅ out | - | - | ✅ out | ✅ out | ✅ out | - |
| 17 | AVLTreeSeqStEph.rs | ✅ out | ✅ out | ✅ out | - | ✅ out | - | - | ✅ out |
| 18 | AVLTreeSeqStPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | - | ✅ out |
| 19 | AVLTreeSeqMtPer.rs | ✅ out | ✅ out | ✅ out | - | ✅ out | ✅ out | - | - |

---

## Proof Holes Summary

```
BSTAVLMtEph.rs: 8 holes
  1 external_type_spec  — ExRwLock<T> (std::sync::RwLock type spec)
  1 assume_spec         — RwLock::new assume specification
  6 external_body       — insert, contains, size, is_empty, height, (lock acq.)

BSTBBAlphaMtEph.rs: 5 holes
  5 external_body       — insert, contains, size, is_empty, height (lock acq.)

BSTPlainMtEph.rs: 2 holes
  2 assume              — spec_size() <= usize::MAX, spec_height() <= usize::MAX

All other files: CLEAN
```

### Hole Analysis

| # | File | Hole Type | Justification | Avoidable? |
|---|------|-----------|---------------|:----------:|
| 1 | BSTAVLMtEph | `external_type_spec` for RwLock | std::sync::RwLock is not natively supported by Verus | Yes — switch to vstd::rwlock (as BSTPlainMtEph does) |
| 2 | BSTAVLMtEph | `assume_spec` for RwLock::new | Needed because std::sync::RwLock constructor has no Verus spec | Yes — switch to vstd::rwlock |
| 3 | BSTAVLMtEph | 6 `external_body` | Lock acquire/release not verifiable with std::sync::RwLock | Yes — switch to vstd::rwlock |
| 4 | BSTBBAlphaMtEph | 5 `external_body` | Same as AVL — std::sync::RwLock | Yes — switch to vstd::rwlock |
| 5 | BSTPlainMtEph | 2 `assume` | `spec_size() <= usize::MAX` — overflow guard | Tolerable — practical assumption |

**Key observation**: BSTPlainMtEph demonstrates the correct pattern (vstd::rwlock with verified predicate, zero external_body on lock ops). BSTAVLMtEph and BSTBBAlphaMtEph should be migrated to the same pattern to eliminate 13 holes.

---

## Overall Assessment

### Architecture Summary

The chapter has three layers:

1. **Verusified core** (7 files): Functional-style BST on `BalBinTree<T>`, with `tree_is_bst` spec and full insert/contains/find proofs. Uses the shared proof pattern from `BSTPlainStEph` (spec fns) imported by all variants.

2. **Plain Rust MtEph** (4 files): Imperative `Option<Box<Node<T>>>` trees with actual RB rebalancing (BSTRBMtEph), splay-style naming but no splay rotations (BSTSplayMtEph), parallel traversals via `ParaPair!`, and `Arc<RwLock>` concurrency.

3. **BSTSet wrappers** (5 files): Implement the full Data Type 37.7 ADT by wrapping an MtEph BST. Parallel `union`/`intersection`/`difference` via `ParaPair!` split-join, but crippled by O(n) `split`/`join` primitives.

### Strengths

1. **Strong core verification**: The 7 verusified files have clean, complete proofs for insert and find. The `tree_is_bst` invariant preservation is proven across all balanced variants (AVL, BB[α]) and the unbalanced case (Plain). Rotation proofs in AVL and RB are thorough.

2. **Verified lock pattern**: BSTPlainMtEph shows how to use vstd::rwlock with a verified predicate (`BstPred`) to thread the BST invariant through lock acquire/release with zero external_body holes.

3. **Complete ADT coverage**: All 14 operations from Data Type 37.7 are implemented in the BSTSet wrappers.

4. **Genuine parallelism in MtEph BSTs**: `in_order`, `pre_order`, `filter`, `reduce`, and `from_sorted_slice` use `ParaPair!` for parallel tree traversal.

5. **Comprehensive test suite**: All 19 source modules have RTTs (24 test files total). Cross-cutting `TestBSTMtEph.rs` tests all Mt variants uniformly.

### Weaknesses

1. **Asymptotically wrong split/join/delete**: BSTSet `split`, `join_pair`, `join_m`, and `delete` are O(n) instead of O(lg n). This cascades to O(n²) for union/intersection/difference.

2. **Splay trees don't splay**: `BSTSplayStEph` and `BSTSplayMtEph` are plain unbalanced BSTs. No zig/zig-zig/zig-zag rotations. The amortized O(lg n) guarantee from the prose does not hold.

3. **RB color invariant not verified**: The `BalBinTree` type lacks a color field, so `BSTRBStEph` cannot express or verify the red-black property. Only BST ordering is verified.

4. **AVL insert doesn't preserve balance**: `avl_insert` ensures `tree_is_bst` but not `tree_is_avl`. The insert does not include a rebalancing step.

5. **No in-order sorted spec**: The `tree_is_bst` spec uses containment quantifiers (`forall|x| tree_contains(left, x) ==> x < root`) rather than proving `in_order(T)` is sorted. The equivalence stated in Def 37.3 is not formalized.

6. **13 avoidable external_body holes**: BSTAVLMtEph (8) and BSTBBAlphaMtEph (5) use std::sync::RwLock instead of vstd::rwlock, requiring external_body on all lock operations.

7. **AVLTreeSeq files are misplaced**: The 4 AVLTreeSeq* files implement implicit-order sequence operations (nth, set, subseq), not BST search. They belong in Chapters 18-19.

8. **Duplicate code**: The insert/contains/find proof bodies are copy-pasted across 7 files. A shared proof module or trait-based approach could reduce duplication.

9. **Duplicate imports**: Several files have `use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;` duplicated (BSTRBMtEph line 8-9, BSTSplayStEph line 6-7, BSTSplayMtEph line 8-9).

---

## Review TODOs

| # | Priority | Item | Files Affected |
|---|----------|------|---------------|
| 1 | High | Implement proper O(lg n) `split`/`join`/`delete` in BSTSet modules (or wire to Chap38 parametric BST) | All 5 BSTSet*MtEph files |
| 2 | High | Fix Splay implementations to actually perform splay rotations, or rename to BSTUnbalanced | BSTSplayStEph.rs, BSTSplayMtEph.rs |
| 3 | High | Migrate BSTAVLMtEph and BSTBBAlphaMtEph from std::sync::RwLock to vstd::rwlock (eliminates 13 holes) | BSTAVLMtEph.rs, BSTBBAlphaMtEph.rs |
| 4 | Medium | Add AVL rebalancing step to `avl_insert` so it preserves `tree_is_avl` (not just `tree_is_bst`) | BSTAVLStEph.rs, BSTAVLMtEph.rs |
| 5 | Medium | Add color field to `BalBinTree` (or create `RBBalBinTree`) so RB invariant can be specified and verified | Chap23, BSTRBStEph.rs |
| 6 | Medium | Formalize in-order sorted spec: `spec fn in_order_seq(tree) -> Seq<T>` and prove equivalence with `tree_is_bst` | BSTPlainStEph.rs |
| 7 | Medium | Wire BSTSet `filter`/`reduce` to parallel MtEph implementations | All 5 BSTSet*MtEph files |
| 8 | Low | Move AVLTreeSeq files to Chap18/Chap19 if they are sequence types | 4 AVLTreeSeq* files |
| 9 | Low | Add `ensures` to `min_node`/`max_node` (currently only `decreases`) | All 7 verusified files |
| 10 | Low | Remove duplicate imports | BSTRBMtEph.rs, BSTSplayStEph.rs, BSTSplayMtEph.rs |
| 11 | Low | Create PTTs for the verusified BST files | New files in rust_verify_test/tests/Chap37/ |
| 12 | Low | Extract shared insert/contains/find proofs into a common proof module to reduce copy-paste across 7 files | Structural refactor |
