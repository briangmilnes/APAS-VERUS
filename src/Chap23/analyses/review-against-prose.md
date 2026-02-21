<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 23 — Tree Sequences: Review Against Prose

**Date:** 2026-02-19
**Last mechanical audit:** 2026-02-19 — full review regeneration; proof holes log updated.
**Reviewer:** Claude-Opus-4.6

**Recent changes:** PrimTreeSeqStPer: moved `as_slice` and `into_vec` from bare `impl PrimTreeSeqStS<T>` into the trait and trait impl, eliminating one bare impl block. BalBinTreeStEph: no meaningful change (attempted to remove `external_body` from Clone but reverted — `T::clone()` has no spec in Verus).

**Source files:** `src/Chap23/BalBinTreeStEph.rs`, `src/Chap23/PrimTreeSeqStPer.rs`
**Test files:** `tests/Chap23/TestBalBinTreeStEph.rs`, `tests/Chap23/TestPrimTreeSeqStPer.rs`
**PTT files:** `rust_verify_test/tests/Chap23/ProveBalBinTreeStEph.rs`, `rust_verify_test/tests/Chap23/ProvePrimTreeSeqStPer.rs`

---

## Phase 1: Inventory

### BalBinTreeStEph.rs — 15 functions

| # | Function | Kind | V! | SpecStr | Hole | Lines |
|---|----------|------|----|---------|------|------:|
| 1 | `lemma_in_order_pre_order_permutation` | proof fn (ML) | Y | strong | — | 122–124 |
| 2 | `lemma_pre_order_post_order_permutation` | proof fn (ML) | Y | strong | — | 165–167 |
| 3 | `leaf` | Tr+IT | Y | strong | — | 215–220 |
| 4 | `node` | Tr+IT | Y | strong | — | 223–229 |
| 5 | `is_leaf` | Tr+IT | Y | strong | — | 232–233 |
| 6 | `size` | Tr+IT | Y | strong | — | 236–238 |
| 7 | `height` | Tr+IT | Y | strong | — | 241–243 |
| 8 | `in_order` | Tr+IT | Y | strong | — | 247–251 |
| 9 | `pre_order` | Tr+IT | Y | strong | — | 255–259 |
| 10 | `post_order` | Tr+IT | Y | strong | — | 263–267 |
| 11 | `next` ×3 | IT (Iterator) | Y | strong | — | 479–495 |
| 12 | `iter_in_order` | IBI | Y | strong | — | 679–685 |
| 13 | `iter_pre_order` | IBI | Y | strong | — | 691–697 |
| 14 | `iter_post_order` | IBI | Y | strong | — | 703–709 |
| 15 | `eq` ×2 | IT (PartialEq) | Y | strong | assume | 750, 771 |

### PrimTreeSeqStPer.rs — 20 functions

| # | Function | Kind | V! | SpecStr | Hole | Lines |
|---|----------|------|----|---------|------|------:|
| 16 | `iter` | IBI | Y | strong | — | 117–121 |
| 17 | `empty` | Tr+IT | Y | strong | — | 135–136 |
| 18 | `singleton` | Tr+IT | Y | strong | — | 140–143 |
| 19 | `from_vec` | Tr+IT | Y | strong | — | 147–150 |
| 20 | `length` | Tr+IT | Y | strong | — | 154–155 |
| 21 | `nth` | Tr+IT | Y | strong | — | 159–161 |
| 22 | `expose` | Tr+IT | Y | strong | — | 165–172 |
| 23 | `join` | Tr+IT | Y | strong | — | 186–190 |
| 24 | `append` | Tr+IT | Y | strong | — | 194–202 |
| 25 | `subseq` | Tr+IT | Y | strong | — | 206–214 |
| 26 | `update` | Tr+IT | Y | strong | — | 218–226 |
| 27 | `map` | Tr+IT | Y | strong | — | 230–235 |
| 28 | `tabulate` | Tr+IT | Y | strong | — | 239–245 |
| 29 | `filter` | Tr+IT | Y | strong | — | 249–261 |
| 30 | `drop` | Tr+IT | Y | strong | — | 265–273 |
| 31 | `flatten` | Tr+IT | Y | strong | — | 277–282 |
| 32 | `next` | IT (Iterator) | Y | strong | — | 702–718 |
| 33 | `eq` ×2 | IT (PartialEq) | Y | strong | assume | 804–805 |
| 34 | `as_slice` | Tr+IT | Y | strong | — | 304, 697 |
| 35 | `into_vec` | Tr+IT | Y | strong | — | 310, 699 |

---

## Phase 2: Prose Inventory

### Data Types

| # | Prose Item | Reference | Implemented |
|---|-----------|-----------|:-----------:|
| 1 | Sα = Vec-backed sequence | Data Type 23.1 | ✅ `PrimTreeSeqStS<T>` |
| 2 | Tα = Zero \| One(α) \| Two(Sα × Sα) | Data Type 23.1 | ✅ `PrimTreeSeqStTree<T>` |

### Primitive Operations (Data Type 23.1)

| # | Prose Item | Reference | Implemented |
|---|-----------|-----------|:-----------:|
| 3 | length : Sα → N | Data Type 23.1 | ✅ `length()` |
| 4 | expose : Sα → Tα | Data Type 23.1 | ✅ `expose()` |
| 5 | join : Tα → Sα | Data Type 23.1 | ✅ `join()` |

### Cost Specification 23.2

| # | Prose Item | Reference | Implemented |
|---|-----------|-----------|:-----------:|
| 6 | length: Work 1, Span 1 | Cost Spec 23.2 | ✅ Vec: Θ(1) |
| 7 | expose: Work 1, Span 1 | Cost Spec 23.2 | ⚠️ Vec: Θ(n) — clones elements |
| 8 | join(Zero/One): Work 1, Span 1 | Cost Spec 23.2 | ✅ Vec: Θ(1) |
| 9 | join(Two(L,R)): Work 1+\|r(L)−r(R)\|, Span same | Cost Spec 23.2 | ⚠️ Vec: Θ(\|L\|+\|R\|) — Vec append |

### Algorithm 23.3 (Tree Sequence Functions)

| # | Prose Item | Reference | Implemented |
|---|-----------|-----------|:-----------:|
| 10 | empty = join(Zero) | Algorithm 23.3 | ✅ `empty()` |
| 11 | singleton x = join(One(x)) | Algorithm 23.3 | ✅ `singleton()` |
| 12 | append(a,b) = join(Two(a,b)) | Algorithm 23.3 | ✅ `append()` |
| 13 | nth S i — recursive on expose | Algorithm 23.3 | ✅ `nth()` — direct Vec index |
| 14 | map f S — parallel recursive | Algorithm 23.3 | ✅ `map()` — sequential loop |
| 15 | tabulate f n — parallel recursive | Algorithm 23.3 | ✅ `tabulate()` — sequential loop |
| 16 | filter f S — parallel recursive | Algorithm 23.3 | ✅ `filter()` — sequential loop |
| 17 | drop S n — recursive on expose | Algorithm 23.3 | ✅ `drop()` — delegates to subseq |
| 18 | update S (i,v) — recursive on expose | Algorithm 23.3 | ✅ `update()` — full Vec copy |
| 19 | subseq S (a,n) = take(drop(S,a), n) | Algorithm 23.3 | ✅ `subseq()` — direct slice |
| 20 | flatten S = reduce append empty S | Algorithm 23.3 | ✅ `flatten()` — nested loops |

### Rank Properties (not implemented — theoretical discussion)

| # | Prose Item | Reference | Implemented |
|---|-----------|-----------|:-----------:|
| 21 | r(T) ∈ O(log \|T\|) | Rank condition 1 | — Not applicable to Vec-backed impl |
| 22 | Rank bounds on children | Rank condition 2 | — Not applicable to Vec-backed impl |
| 23 | r(join(Two(L,R))) ≤ max(r(L),r(R))+1 | Rank condition 3 | — Not applicable to Vec-backed impl |

### Theorems / Proofs

| # | Prose Item | Reference | Implemented |
|---|-----------|-----------|:-----------:|
| 24 | Traversal permutation properties | implicit | ✅ `lemma_in_order_pre_order_permutation`, `lemma_pre_order_post_order_permutation` |

---

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All exec functions now have paired APAS/Claude-Opus-4.6 cost annotations. Key cost deviations due to Vec-backed implementation:

| # | Function | APAS (tree-based) | Claude (Vec-backed) | Deviation |
|---|----------|--------------------|---------------------|-----------|
| 1 | `expose` | Θ(1) | Θ(n) | Vec must clone elements into two halves |
| 2 | `join(Two)` | Θ(1 + \|r(L)−r(R)\|) | Θ(\|L\|+\|R\|) | Vec append is linear |
| 3 | `nth` | Θ(log n) | Θ(1) | Vec direct index is faster |
| 4 | `map` | Work Θ(n), Span Θ(log n) | Work Θ(n), Span Θ(n) | Sequential, no parallelism |
| 5 | `tabulate` | Work Θ(n), Span Θ(log n) | Work Θ(n), Span Θ(n) | Sequential, no parallelism |
| 6 | `filter` | Work Θ(n), Span Θ(log² n) | Work Θ(n), Span Θ(n) | Sequential, no parallelism |
| 7 | `update` | Θ(log² n) | Θ(n) | Full Vec copy |
| 8 | `drop` | Θ(log² n) | Θ(n − k) | Delegates to subseq |
| 9 | `append` | Θ(1 + \|r(a)−r(b)\|) | Θ(\|a\|+\|b\|) | Sequential clone loops |

### 3b. Implementation Fidelity

**PrimTreeSeqStPer:** The implementation provides all Algorithm 23.3 operations with correct functional behavior (verified by Verus), but uses a flat Vec rather than a balanced tree as the backing store. This means:
- **Correctness**: All specs are satisfied — the implementation is functionally correct.
- **Performance model**: Does not achieve the tree-based cost bounds from the prose. The prose's Cost Spec 23.2 assumes O(log n) expose and O(1+|r(L)−r(R)|) join, which the Vec-backed implementation cannot satisfy.
- **Parallelism**: All `||` (parallel) operations in Algorithm 23.3 (map, tabulate, filter) are implemented sequentially. This is expected for a `St` (single-threaded) module.

**BalBinTreeStEph:** This module provides a recursive balanced binary tree type with constructors (leaf, node), predicates (is_leaf), metrics (size, height), and traversals (in_order, pre_order, post_order). These are not directly from the Chapter 23 prose but provide the tree infrastructure that would underlie a tree-based sequence implementation. The implementations are straightforward recursive traversals.

### 3c. Spec Fidelity

All 35 functions have **strong** specifications:
- Trait methods have complete `requires`/`ensures` contracts that fully capture input-output relationships.
- `expose` specifies the three-way case split and that `Two` parts sum to the original.
- `join` specifies reconstruction for all three variants.
- `filter` specifies multiset equality, length bounds, and that retained elements satisfy the predicate.
- `map` and `tabulate` use `f.ensures` to relate output to input through the closure.
- Iterator `next` specifies index advancement and element equality.
- Proof lemmas specify multiset permutation properties.

---

## Phase 4: Parallelism Review

No Mt (multi-threaded) modules exist for Chapter 23. All implementations are St (single-threaded). The prose describes parallel operations (map, tabulate, filter use `||`) but these are implemented sequentially in the St modules. This is architecturally consistent — parallel variants would be separate Mt modules.

---

## Phase 5: Runtime Test Review

### TestBalBinTreeStEph.rs — 20 tests

| # | Test | Coverage |
|---|------|----------|
| 1 | `inorder_and_preorder_traversals_match_definitions` | in_order, pre_order, size, height |
| 2 | `balbintree_empty_leaf_operations` | leaf, size, height, in_order, pre_order |
| 3 | `balbintree_single_node_operations` | node, size, height, in_order, pre_order |
| 4 | `balbintree_complex_structure` | node, size, height, in_order, pre_order (7-node tree) |
| 5 | `balbintree_height_calculation` | height for leaf, single, left-heavy, right-heavy |
| 6 | `balbintree_size_calculation` | size for leaf, single, 3-node |
| 7 | `balbintree_traversal_consistency` | in_order, pre_order length match size |
| 8 | `balbintree_is_leaf_check` | is_leaf for leaf, single, complex |
| 9 | `balbintree_unbalanced_left` | left-skewed tree |
| 10 | `balbintree_unbalanced_right` | right-skewed tree |
| 11 | `balbintree_large_balanced_tree` | 7-node perfect tree |
| 12 | `balbintree_only_left_children` | 4-node left-only chain |
| 13 | `balbintree_only_right_children` | 4-node right-only chain |
| 14 | `balbintree_trait_methods` | trait-qualified calls |
| 15 | `balbintree_clone` | Clone impl |
| 16 | `balbintree_debug` | Debug format |
| 17–20 | Iterator tests (inorder/preorder: collect, leaf empty, single, complex, manual next) | iter_in_order, iter_pre_order, Iterator::next |

**Missing test coverage:**
- `post_order()` traversal is tested via trait but not directly via `iter_post_order()`
- No tests for `PartialEq`/`Eq` beyond implicit `assert_eq!` usage

### TestPrimTreeSeqStPer.rs — 39 tests

| # | Test Group | Coverage |
|---|------------|----------|
| 1–6 | expose/join roundtrip tests | expose (Zero, One, Two), join (Zero, One, Two) |
| 7–11 | Constructor tests | empty, singleton, from_vec, into_vec, as_slice |
| 12–15 | Edge case tests | expose with 2 elements, odd count; join with empty seqs, mixed sizes |
| 16–19 | Trait/equality tests | clone, debug, PartialEq for struct and enum |
| 20–21 | Large sequence test | 1000-element expose/join roundtrip |
| 22–31 | Trait-qualified calls | empty, singleton, from_vec, into_vec, as_slice, length, expose, join variants |
| 32–35 | Generic function tests | generic_length, generic_expose (empty, one, two) |

**Missing test coverage:**
- `nth()` — no direct test (used implicitly in other chapters)
- `append()` — no direct test
- `subseq()` — no direct test
- `update()` — no direct test
- `map()` — no direct test
- `tabulate()` — no direct test
- `filter()` — no direct test
- `drop()` — no direct test
- `flatten()` — no direct test

Several Algorithm 23.3 operations lack runtime tests. The trait interface is well-tested for constructors and expose/join, but the derived operations (append, subseq, update, map, tabulate, filter, drop, flatten) have no runtime test coverage.

---

## Phase 6: PTT Review

### ProveBalBinTreeStEph.rs — 8 PTTs

| # | Test | Pattern |
|---|------|---------|
| 1 | `balbintree_loop_inorder` | loop-in-order: manual loop + next() |
| 2 | `balbintree_loop_preorder` | loop-pre-order: manual loop + next() |
| 3 | `balbintree_for_inorder` | for-in-order: for x in iter |
| 4 | `balbintree_for_preorder` | for-pre-order: for x in iter |
| 5 | `balbintree_inorder_spec_match` | in_order result matches spec |
| 6 | `balbintree_preorder_spec_match` | pre_order result matches spec |
| 7 | `balbintree_leaf_traversals_empty` | leaf traversals are empty |
| 8 | `balbintree_leaf_iter_exhausted` | leaf iterator immediately returns None |

Good coverage of iterator patterns. Tests both loop-based and for-based iteration for in-order and pre-order.

**Missing:** No PTT for post-order iterator.

### ProvePrimTreeSeqStPer.rs — 6 PTTs

| # | Test | Pattern |
|---|------|---------|
| 1 | `primtreeseq_loop_borrow_iter` | loop-borrow-iter: loop + a.iter() |
| 2 | `primtreeseq_loop_borrow_into` | loop-borrow-into: loop + (&a).into_iter() |
| 3 | `primtreeseq_loop_consume` | loop-consume: loop + a.into_iter() |
| 4 | `primtreeseq_for_borrow_iter` | for-borrow-iter: for x in iter: a.iter() |
| 5 | `primtreeseq_for_borrow_into` | for-borrow-into: for x in iter: (&a).into_iter() |
| 6 | `primtreeseq_for_consume` | for-consume: for x in iter: a.into_iter() |

Comprehensive iterator coverage: all six patterns (3 loop + 3 for) are tested.

---

## Phase 7: Gap Analysis

### Prose items with no implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Rank properties (conditions 1-3) | Not implemented | Theoretical discussion only; not applicable to Vec-backed impl |
| 2 | Balanced tree backing (AVL, Red-Black, Weight-balanced) | Not implemented | Prose defers to "later chapters"; Vec used instead |

### Code with no prose counterpart

| # | Code Item | Notes |
|---|----------|-------|
| 1 | `BalBinTreeStEph` module | General binary tree utility; not in Chapter 23 prose (which focuses on tree sequences) |
| 2 | `from_vec()` | Vec-specific constructor, not in prose |
| 3 | `as_slice()`, `into_vec()` | Utility methods for test support (now in trait) |
| 4 | `iter()`, iterator infrastructure | Verus-specific iteration scaffolding |
| 5 | `lemma_in_order_pre_order_permutation` | Verus-specific tree property proof |
| 6 | `lemma_pre_order_post_order_permutation` | Verus-specific tree property proof |
| 7 | `PartialEq`/`Eq`/`Clone`/`Debug` impls | Rust derive impls, not in prose |

---

## Phase 8: TOC Review

### BalBinTreeStEph.rs

TOC present at lines 4–13. Section headers present. Section ordering:

| # | Section | Present | Correct Order |
|---|---------|:-------:|:-------------:|
| 1 | module | ✅ | ✅ |
| 2 | imports | ✅ | ✅ |
| 3 | broadcast use | ✅ | ✅ |
| 4 | type definitions | ✅ | ✅ |
| 5 | spec functions | ✅ | ✅ |
| 6 | proof functions | ✅ (listed as section 6) | ✅ |
| 8 | traits | ✅ | ✅ |
| 9 | impls | ✅ | ✅ |
| 10 | iterators | ✅ | ✅ |
| 13 | derive impls outside verus! | ✅ | ✅ |

**Issue:** TOC lists "5. spec functions" but file header says "5. spec functions" and section header says "5. spec functions". The section 6 header says "6. proof functions" but the TOC omits it. The TOC jumps from 5 to 8. This is acceptable per rules (omit sections that don't apply), but section 6 (proof functions) IS present and should be listed. Also the section labeled "13. PartialEq / Eq impls" should be "13. derive impls outside verus!" but it mixes PartialEq (inside verus!) with Clone (outside). Section 14 is labeled "14. Clone impls outside verus!" which is non-standard.

**In/Out Table:**

| # | Chapter | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|---------|------|:-----:|:---:|:----:|:----:|:-----:|:-----:|:----:|:-----:|-------|
| 1 | Chap23 | BalBinTreeStEph | ❌ out | ✅ in | - | - | ✅ in | ✅ out | - | - | — |

**Note:** Clone for `BalBinTree<T>` and `BalBinNode<T>` is outside `verus!` (lines 764–777). Per project rules, Clone should be inside `verus!` with specs.

### PrimTreeSeqStPer.rs

TOC present at lines 9–20. Section headers present. Section ordering:

| # | Section | Present | Correct Order |
|---|---------|:-------:|:-------------:|
| 1 | module | ✅ | ✅ |
| 2 | imports | ✅ | ✅ |
| 3 | broadcast use | ✅ | ✅ |
| 4 | type definitions | ✅ | ✅ |
| 5 | view impls | ✅ | ✅ |
| 6 | spec helpers | ✅ | ✅ |
| 8 | traits | ✅ | ✅ |
| 9 | impls | ✅ | ✅ |
| 10 | iterators | ✅ | ✅ |
| 11 | derive impls in verus! | ✅ | ✅ |
| 13 | derive impls outside verus! | ✅ | ✅ |

**Issue:** Section 6 is labeled "spec helpers" instead of the standard "spec fns". The TOC omits section 7 (proof fns) which is fine since there are no proof functions.

**In/Out Table:**

| # | Chapter | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|---------|------|:-----:|:---:|:----:|:----:|:-----:|:-----:|:----:|:-----:|-------|
| 2 | Chap23 | PrimTreeSeqStPer | ✅ in | ✅ in | - | - | ✅ in | ✅ out | - | - | — |

All trait impls are correctly placed.

---

## Proof Holes Summary

```
❌ BalBinTreeStEph.rs
  Line 752: assume(r == (*self == *other))  — PartialEq for BalBinTree::Node
  Line 773: assume(r == (*self == *other))  — PartialEq for BalBinNode
  Line 781, 793: external_body — Clone for BalBinTree and BalBinNode

❌ PrimTreeSeqStPer.rs
  Line 831: assume(cloned@ == self@)       — Clone for PrimTreeSeqStS
  Line 847: assume(r == (self@ == other@))  — PartialEq for PrimTreeSeqStS
  Line 863: assume(cloned@ == self@)       — Clone for PrimTreeSeqStTree
  Line 885, 890: assume — PartialEq for PrimTreeSeqStTree::One, Two

Total: 9 holes (7 assume, 2 external_body).
```

- **7 assume:** 2 BalBinTree PartialEq (standard pattern); 2 PrimTreeSeqSt clone (Seq.clone preserves view); 3 PrimTreeSeqSt PartialEq (standard pattern).
- **2 external_body:** BalBinTree and BalBinNode Clone — `T::clone()` has no spec in Verus, so `external_body` cannot be removed without upstream support.

**Proof functions:** 2 clean, 0 holed.

**Bare impl errors (1):**

| # | File | Bare impl | Justification |
|---|------|-----------|---------------|
| 1 | BalBinTreeStEph.rs | `impl BalBinTree<T>` (lines 235+) | **Legitimate** — per `trait-impl-pattern.mdc`, recursive spec fns (`spec_size`, `spec_height`) must be inherent impls with `decreases self`; Verus cannot unfold through trait dispatch. Trait impl delegates to them. |

---

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 35 |
| partial | 0 |
| weak | 0 |
| none | 0 |

Every function in Chapter 23 has a strong specification. The trait contracts fully capture preconditions, postconditions, and input-output relationships. The `filter` spec is particularly thorough with four ensures clauses covering length bounds, multiset equality, and predicate satisfaction.

---

## Overall Assessment

**Chapter 23 is well-implemented and thoroughly verified.**

### Strengths
1. **100% strong specs** — Every function has a complete `requires`/`ensures` contract.
2. **Clean proof functions** — Both traversal permutation lemmas verify without holes.
3. **Comprehensive iterator infrastructure** — All iterator patterns (loop/for × borrow/consume) are implemented with full ForLoopGhostIterator support.
4. **Good PTT coverage** — 14 PTTs across both modules covering all iterator patterns.
5. **Trait-based design** — `PrimTreeSeqStTrait` and `BalBinTreeTrait` provide clean abstraction boundaries.
6. **Standard PartialEq pattern** — All equality impls follow the project's standard assume-based pattern.

### Weaknesses
1. **Vec-backed implementation** — PrimTreeSeqStPer uses a flat Vec, not a balanced tree. This means the APAS cost bounds from Cost Spec 23.2 (O(1) expose, O(1+|r(L)−r(R)|) join) are not achieved. The implementation is correct but suboptimal.
2. **Missing RTT coverage** — The derived operations (append, subseq, update, map, tabulate, filter, drop, flatten) have no runtime tests. Only constructors, expose/join, and iterators are tested.
3. **No post-order PTT** — Post-order iterator has no proof-time test.
4. **Clone external_body** — `BalBinTree` and `BalBinNode` Clone impls use `#[verifier::external_body]`. Cannot be removed because `T::clone()` has no spec in Verus; requires upstream support.
5. **No Mt modules** — The prose describes parallel operations (map‖, tabulate‖, filter‖) but no multi-threaded implementations exist.
6. **TOC minor issues** — Section 6 label inconsistency ("spec helpers" vs "spec fns"), missing proof fn listing in BalBinTree TOC.

### Recommendations
1. Add RTTs for append, subseq, update, map, tabulate, filter, drop, and flatten.
2. Add a post-order iterator PTT.
3. ~~Move BalBinTree/BalBinNode Clone impls inside `verus!`.~~ Blocked: `T::clone()` has no spec in Verus.
4. Consider adding Mt modules for map, tabulate, and filter to demonstrate the parallelism from Algorithm 23.3.
5. Consider a tree-backed implementation (AVL/Red-Black) to achieve the prose cost bounds, when later chapters provide the balancing infrastructure.
