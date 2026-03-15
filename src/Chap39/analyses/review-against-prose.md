# Review Against Prose: Chapter 39 -- Treaps

Generated: 2026-03-15 by Claude-Opus-4.6

## Phase 1: Inventory

| # | Chap | File | Module | Tr | IT | ML | V! | -V! | Unk | Hole | NoSpec |
|---|------|------|--------|:--:|:--:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | 39 | BSTTreapStEph.rs | BSTTreapStEph | 34 | 35 | 0 | 35 | 0 | 35 | 0 | 0 |
| 2 | 39 | BSTTreapMtEph.rs | BSTTreapMtEph | 12 | 13 | 20 | 33 | 0 | 24 | 7 | 2 |
| 3 | 39 | BSTParaTreapMtEph.rs | BSTParaTreapMtEph | 17 | 17 | 16 | 18 | 15 | 9 | 9 | 15 |
| 4 | 39 | BSTSetTreapMtEph.rs | BSTSetTreapMtEph | 20 | 20 | 2 | 22 | 0 | 22 | 0 | 0 |

Total functions: 35 + 33 + 33 + 22 = 123.

## Phase 2: Prose Inventory

APAS Chapter 39 covers:

**Definition 39.1** -- Treap: BST with random priorities satisfying heap property.

**Algorithm 39.2** -- qsTree: quicksort generates the treap structure (analysis tool).

**Data Structure 39.3** -- Treap implementation of the Data Type 38.1 interface:
- type T = TLeaf | TNode of (T x K x Z x T)
- priority(T): returns priority of root or -infinity
- join(T1, (k,p), T2): main join with priority-based rebalancing
- expose(T): T -> E
- joinMid(T): E -> T (calls join with fresh priority)

**Cost of Join**: O(h(T1) + h(T2)) = O(log(|T1| + |T2|)) with high probability.

**Cost of Split**: O(log |T|) with high probability (each joinM during split
takes O(1) because the split key has higher priority than the subtrees).

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

BSTTreapStEph.rs already has full `APAS:` and `Claude-Opus-4.6:` cost annotations
on all 35 trait functions. No additions needed.

### 3b. Implementation Fidelity

| # | Chap | File | Function | Prose Ref | Deviation |
|---|------|------|----------|-----------|-----------|
| 1 | 39 | BSTTreapStEph.rs | new | DS 39.3 | Faithful: creates TLeaf |
| 2 | 39 | BSTTreapStEph.rs | insert_link | DS 39.3 | Uses rotate_left/right instead of explicit join. Functionally equivalent: inserts at leaf, bubbles up via rotations to maintain heap order. |
| 3 | 39 | BSTTreapStEph.rs | delete_link | Derived | Filter-then-rebuild approach. APAS does not specify delete algorithm for Treaps. |
| 4 | 39 | BSTTreapStEph.rs | find (find_link) | Standard BST | Standard BST search. APAS notes O(log n) cost. |
| 5 | 39 | BSTTreapStEph.rs | rotate_left | DS 39.3 | Standard BST rotation, maintains heap property |
| 6 | 39 | BSTTreapStEph.rs | rotate_right | DS 39.3 | Standard BST rotation |
| 7 | 39 | BSTTreapStEph.rs | minimum/maximum | Standard BST | Left/right spine traversal |

**Key difference**: BSTTreapStEph implements insert via rotation-based bubbling rather
than the parametric split+joinMid approach. This is a valid alternative that is more
traditional for treaps. The parametric approach is implemented in BSTParaTreapMtEph.

### 3c. Spec Fidelity

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|:-------------:|-------|
| 1 | 39 | BSTTreapStEph.rs | new | Strong | 0 size, wf, bst |
| 2 | 39 | BSTTreapStEph.rs | insert | Strong | wf maintained, size bounded, containment preserved, BST maintained |
| 3 | 39 | BSTTreapStEph.rs | delete | Strong | wf maintained, size bounded, containment subset |
| 4 | 39 | BSTTreapStEph.rs | find | Partial | ensures found ==> contains, but does NOT ensure !found ==> !contains (one-directional) |
| 5 | 39 | BSTTreapStEph.rs | contains | Strong | biconditional: found == spec_contains(*target) |
| 6 | 39 | BSTTreapStEph.rs | rotate_left/right | Strong | preserves wf, size, BST, containment bijection |
| 7 | 39 | BSTTreapStEph.rs | insert_link | Strong | wf, size bounds, containment forward+backward, BST |
| 8 | 39 | BSTTreapStEph.rs | delete_link | Partial | wf, size, containment subset, BST -- but no ensures about what was deleted |
| 9 | 39 | BSTTreapStEph.rs | minimum/maximum | Strong | matches spec_min/spec_max |
| 10 | 39 | BSTTreapStEph.rs | size | Strong | equals spec_size |
| 11 | 39 | BSTSetTreapMtEph.rs | all | Strong | Full set algebra specs (union, intersect, difference, etc.) |

### Spec Fidelity for Mt Files

| # | Chap | File | Aspect | Strength | Notes |
|---|------|------|--------|:--------:|-------|
| 1 | 39 | BSTTreapMtEph.rs | find | Hole | assume() bridges RwLock boundary |
| 2 | 39 | BSTTreapMtEph.rs | size | Hole | assume() bridges RwLock boundary |
| 3 | 39 | BSTTreapMtEph.rs | min/max | Hole | assume() bridges RwLock boundary |
| 4 | 39 | BSTTreapMtEph.rs | in_order/pre_order | Hole | assume() bridges RwLock boundary |
| 5 | 39 | BSTParaTreapMtEph.rs | join_mid, split, etc. | Hole | 10 external_body wrapping parallel inner functions |
| 6 | 39 | BSTSetTreapMtEph.rs | all functions | Clean | Zero holes -- delegates to BSTParaTreapMtEph |

## Phase 4: Parallelism Review (Mt files)

### BSTTreapMtEph.rs -- Coarse-lock wrapper

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|:--------------:|-------|
| 1 | 39 | BSTTreapMtEph.rs | insert | Sequential | acquire_write, insert_link, release_write |
| 2 | 39 | BSTTreapMtEph.rs | delete | Sequential | acquire_write, delete_link, release_write |
| 3 | 39 | BSTTreapMtEph.rs | find | Sequential | acquire_read, find_link, release_read |
| 4 | 39 | BSTTreapMtEph.rs | size | Sequential | acquire_read, size_link, release_read |

BSTTreapMtEph is a coarse-lock wrapper (RwLock around root Link). All operations
are sequential with lock boundaries. This is correct -- it provides thread-safe
access without internal parallelism.

### BSTParaTreapMtEph.rs -- Parametric parallel treap

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|:--------------:|-------|
| 1 | 39 | BSTParaTreapMtEph.rs | union_inner | Parallel | Uses `ParaPair!` |
| 2 | 39 | BSTParaTreapMtEph.rs | intersect_inner | Parallel | Uses `ParaPair!` |
| 3 | 39 | BSTParaTreapMtEph.rs | difference_inner | Parallel | Uses `ParaPair!` |
| 4 | 39 | BSTParaTreapMtEph.rs | filter_inner | Parallel | Uses `ParaPair!` with Arc-shared pred |
| 5 | 39 | BSTParaTreapMtEph.rs | reduce_inner | Parallel | Uses `ParaPair!` with Arc-shared op |
| 6 | 39 | BSTParaTreapMtEph.rs | split_inner | Sequential | Recursive descent |
| 7 | 39 | BSTParaTreapMtEph.rs | join_with_priority | Sequential | Priority-based treap join |

Parallel functions match the APAS algorithms (38.6-38.10) but specialized to
treap's priority-based join. The join_with_priority function implements
Data Structure 39.3's join algorithm.

### BSTSetTreapMtEph.rs -- Set shim

All functions delegate to BSTParaTreapMtEph. Pure shim -- no parallelism logic.

## Phase 5: Runtime Test Review

| # | Chap | File | Test File | Tests | Coverage |
|---|------|------|-----------|:-----:|----------|
| 1 | 39 | BSTTreapStEph.rs | TestBSTTreapStEph.rs | Yes | insert, find, contains, delete, min, max, size, height, in_order, pre_order, macro |
| 2 | 39 | BSTTreapMtEph.rs | TestBSTTreapMtEph.rs | Yes | insert, find, contains, delete, min, max, size, height, in_order, pre_order, macro, default |
| 3 | 39 | BSTParaTreapMtEph.rs | TestBSTParaTreapMtEph.rs | Yes | insert, find, delete, split, join_pair, union, intersect, difference, filter, reduce, in_order, macro |
| 4 | 39 | BSTSetTreapMtEph.rs | TestBSTSetTreapMtEph.rs | Yes | empty, singleton, insert, delete, contains, find, union, intersection, difference, split, join_pair, filter, reduce, min, max, macro |

All four modules have comprehensive RTTs.

## Phase 6: PTT Review

No proof-time tests exist for Chapter 39. None are needed (no iterators, no
complicated callability).

## Phase 7: Gap Analysis

| # | Chap | File | Gap | Severity | Notes |
|---|------|------|-----|:--------:|-------|
| 1 | 39 | BSTTreapStEph.rs | 2 external_body | Medium | find (trait find) and insert_link |
| 2 | 39 | BSTTreapStEph.rs | find spec is one-directional | Low | found ==> contains, but !found does not ensure !contains |
| 3 | 39 | BSTTreapMtEph.rs | 6 assume() | Medium | All at RwLock boundaries (find, size, min, max, in_order, pre_order) |
| 4 | 39 | BSTParaTreapMtEph.rs | 10 external_body | Medium | View + 9 trait methods with parallel impls |
| 5 | 39 | BSTSetTreapMtEph.rs | 0 holes | Clean | Fully delegating shim |
| 6 | 39 | BSTTreapStEph.rs | No expose/joinMid | Design | BSTTreapStEph implements treap operations directly (rotation-based). The parametric expose/joinMid interface is in BSTParaTreapMtEph. |
| 7 | 39 | BSTTreapMtEph.rs | 8 requires_true warnings | Low | clone_link, size_link, find_link, min_link, max_link have vacuous preconditions |
| 8 | 39 | All | No heap-ordering spec | Medium | Treap invariant (parent priority >= child priority) is not formally specified or verified. Only BST ordering is checked. |

**Summary**: 18 total holes (6 assume, 12 external_body).
BSTSetTreapMtEph is clean (0 holes). BSTTreapStEph has only 2 holes (find, insert_link).
The heap-ordering invariant from Definition 39.1 is not formally verified -- only
structural BST properties are tracked. This means the probabilistic balance guarantees
are assumed rather than proven, which is acceptable since random priorities are
implementation concerns, not functional correctness.

## Phase 8: TOC Review

### BSTTreapStEph.rs

Sections: 1 (module), 4 (type definitions), 6 (spec fns), 8 (traits), 9 (impls),
11 (derive impls in verus!), 12 (macros), 13 (derive impls outside verus!).

Missing sections 2 (imports -- imports exist but no header), 3 (broadcast), 5 (views),
7 (proof fns). All sections present are inside `verus!`. Debug/Display correctly
outside `verus!`. Default impl correctly inside `verus!`.

**In/Out placement**: All code inside `verus!` except Debug/Display. Correct.

### BSTTreapMtEph.rs

Sections are present and correctly ordered. Clone impls inside `verus!` with
assume workarounds. Debug outside. Free functions (link-level operations) inside
`verus!`. Correct placement.

### BSTParaTreapMtEph.rs

Similar structure to Chap38/BSTParaMtEph.rs. Parallel inner functions outside
`verus!`. Trait+impl inside `verus!`. Correct.

### BSTSetTreapMtEph.rs

Clean shim with proper TOC. All inside `verus!` except Debug/Display. Macro
outside `verus!`. Correct.
