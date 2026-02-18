<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 37 — Review Against Prose

- **Date**: 2026-02-13
- **Reviewer**: Claude-Opus-4.6
- **Prose source**: `prompts/Chap37.txt`
- **Source directory**: `src/Chap37/` (19 files)
- **Test directory**: `tests/Chap37/` (24 files)
- **PTT directory**: `rust_verify_test/tests/Chap37/` (0 files)
- **Verification status**: None — all files are plain Rust, no `verus!` blocks

---

## Phase 1: Inventory (tool-generated)

492 functions extracted across 19 files by `veracity-review-module-fn-impls`.

### Module Summary

| # | Module | File | Trait Fns | Impl Fns | Internal Fns |
|---|--------|------|-----------|----------|-------------|
| 1 | AVLTreeSeq | AVLTreeSeq.rs | ~18 | ~18 | ~12 |
| 2 | AVLTreeSeqStEph | AVLTreeSeqStEph.rs | ~15 | ~15 | ~10 |
| 3 | AVLTreeSeqStPer | AVLTreeSeqStPer.rs | ~14 | ~14 | ~8 |
| 4 | AVLTreeSeqMtPer | AVLTreeSeqMtPer.rs | ~12 | ~12 | ~8 |
| 5 | BSTPlainStEph | BSTPlainStEph.rs | ~10 | ~10 | ~5 |
| 6 | BSTPlainMtEph | BSTPlainMtEph.rs | ~10 | ~10 | ~3 |
| 7 | BSTAVLStEph | BSTAVLStEph.rs | ~11 | ~11 | ~13 |
| 8 | BSTAVLMtEph | BSTAVLMtEph.rs | ~14 | ~14 | ~18 |
| 9 | BSTRBStEph | BSTRBStEph.rs | ~11 | ~11 | ~14 |
| 10 | BSTRBMtEph | BSTRBMtEph.rs | ~14 | ~14 | ~18 |
| 11 | BSTBBAlphaStEph | BSTBBAlphaStEph.rs | ~11 | ~11 | ~12 |
| 12 | BSTBBAlphaMtEph | BSTBBAlphaMtEph.rs | ~11 | ~11 | ~12 |
| 13 | BSTSplayStEph | BSTSplayStEph.rs | ~11 | ~11 | ~8 |
| 14 | BSTSplayMtEph | BSTSplayMtEph.rs | ~14 | ~14 | ~15 |
| 15 | BSTSetPlainMtEph | BSTSetPlainMtEph.rs | ~20 | ~20 | ~3 |
| 16 | BSTSetAVLMtEph | BSTSetAVLMtEph.rs | ~20 | ~20 | ~3 |
| 17 | BSTSetRBMtEph | BSTSetRBMtEph.rs | ~20 | ~20 | ~3 |
| 18 | BSTSetBBAlphaMtEph | BSTSetBBAlphaMtEph.rs | ~20 | ~20 | ~3 |
| 19 | BSTSetSplayMtEph | BSTSetSplayMtEph.rs | ~20 | ~20 | ~3 |

---

## Phase 2: Prose Inventory

### Definitions

| # | Definition | Description |
|---|-----------|-------------|
| 1 | Def 37.1 — Full Binary Tree | Recursive type: `Leaf \| Node(tree × α × tree)` |
| 2 | Def 37.2 — In-Order Traversal | `inOrder T = case T of Leaf ⇒ ⟨⟩ \| Node(L,k,R) ⇒ inOrder(L) ++ ⟨k⟩ ++ inOrder(R)` |
| 3 | Def 37.3 — Binary Search Tree (BST) | Full binary tree where `inOrder(T)` is sorted by `<` |
| 4 | Def 37.5 — Perfectly Balanced BST | Height exactly ⌈lg(n+1)⌉ |
| 5 | Def 37.6 — Nearly Balanced BST | Height O(lg n) for all trees with n elements satisfying balancing invariants |

### Algorithms

| # | Algorithm | Description |
|---|-----------|-------------|
| 1 | Algorithm 37.4 — Searching a BST | `find T k`: recursive search comparing key with root |

### Data Types / ADT

| # | ADT | Functions |
|---|-----|----------|
| 1 | Data Type 37.7 — BST ADT | `empty`, `singleton`, `size`, `find`, `delete`, `insert`, `union`, `intersection`, `difference`, `split`, `joinPair`, `joinM`, `filter`, `reduce` |

### Cost Specs

| # | Operation | APAS Cost |
|---|-----------|-----------|
| 1 | find | Work O(h(T)) |
| 2 | empty | Work Θ(1) |
| 3 | singleton | Work Θ(1) |
| 4 | size | Work Θ(1) |
| 5 | insert / delete | Work O(lg n) for balanced BSTs |
| 6 | union / intersection / difference | Work O(m · lg(n/m)) (Chap 38 parametric) |
| 7 | split | Work O(lg n) |
| 8 | joinPair / joinM | Work O(lg n) |
| 9 | filter / reduce | Work Θ(n), Span Θ(lg n) |

### Balancing Schemes Discussed

| # | Scheme | Balance Guarantee | Implemented |
|---|--------|------------------|-------------|
| 1 | AVL trees | h(T) = O(lg n) worst-case | Yes — `BSTAVLStEph`, `BSTAVLMtEph` |
| 2 | Red-Black trees | h(T) = O(lg n) worst-case | Yes — `BSTRBStEph`, `BSTRBMtEph` |
| 3 | Weight-balanced (BB[α]) | h(T) = O(lg n) worst-case | Yes — `BSTBBAlphaStEph`, `BSTBBAlphaMtEph` |
| 4 | Treaps | h(T) = O(lg n) w.h.p. | No — deferred to Chap38 |
| 5 | Splay trees | O(lg n) amortized | Yes — `BSTSplayStEph`, `BSTSplayMtEph` |

### Theorems / Properties

| # | Property | Notes |
|---|----------|-------|
| 1 | BST property | In-order traversal is sorted |
| 2 | Find visits at most h(T) nodes | Follows from BST property |
| 3 | Near-balance implies h = O(lg n) | Def 37.6 |

---

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations added to internal helper functions in `BSTAVLStEph.rs`, `BSTPlainStEph.rs`, and `BSTRBStEph.rs`. The remaining 16 files have cost annotations on trait-level declarations (from `claude-4-sonet` era) but still need annotations on internal helper functions.

**Annotation status by file:**

| # | File | Trait decls annotated | Internal fns annotated |
|---|------|:--------------------:|:---------------------:|
| 1 | AVLTreeSeq.rs | ✅ (APAS) | ❌ |
| 2 | AVLTreeSeqStEph.rs | ✅ (APAS) | ❌ |
| 3 | AVLTreeSeqStPer.rs | ✅ (APAS) | ❌ |
| 4 | AVLTreeSeqMtPer.rs | ✅ (APAS) | ❌ |
| 5 | BSTPlainStEph.rs | ✅ (claude-4-sonet) | ✅ (Claude-Opus-4.6) |
| 6 | BSTPlainMtEph.rs | ✅ (claude-4-sonet) | ❌ |
| 7 | BSTAVLStEph.rs | ✅ (claude-4-sonet) | ✅ (Claude-Opus-4.6) |
| 8 | BSTAVLMtEph.rs | ✅ (claude-4-sonet) | ❌ |
| 9 | BSTRBStEph.rs | ✅ (claude-4-sonet) | ✅ (Claude-Opus-4.6) |
| 10 | BSTRBMtEph.rs | ✅ (claude-4-sonet) | ❌ |
| 11 | BSTBBAlphaStEph.rs | ✅ (claude-4-sonet) | ❌ |
| 12 | BSTBBAlphaMtEph.rs | ✅ (claude-4-sonet) | ❌ |
| 13 | BSTSplayStEph.rs | ✅ (claude-4-sonet) | ❌ |
| 14 | BSTSplayMtEph.rs | ✅ (claude-4-sonet) | ❌ |
| 15 | BSTSetPlainMtEph.rs | ✅ (claude-4-sonet) | ❌ |
| 16 | BSTSetAVLMtEph.rs | ✅ (claude-4-sonet) | ❌ |
| 17 | BSTSetRBMtEph.rs | ✅ (claude-4-sonet) | ❌ |
| 18 | BSTSetBBAlphaMtEph.rs | ✅ (claude-4-sonet) | ❌ |
| 19 | BSTSetSplayMtEph.rs | ✅ (claude-4-sonet) | ❌ |

### 3b. Implementation Fidelity

**Algorithm 37.4 (Searching a BST):**
All BST files implement `find` / `find_link` / `find_node` matching the prose exactly: compare key with root, recurse left if `<`, recurse right if `>`, return if `=`. Work O(h(T)) as stated.

**Data Type 37.7 (BST ADT):**

| # | ADT Operation | StEph Implementation | MtEph Implementation | BSTSet Implementation | Notes |
|---|---------------|---------------------|---------------------|----------------------|-------|
| 1 | empty | ✅ `new()` | ✅ `new()` | ✅ `empty()` | |
| 2 | singleton | — | — | ✅ `singleton()` | Not in base BST trait |
| 3 | size | ✅ `size()` | ✅ `size()` | ✅ `size()` | |
| 4 | find | ✅ `find()` | ✅ `find()` | ✅ `find()` | |
| 5 | delete | — | — | ✅ `delete()` | **O(n) rebuild** — not O(lg n) |
| 6 | insert | ✅ `insert()` | ✅ `insert()` | ✅ `insert()` | |
| 7 | union | — | — | ✅ `union()` | Split-join parallel |
| 8 | intersection | — | — | ✅ `intersection()` | Split-join parallel |
| 9 | difference | — | — | ✅ `difference()` | Split-join parallel |
| 10 | split | — | — | ✅ `split()` | **O(n) linear scan** — not O(lg n) |
| 11 | joinPair | — | — | ✅ `join_pair()` | **O(n) BTreeSet rebuild** |
| 12 | joinM | — | — | ✅ `join_m()` | **O(n) BTreeSet rebuild** |
| 13 | filter | — | ✅ (AVL/RB/Splay Mt) | ✅ `filter()` | Parallel in MtEph, seq in BSTSet |
| 14 | reduce | — | ✅ (AVL/RB/Splay Mt) | ✅ `reduce()` | Parallel in MtEph, seq in BSTSet |

**Key deviations from prose:**

1. **`delete` is O(n)**: The BSTSet modules implement `delete` via linear scan + rebuild from Vec. The prose expects O(lg n) deletion via tree surgery (successor swap + rebalance). This is a major cost deviation.

2. **`split` is O(n)**: The BSTSet modules implement `split` via `in_order()` traversal (O(n)) followed by partition into two Vecs. The prose expects O(lg n) split via recursive tree decomposition. Since `union`/`intersection`/`difference` call `split` recursively, this makes those operations O(n²) instead of O(m lg(n/m)).

3. **`join_pair` / `join_m` are O(n)**: These collect all values into a `BTreeSet` and rebuild. The prose expects O(lg n) via balanced tree join. Combined with the O(n) split, the set operations are much more expensive than APAS specifies.

4. **`filter`/`reduce` in BSTSet are sequential**: Despite having `FnMut + Send` bound, the BSTSet implementations use sequential `iter().filter_map()` and `fold()`. The MtEph BST modules (AVL, RB, Splay) do have genuine parallel filter/reduce implementations.

5. **No actual Splay rotations**: `BSTSplayStEph` and `BSTSplayMtEph` use a simple unbalanced BST insert (same as plain BST), not splay rotations. The naming is misleading — the `find` does not splay the accessed node to the root.

6. **AVLTreeSeq files are implicit-order sequence trees**: These are not BSTs (no key ordering). They implement array-sequence operations (nth, set, subseq) via implicit-order AVL trees where in-order position = index. They belong more to Chapter 18-19 (Array Sequences) than Chapter 37 (BSTs).

### 3c. Spec Fidelity

**No Verus specifications exist in any Chapter 37 file.** All files are plain Rust without `verus!` blocks. Therefore:
- No `requires` or `ensures` clauses
- No spec functions
- No proof functions
- No loop invariants
- No ghost state

Spec strength classification: **all 492 entries are `none`**.

---

## Phase 4: Parallelism Review

### 4a. Mt Module Classification

The Mt modules fall into two categories:

**MtEph BST modules** (BSTAVLMtEph, BSTRBMtEph, BSTBBAlphaMtEph, BSTSplayMtEph, BSTPlainMtEph):
Use `Arc<RwLock<...>>` for thread-safe interior mutability. Most basic operations (insert, find, size) are sequential under a lock. Traversal and aggregate operations have parallel implementations.

**BSTSet Mt modules** (BSTSetPlainMtEph, BSTSetAVLMtEph, BSTSetRBMtEph, BSTSetBBAlphaMtEph, BSTSetSplayMtEph):
Wrap the MtEph BSTs. `union`/`intersection`/`difference` use `ParaPair!` for recursive parallel split-join. However, the underlying `split` and `join` are O(n), negating the parallelism benefit.

### 4b. Parallelism Gap Table — MtEph BSTs

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------|
| 1 | new | Θ(1) | Θ(1) | N/A | Constructor |
| 2 | insert | O(lg n) | O(lg n) | Sequential | Lock + recursive descent |
| 3 | find | O(lg n) | O(lg n) | Sequential | Lock + recursive descent |
| 4 | contains | O(lg n) | O(lg n) | Sequential | Delegates to find |
| 5 | size | Θ(1) | Θ(1) | Sequential | Cached at root |
| 6 | height | Θ(n) | Θ(n) | Sequential | Full tree walk |
| 7 | minimum | O(lg n) | O(lg n) | Sequential | Left-spine walk |
| 8 | maximum | O(lg n) | O(lg n) | Sequential | Right-spine walk |
| 9 | in_order | Θ(lg n) | Θ(lg n) | **Parallel** | `ParaPair!` fork-join on subtrees |
| 10 | pre_order | Θ(lg n) | Θ(lg n) | **Parallel** | `ParaPair!` fork-join on subtrees |
| 11 | filter | Θ(lg n) | Θ(lg n) | **Parallel** | `ParaPair!` + `Arc<F>` |
| 12 | reduce | Θ(lg n) | Θ(lg n) | **Parallel** | `ParaPair!` + `Arc<F>` |
| 13 | from_sorted_slice | Θ(lg n) | Θ(lg n) | **Parallel** | `ParaPair!` recursive construction |

### 4c. Parallelism Gap Table — BSTSet Modules

| # | Function | APAS Span | Actual Span | Parallel? | Notes |
|---|----------|-----------|-------------|-----------|-------|
| 1 | empty | Θ(1) | Θ(1) | N/A | |
| 2 | singleton | Θ(1) | Θ(1) | N/A | |
| 3 | size | Θ(1) | Θ(1) | Sequential | |
| 4 | find | O(lg n) | O(lg n) | Sequential | |
| 5 | insert | O(lg n) | O(lg n) | Sequential | |
| 6 | delete | O(lg n) | **O(n)** | Sequential | Linear scan + rebuild |
| 7 | union | O(lg²n) | **O(n²)** | **Parallel** | `ParaPair!` but O(n) split |
| 8 | intersection | O(lg²n) | **O(n²)** | **Parallel** | `ParaPair!` but O(n) split |
| 9 | difference | O(lg²n) | **O(n²)** | **Parallel** | `ParaPair!` but O(n) split |
| 10 | split | O(lg n) | **O(n)** | Sequential | `in_order()` + partition |
| 11 | join_pair | O(lg n) | **O(n)** | Sequential | BTreeSet rebuild |
| 12 | join_m | O(lg n) | **O(n)** | Sequential | BTreeSet rebuild |
| 13 | filter | Θ(n) | Θ(n) | Sequential | `iter().filter_map()` |
| 14 | reduce | Θ(n) | Θ(n) | Sequential | `fold()` |

---

## Phase 5: Runtime Test Review

### 5a. Coverage Table

| # | Source Module | RTT File | Status |
|---|-------------|----------|--------|
| 1 | AVLTreeSeq.rs | TestAVLTreeSeq.rs | ✅ |
| 2 | AVLTreeSeqStEph.rs | TestAVLTreeSeqStEph.rs, TestAVLTreeSeqStEphChap37.rs, TestAVLTreeSeqStEph18.rs | ✅ |
| 3 | AVLTreeSeqStPer.rs | TestAVLTreeSeqStPer.rs, TestAVLTreeSeqStPer18.rs, TestAVLTreeSeqStPer19.rs | ✅ |
| 4 | AVLTreeSeqMtPer.rs | TestAVLTreeSeqMtPer.rs | ✅ |
| 5 | BSTPlainStEph.rs | TestBSTPlainStEph.rs | ✅ |
| 6 | BSTPlainMtEph.rs | TestBSTPlainMtEph.rs | ✅ |
| 7 | BSTAVLStEph.rs | TestBSTAVLStEph.rs | ✅ |
| 8 | BSTAVLMtEph.rs | TestBSTAVLMtEph.rs | ✅ |
| 9 | BSTRBStEph.rs | TestBSTRBStEph.rs | ✅ |
| 10 | BSTRBMtEph.rs | TestBSTRBMtEph.rs | ✅ |
| 11 | BSTBBAlphaStEph.rs | TestBSTBBAlphaStEph.rs | ✅ |
| 12 | BSTBBAlphaMtEph.rs | TestBSTBBAlphaMtEph.rs | ✅ |
| 13 | BSTSplayStEph.rs | TestBSTSplayStEph.rs | ✅ |
| 14 | BSTSplayMtEph.rs | TestBSTSplayMtEph.rs | ✅ |
| 15 | BSTSetPlainMtEph.rs | TestBSTSetPlainMtEph.rs | ✅ |
| 16 | BSTSetAVLMtEph.rs | TestBSTSetAVLMtEph.rs | ✅ |
| 17 | BSTSetRBMtEph.rs | TestBSTSetRBMtEph.rs | ✅ |
| 18 | BSTSetBBAlphaMtEph.rs | TestBSTSetBBAlphaMtEph.rs | ✅ |
| 19 | BSTSetSplayMtEph.rs | TestBSTSetSplayMtEph.rs | ✅ |

Additionally: `TestBSTMtEph.rs` — a cross-cutting test for all Mt BST variants.

**All 19 source modules have corresponding RTT files.** Coverage is excellent.

---

## Phase 6: Proof-Time Test (PTT) Review

**No PTTs exist** for Chapter 37 (`rust_verify_test/tests/Chap37/` does not exist).

**No PTTs are needed**: Chapter 37 contains no `verus!` blocks, no verified iterators, no verified loops, no ghost state. All code is plain Rust. PTTs are only relevant for Verus-verified code.

---

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Treaps (balancing scheme #4) | Not implemented | Deferred to Chapter 38 parametric implementation |
| 2 | Pre-order traversal definition (Def 37.2 variant) | ✅ Implemented | `pre_order()` in all BST modules |
| 3 | Efficient O(lg n) split | ❌ Missing | BSTSet split is O(n) via traversal |
| 4 | Efficient O(lg n) joinPair | ❌ Missing | BSTSet join is O(n) via BTreeSet |
| 5 | Efficient O(lg n) joinM | ❌ Missing | BSTSet joinM is O(n) via BTreeSet |
| 6 | Efficient O(lg n) delete | ❌ Missing | BSTSet delete is O(n) via rebuild |

### Code with No Prose Counterpart

| # | Code Item | Notes |
|---|----------|-------|
| 1 | AVLTreeSeq (implicit-order) | Sequence ADT, not BST — supports nth, set, subseq |
| 2 | AVLTreeSeqStEph, StPer, MtPer | Sequence variants, not search trees |
| 3 | `from_vec`, `from_sorted_slice` | Construction helpers |
| 4 | `values_in_order` | Convenience for collecting tree to Vec |
| 5 | `push_back`, `insert_value`, `delete_value` | Sequence mutation on AVLTreeSeq |
| 6 | `to_arrayseq` | Conversion to ArraySeq |
| 7 | All macros (`BSTPlainStEphLit!`, etc.) | Test convenience macros |
| 8 | Iterator implementations | Standard Rust iteration infrastructure |
| 9 | `PartialEq`, `Eq`, `Clone`, `Debug`, `Display`, `Default` | Derive/trait impls |
| 10 | `as_tree` | Accessor on BSTSet wrappers |

---

## Phase 8: Table of Contents Review

**No files in Chapter 37 have a TOC header.** This is expected — the files are plain Rust without `verus!` blocks. The TOC standard applies to verusified files.

### In/Out Table

Not applicable — no `verus!` blocks exist in any Chapter 37 file. All trait impls, derive impls, macros, and iterators are in plain Rust outside any verification boundary.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | AVLTreeSeq.rs | - | ✅ out | ✅ out | - | ✅ out | ✅ out | ✅ out | - | - |
| 2 | AVLTreeSeqStEph.rs | ✅ out | ✅ out | ✅ out | - | ✅ out | - | - | ✅ out | - |
| 3 | AVLTreeSeqStPer.rs | ✅ out | ✅ out | - | - | ✅ out | ✅ out | - | ✅ out | - |
| 4 | AVLTreeSeqMtPer.rs | ✅ out | ✅ out | ✅ out | - | ✅ out | ✅ out | - | - | IntoIterator |
| 5 | BSTPlainStEph.rs | - | - | - | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 6 | BSTPlainMtEph.rs | - | - | - | - | - | - | - | ✅ out | derive(Clone,Debug) |
| 7 | BSTAVLStEph.rs | - | - | ✅ out | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 8 | BSTAVLMtEph.rs | - | - | ✅ out | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 9 | BSTRBStEph.rs | - | - | ✅ out | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 10 | BSTRBMtEph.rs | - | - | ✅ out | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 11 | BSTBBAlphaStEph.rs | - | - | ✅ out | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 12 | BSTBBAlphaMtEph.rs | - | - | ✅ out | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 13 | BSTSplayStEph.rs | - | - | ✅ out | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 14 | BSTSplayMtEph.rs | - | - | ✅ out | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 15 | BSTSetPlainMtEph.rs | - | - | - | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 16 | BSTSetAVLMtEph.rs | - | - | - | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 17 | BSTSetRBMtEph.rs | - | - | - | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 18 | BSTSetBBAlphaMtEph.rs | - | - | - | - | - | - | - | ✅ out | derive(Debug,Clone) |
| 19 | BSTSetSplayMtEph.rs | - | - | - | - | - | - | - | ✅ out | derive(Debug,Clone) |

All placements are correct for plain Rust files (everything outside `verus!` is fine since there is no `verus!`).

---

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap37/

Modules:
   19 clean (no holes)
   0 holed (contains holes)
   19 total

Holes Found: 0 total
```

**No proof holes.** This is trivially true since there are no `verus!` blocks, no proof functions, no `assume(...)`, no `admit()`, and no `#[verifier::external_body]`.

---

## Spec Strength Summary

| Classification | Count |
|---------------|-------|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 492 |

**All 492 function entries have spec strength `none`.** No function has `requires`/`ensures` specifications because the chapter contains no Verus verification code.

---

## Overall Assessment

### Strengths

1. **Complete algorithmic coverage**: All five balancing schemes mentioned in the prose (AVL, RB, BB[α], Splay, plus unbalanced/Plain) are implemented in both StEph and MtEph variants.

2. **Full BST ADT coverage**: The BSTSet wrapper modules implement all 14 operations from Data Type 37.7 (empty, singleton, size, find, delete, insert, union, intersection, difference, split, joinPair, joinM, filter, reduce).

3. **Comprehensive test suite**: All 19 source modules have corresponding runtime tests (24 test files total). The `TestBSTMtEph.rs` cross-cutting test exercises all Mt variants uniformly.

4. **Genuine parallelism in MtEph BSTs**: The `in_order`, `pre_order`, `filter`, `reduce`, and `from_sorted_slice` operations in the MtEph BST modules use `ParaPair!` for genuine parallel tree traversal with Θ(lg n) span.

5. **Parallel set operations**: `union`, `intersection`, `difference` in the BSTSet modules use parallel recursive divide-and-conquer with `ParaPair!`.

### Weaknesses

1. **No formal verification**: This is the most significant gap. All 492 functions have spec strength `none`. Chapter 37 is entirely plain Rust — a future Verus verification pass would need to add type definitions, views, spec functions, and proofs from scratch.

2. **Asymptotically wrong `split`/`join`/`delete`**: The BSTSet modules implement `split`, `join_pair`, `join_m`, and `delete` in O(n) time instead of the O(lg n) the prose specifies. This cascades to make `union`/`intersection`/`difference` O(n²) instead of O(m lg(n/m)). The Chapter 38 parametric BST implementation (`BSTParaMtEph`, `BSTParaStEph`) fixes this with proper tree-based split/join — the BSTSet wrappers should be updated to use those.

3. **Splay trees don't actually splay**: `BSTSplayStEph` and `BSTSplayMtEph` use plain unbalanced BST insertion — they do not perform splay rotations. The amortized O(lg n) guarantee from the prose does not hold. These should either implement actual splay rotations or be renamed to `BSTUnbalanced*`.

4. **BSTSet `filter`/`reduce` are sequential**: Despite the `Send` bound, the BSTSet wrapper modules use sequential `iter().filter_map()` and `fold()`. The underlying MtEph BSTs have parallel implementations that could be exposed.

5. **AVLTreeSeq files are misplaced**: The four `AVLTreeSeq*` files implement implicit-order sequence operations (nth, set, subseq), not BST search operations. They belong in Chapters 18-19 (Array Sequences) rather than Chapter 37 (BSTs). They share no code with the BST modules.

6. **Cost annotation format inconsistency**: Older annotations use `/// claude-4-sonet:` while newer ones use `/// - Claude-Opus-4.6:`. A normalization pass would improve consistency.

7. **Duplicate imports**: Several files have duplicate `use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;` statements (e.g., BSTAVLStEph.rs line 7, BSTAVLMtEph.rs line 9, BSTRBStEph.rs line 7).

### Priority Action Items

| # | Priority | Item |
|---|----------|------|
| 1 | High | Implement proper O(lg n) `split`/`join`/`delete` in BSTSet modules (or wire to Chap38 parametric BST) |
| 2 | High | Fix Splay implementations to actually perform splay rotations, or rename to BSTUnbalanced |
| 3 | Medium | Add Verus verification (`verus!` blocks, spec functions, requires/ensures) |
| 4 | Medium | Wire BSTSet `filter`/`reduce` to parallel MtEph implementations |
| 5 | Low | Move AVLTreeSeq files to Chap18/Chap19 if they're sequence types |
| 6 | Low | Normalize cost annotation format across all files |
| 7 | Low | Remove duplicate imports |
| 8 | Low | Add cost annotations to remaining internal helper functions |
