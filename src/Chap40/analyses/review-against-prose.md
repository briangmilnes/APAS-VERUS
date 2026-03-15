# Review Against Prose: Chapter 40 -- Augmenting Binary Search Trees

Generated: 2026-03-15 by Claude-Opus-4.6

## Phase 1: Inventory

| # | Chap | File | Module | Tr | IT | ML | V! | -V! | Unk | Hole | NoSpec |
|---|------|------|--------|:--:|:--:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | 40 | BSTKeyValueStEph.rs | BSTKeyValueStEph | 26 | 28 | 9 | 37 | 0 | 37 | 0 | 0 |
| 2 | 40 | BSTSizeStEph.rs | BSTSizeStEph | 31 | 33 | 5 | 38 | 0 | 38 | 0 | 0 |
| 3 | 40 | BSTReducedStEph.rs | BSTReducedStEph | 36 | 38 | 3 | 41 | 0 | 39 | 2 | 0 |

Total functions: 37 + 38 + 41 = 116.

## Phase 2: Prose Inventory

APAS Chapter 40 covers:

**Section 1: Augmenting with Values** -- Store (key, value) pairs in BST nodes.
Key-value BST ("dictionary/table"). Functions like find/split return optional values.

**Section 2: Augmenting with Size** -- Store subtree size at each node for O(1) size.
- TNode type: (L, k, p, n, R) with n = size
- size(T) reads n field in O(1)
- makeNode(L, k, p, R) computes size from children

**Algorithm 40.1 -- Rank and Select**:
- rank(T, k): number of keys <= k. O(log n) with size augmentation.
- select(T, i): key with rank i. O(log n) with size augmentation.

**Exercise 40.1 -- splitRank**: splits tree at rank boundary.

**Section 3: Augmenting with Reduced Values** -- Store associative reduction at each node.
- TNode type: (L, k, p, n, v, r, R) where r = reduced value
- reducedVal(T): returns r field in O(1)
- makeNode applies f(reducedVal(L), f(v, reducedVal(R)))

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All three Chap40 files already have `Claude-Opus-4.6:` cost annotations on trait
functions. Several also have `APAS:` lines. The annotations are complete.

### 3b. Implementation Fidelity

**BSTKeyValueStEph.rs**:

| # | Chap | File | Function | Prose Ref | Deviation |
|---|------|------|----------|-----------|-----------|
| 1 | 40 | BSTKeyValueStEph.rs | new | Sec 1 | Faithful |
| 2 | 40 | BSTKeyValueStEph.rs | insert | Sec 1 | Rotation-based treap insert with (key, value). Faithful to treap paradigm. |
| 3 | 40 | BSTKeyValueStEph.rs | delete | Sec 1 | Filter-rebuild approach. O(n) not O(log n). |
| 4 | 40 | BSTKeyValueStEph.rs | find/get | Sec 1 | Standard BST search returning &V. Faithful. |
| 5 | 40 | BSTKeyValueStEph.rs | minimum_key/maximum_key | Sec 1 | Left/right spine traversal. Faithful. |
| 6 | 40 | BSTKeyValueStEph.rs | keys/values | N/A | Verus scaffolding for collection traversal. |

**BSTSizeStEph.rs**:

| # | Chap | File | Function | Prose Ref | Deviation |
|---|------|------|----------|-----------|-----------|
| 1 | 40 | BSTSizeStEph.rs | size | Sec 2 | O(1) via node.size field. Faithful. |
| 2 | 40 | BSTSizeStEph.rs | make_node | Sec 2 | Computes size = 1 + left.size + right.size. Faithful. |
| 3 | 40 | BSTSizeStEph.rs | update_size | Sec 2 | Recomputes size in-place. Faithful. |
| 4 | 40 | BSTSizeStEph.rs | rank | Alg 40.1 | Faithful to prose: Leaf=>0, Less=>recurse left, Equal=>|L|+1, Greater=>|L|+1+recurse right. |
| 5 | 40 | BSTSizeStEph.rs | select | Alg 40.1 | Faithful to prose: Leaf=>exception, compare i vs |L|+1. |
| 6 | 40 | BSTSizeStEph.rs | split_rank | Ex 40.1 | Implements the exercise. |
| 7 | 40 | BSTSizeStEph.rs | insert | Sec 2 | Rotation-based treap insert with update_size. Faithful. |
| 8 | 40 | BSTSizeStEph.rs | delete | Sec 2 | Filter-rebuild approach. |

**BSTReducedStEph.rs**:

| # | Chap | File | Function | Prose Ref | Deviation |
|---|------|------|----------|-----------|-----------|
| 1 | 40 | BSTReducedStEph.rs | ReduceOp trait | Sec 3 | Captures associative function f with identity I. Faithful. |
| 2 | 40 | BSTReducedStEph.rs | make_node | Sec 3 | Computes r = combine(reducedVal(L), combine(lift(v), reducedVal(R))). Faithful to prose makeNode. |
| 3 | 40 | BSTReducedStEph.rs | update_node | Sec 3 | In-place version of make_node. |
| 4 | 40 | BSTReducedStEph.rs | reduced_value | Sec 3 | O(1) read of root's reduced_value field. Faithful. |
| 5 | 40 | BSTReducedStEph.rs | range_reduce | Sec 3 | Range query using BST search + reduction. O(log n). |
| 6 | 40 | BSTReducedStEph.rs | SumOp/CountOp | N/A | Concrete ReduceOp implementations for testing. |

### 3c. Spec Fidelity

**BSTKeyValueStEph.rs**:

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|:-------------:|-------|
| 1 | 40 | BSTKeyValueStEph.rs | insert | Strong | ensures self@ == old@.insert(key, value), size bounds |
| 2 | 40 | BSTKeyValueStEph.rs | delete | Strong | ensures self@ == old@.remove(*key) |
| 3 | 40 | BSTKeyValueStEph.rs | find/get | Strong | biconditional contains_key, value correctness |
| 4 | 40 | BSTKeyValueStEph.rs | contains | Strong | biconditional contains_key |
| 5 | 40 | BSTKeyValueStEph.rs | min/max_key | Strong | spec_min/max_key matching |

**BSTSizeStEph.rs**:

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|:-------------:|-------|
| 1 | 40 | BSTSizeStEph.rs | size | Strong | equals spec_size |
| 2 | 40 | BSTSizeStEph.rs | insert | Strong | set insert + wf + size bounds |
| 3 | 40 | BSTSizeStEph.rs | delete | Strong | set remove + wf + size bound |
| 4 | 40 | BSTSizeStEph.rs | find | Partial | only ensures None when link is None |
| 5 | 40 | BSTSizeStEph.rs | contains | Partial | only ensures contains == self@.contains(*target) |
| 6 | 40 | BSTSizeStEph.rs | rank | Partial | only ensures rank <= spec_size |
| 7 | 40 | BSTSizeStEph.rs | select | Partial | only ensures None when rank out of range |
| 8 | 40 | BSTSizeStEph.rs | split_rank | Partial | only ensures child wf, no content spec |

**BSTReducedStEph.rs**:

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|:-------------:|-------|
| 1 | 40 | BSTReducedStEph.rs | reduced_value | Partial | only ensures identity on empty; no spec connecting to actual reduction |
| 2 | 40 | BSTReducedStEph.rs | range_reduce | Partial | same: only ensures identity on empty |
| 3 | 40 | BSTReducedStEph.rs | identity/combine | Hole | external_body on ReduceOp impls (SumOp, CountOp) |

## Phase 4: Parallelism Review

Chapter 40 has no Mt files. All three modules are StEph (single-threaded ephemeral).
This is consistent with APAS Chapter 40 which does not present parallel algorithms --
it describes augmentation techniques applicable to any balanced BST.

## Phase 5: Runtime Test Review

| # | Chap | File | Test File | Tests | Coverage |
|---|------|------|-----------|:-----:|----------|
| 1 | 40 | BSTKeyValueStEph.rs | TestBSTKeyValueStEph.rs | Yes | basic ops, update existing key, collections (keys, values), delete, min/max, height, empty tree, macro |
| 2 | 40 | BSTSizeStEph.rs | TestBSTSizeStEph.rs | Yes | basic ops, rank, select, split_rank, in_order, delete, macro |
| 3 | 40 | BSTReducedStEph.rs | TestBSTReducedStEph.rs | Yes | sum reduction, count reduction, range_reduce, update existing key, delete, macro |

All three modules have comprehensive RTTs. The rank/select tests in BSTSizeStEph
are particularly thorough, testing all 9 ranks and edge cases.

## Phase 6: PTT Review

No proof-time tests exist for Chapter 40. None are needed.

## Phase 7: Gap Analysis

| # | Chap | File | Gap | Severity | Notes |
|---|------|------|-----|:--------:|-------|
| 1 | 40 | BSTKeyValueStEph.rs | 5 external_body | Medium | insert, delete, find, contains, get |
| 2 | 40 | BSTSizeStEph.rs | 4 external_body | Medium | insert, delete, find, contains |
| 3 | 40 | BSTReducedStEph.rs | 5 external_body | Medium | insert, delete, find, contains, get |
| 4 | 40 | BSTReducedStEph.rs | 2 holes in ReduceOp | Low | identity() and combine() for SumOp/CountOp are external_body |
| 5 | 40 | BSTSizeStEph.rs | rank/select specs weak | Medium | rank only ensures <= size; select only ensures None on OOB. No functional correctness. |
| 6 | 40 | BSTSizeStEph.rs | split_rank spec weak | Medium | Only ensures child wf, no content relationship |
| 7 | 40 | BSTReducedStEph.rs | reduced_value spec weak | Medium | Only ensures identity on empty. No spec connecting stored reduced value to actual tree content. |
| 8 | 40 | BSTReducedStEph.rs | range_reduce spec weak | Medium | Only ensures identity on empty |
| 9 | 40 | BSTKeyValueStEph.rs | delete is O(n) | Low | Filter-rebuild instead of O(log n). Functional spec is correct but cost deviates from APAS expectation. |
| 10 | 40 | BSTSizeStEph.rs | delete is O(n) | Low | Same filter-rebuild approach |
| 11 | 40 | BSTReducedStEph.rs | delete is O(n) | Low | Same filter-rebuild approach |
| 12 | 40 | All | No Mt files | Design | Correct -- APAS Ch40 is about augmentation, not parallelism |

**Summary**: 14 total holes (all external_body).
All three files share the same structural pattern: rotation-based treap insert (proven
for size/containment), filter-rebuild delete (proven for content but O(n)), and
external_body on insert/delete/find/contains where the implementation exists but
verification is incomplete.

The rank, select, reduced_value, and range_reduce functions have weak specs that
ensure basic safety (bounds, empty-case identity) but do not prove functional
correctness against the APAS definitions. These are targets for strengthening.

## Phase 8: TOC Review

### BSTKeyValueStEph.rs

Sections: 1 (module), 3 (broadcast), 4 (types), 5 (views), 6 (spec fns),
7 (proof fns), 8 (traits), 9 (impls), 11 (derive impls in verus!),
12 (macros), 13 (derive impls outside verus!).

The clone_link and compare_kv_links free functions appear between sections 8 and 9.
They should be in section 9 or a labeled free-function section.

**In/Out placement**: All code inside `verus!` except Debug/Display. Default and
PartialEq inside `verus!` with accept workarounds. Correct.

### BSTSizeStEph.rs

Sections: 1 (module), 2 (imports -- inside verus!), 4 (types), 5 (views),
7 (proof fns), 8 (traits), 9 (impls), 11 (derive impls in verus!),
12 (macros), 13 (derive impls outside verus!).

Imports placed inside `verus!` block -- unconventional but functional.
clone_link and compare_links free functions between sections 8 and 9.

### BSTReducedStEph.rs

Sections: 1 (module), 4 (types), 5 (views), 7 (proof fns), 8 (traits),
9 (impls), 11 (derive impls in verus!), 12 (macros), 13 (derive impls outside verus!).

Similar structure to BSTKeyValueStEph. clone_link and compare_reduced_links
free functions between sections 8 and 9.

All three files follow the same structural template, which is consistent.
