# Review Against Prose: Chapter 38 -- Parametric BSTs

Generated: 2026-03-15 by Claude-Opus-4.6

## Phase 1: Inventory

| # | Chap | File | Module | Tr | IT | ML | V! | -V! | Unk | Hole | NoSpec |
|---|------|------|--------|:--:|:--:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | 38 | BSTParaStEph.rs | BSTParaStEph | 20 | 20 | 8 | 28 | 0 | 25 | 3 | 0 |
| 2 | 38 | BSTParaMtEph.rs | BSTParaMtEph | 17 | 17 | 16 | 18 | 14 | 9 | 7 | 16 |

Total functions: 60 (StEph) + 32 (MtEph) = 92.

## Phase 2: Prose Inventory

APAS Chapter 38 covers:

**Data Type 38.1** -- Parametric BST interface: types K, T, E (Leaf | Node), plus
`size`, `expose`, `joinMid`.

**Algorithm 38.2** -- empty, singleton, joinM (trivial wrappers around joinMid).

**Algorithm 38.3** -- split(T, k): recursive descent splitting at key k.

**Algorithm 38.4** -- joinPair(T1, T2): find minKey in T2, split it out, joinM.
Also defines minKey helper.

**Algorithm 38.5** -- insert(T, k) via split+joinM; delete(T, k) via split+joinPair.

**Algorithm 38.6** -- union(T1, T2): parallel divide-and-conquer using split.

**Algorithm 38.7** -- intersect(T1, T2): parallel, symmetric to union.

**Algorithm 38.8** -- difference(T1, T2): parallel, symmetric to union.

**Algorithm 38.9** -- filter(f, T): parallel recursive filter.

**Algorithm 38.10** -- reduce(f, I, T): parallel recursive reduce.

**Cost Specification 38.11**: empty O(1), singleton O(1), split O(lg n),
join O(lg(n1+n2)), find/insert/delete O(lg n),
union/intersect/difference Work O(m lg(n/m)) Span O(lg n).

**Theorems**: Cost analysis for union (brick method, direct derivation),
span analysis O(lg m lg n) improvable to O(lg n).

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations added to BSTParaStEph.rs trait declarations for all 15 public
functions. Every annotation now has both `APAS:` and `Claude-Opus-4.6:` lines.

### 3b. Implementation Fidelity

| # | Chap | File | Function | Prose Ref | Deviation |
|---|------|------|----------|-----------|-----------|
| 1 | 38 | BSTParaStEph.rs | new | Alg 38.2 | Faithful |
| 2 | 38 | BSTParaStEph.rs | singleton | Alg 38.2 | Faithful |
| 3 | 38 | BSTParaStEph.rs | expose | DT 38.1 | Uses RwLock read; clone extracts fields |
| 4 | 38 | BSTParaStEph.rs | join_mid | DT 38.1 | Faithful -- rebalancing is not applicable (parametric) |
| 5 | 38 | BSTParaStEph.rs | join_m | Alg 38.2 | Faithful wrapper |
| 6 | 38 | BSTParaStEph.rs | split | Alg 38.3 | Faithful recursive algorithm |
| 7 | 38 | BSTParaStEph.rs | insert | Alg 38.5 | Faithful: split + joinM |
| 8 | 38 | BSTParaStEph.rs | delete | Alg 38.5 | Faithful: split + joinPair |
| 9 | 38 | BSTParaStEph.rs | find | N/A | Standard BST search (APAS defers to Cost Spec 38.11) |
| 10 | 38 | BSTParaStEph.rs | min_key | Alg 38.4 | Faithful |
| 11 | 38 | BSTParaStEph.rs | join_pair | Alg 38.4 | Faithful: minKey + split + joinM |
| 12 | 38 | BSTParaStEph.rs | union | Alg 38.6 | Sequential (not parallel). Code matches prose structure. |
| 13 | 38 | BSTParaStEph.rs | intersect | Alg 38.7 | Sequential. Faithful to prose. |
| 14 | 38 | BSTParaStEph.rs | difference | Alg 38.8 | Sequential. Faithful to prose. |
| 15 | 38 | BSTParaStEph.rs | filter | Alg 38.9 | Sequential. Faithful to prose. |
| 16 | 38 | BSTParaStEph.rs | reduce | Alg 38.10 | Sequential. Faithful to prose. |
| 17 | 38 | BSTParaStEph.rs | collect_in_order | N/A | Verus scaffolding |
| 18 | 38 | BSTParaStEph.rs | in_order | N/A | Verus scaffolding for traversal |

**Notable**: StEph implements union/intersect/difference/filter/reduce sequentially.
This is correct for a single-threaded file. MtEph provides the parallel versions.

### 3c. Spec Fidelity

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|:-------------:|-------|
| 1 | 38 | BSTParaStEph.rs | new | Strong | ensures empty set, wf |
| 2 | 38 | BSTParaStEph.rs | singleton | Strong | ensures singleton set, finite, wf |
| 3 | 38 | BSTParaStEph.rs | expose | Strong | ensures full BST decomposition with ordering |
| 4 | 38 | BSTParaStEph.rs | join_mid | Strong | ensures correct set union with key |
| 5 | 38 | BSTParaStEph.rs | split | Strong | 8-clause ensures: found, partitions, disjoint, ordering |
| 6 | 38 | BSTParaStEph.rs | insert | Strong | ensures self@ =~= old(self)@.insert(key@) |
| 7 | 38 | BSTParaStEph.rs | delete | Strong | ensures self@ =~= old(self)@.remove(key@) |
| 8 | 38 | BSTParaStEph.rs | find | Strong | ensures biconditional containment |
| 9 | 38 | BSTParaStEph.rs | min_key | Strong | ensures containment + less-or-equal all |
| 10 | 38 | BSTParaStEph.rs | join_pair | Strong | ensures union, finite |
| 11 | 38 | BSTParaStEph.rs | union | Strong | ensures combined@ == self@.union(other@) |
| 12 | 38 | BSTParaStEph.rs | intersect | Strong | ensures common@ == self@.intersect(other@) |
| 13 | 38 | BSTParaStEph.rs | difference | Strong | ensures remaining@ == self@.difference(other@) |
| 14 | 38 | BSTParaStEph.rs | filter | Strong | 4-clause ensures: subset, finite, predicate-consistent |
| 15 | 38 | BSTParaStEph.rs | reduce | Partial | Only ensures identity on empty tree; no associativity proof |
| 16 | 38 | BSTParaStEph.rs | collect_in_order | Strong | ensures len preservation + element containment |
| 17 | 38 | BSTParaStEph.rs | in_order | Strong | ensures len + containment bijection |

## Phase 4: Parallelism Review (MtEph)

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|:--------------:|-------|
| 1 | 38 | BSTParaMtEph.rs | union_inner | Parallel | Uses `ParaPair!` macro for fork-join |
| 2 | 38 | BSTParaMtEph.rs | intersect_inner | Parallel | Uses `ParaPair!` macro |
| 3 | 38 | BSTParaMtEph.rs | difference_inner | Parallel | Uses `ParaPair!` macro |
| 4 | 38 | BSTParaMtEph.rs | filter_inner | Parallel | Uses `ParaPair!` with Arc-shared predicate |
| 5 | 38 | BSTParaMtEph.rs | reduce_inner | Parallel | Uses `ParaPair!` with Arc-shared op |
| 6 | 38 | BSTParaMtEph.rs | insert | Sequential | Split + rebuild |
| 7 | 38 | BSTParaMtEph.rs | delete | Sequential | Split + join_pair + rebuild |
| 8 | 38 | BSTParaMtEph.rs | find | Sequential | Loop-based search |
| 9 | 38 | BSTParaMtEph.rs | split_inner | Sequential | Recursive descent |
| 10 | 38 | BSTParaMtEph.rs | collect_in_order | Sequential | DFS traversal |

All parallel functions (union, intersect, difference, filter, reduce) correctly use
`ParaPair!` for fork-join parallelism. Functions that are inherently sequential
(find, split, insert, delete) remain sequential. This matches APAS Section 3
("Parallel Functions").

**Concern**: All 7 parallel trait functions in MtEph have `external_body`, meaning
their verification specs are assumed rather than proven. The actual parallel
implementations live outside `verus!` in functions suffixed `_inner` and `_parallel`.

## Phase 5: Runtime Test Review

| # | Chap | File | Test File | Tests | Coverage |
|---|------|------|-----------|:-----:|----------|
| 1 | 38 | BSTParaStEph.rs | TestBSTParaStEph.rs | Yes | insert, find, split, join_pair, union, delete, filter, reduce, in_order, macro |
| 2 | 38 | BSTParaMtEph.rs | TestBSTParaMtEph.rs | Yes | insert, find, split, join_pair, union, delete, intersect, difference, filter, reduce, in_order |

Tests are comprehensive, covering all major algorithms.

## Phase 6: PTT Review

No proof-time tests exist for Chapter 38. None are needed per project rules (no
iterators, no complicated callability issues).

## Phase 7: Gap Analysis

| # | Chap | File | Gap | Severity | Notes |
|---|------|------|-----|:--------:|-------|
| 1 | 38 | BSTParaStEph.rs | 4 assume() holes | Medium | 1 in expose (clone bridge), 2 in insert/delete (size overflow), 1 in union (size overflow) |
| 2 | 38 | BSTParaStEph.rs | 1 external_body (clone) | Low | ParamBST::clone uses external_body |
| 3 | 38 | BSTParaMtEph.rs | 8 external_body | Medium | View, join_pair, union, intersect, difference, filter, reduce, in_order |
| 4 | 38 | BSTParaMtEph.rs | 1 assume_specification | Medium | split_inner bridging |
| 5 | 38 | BSTParaStEph.rs | StEph sequential for union/intersect/diff | Design | Correct for StEph; MtEph provides parallelism |
| 6 | 38 | BSTParaStEph.rs | reduce spec is weak | Low | Only ensures identity on empty; no fold semantics |
| 7 | 38 | BSTParaStEph.rs | No `find` in prose | Low | APAS mentions find in cost table but provides no algorithm; implementation is standard BST |

**Summary**: 14 total holes (4 assume, 1 assume_specification, 9 external_body).
StEph has strong specs throughout. MtEph's parallel functions all have external_body
because the parallel inner functions live outside `verus!`.

## Phase 8: TOC Review

### BSTParaStEph.rs

Sections present: 1 (module), 2 (imports), 3 (broadcast), 4 (types), 5 (views),
6 (spec fns), 7 (proof fns), 8 (traits), 9 (impls), 10 (free fns -- labeled
incorrectly), 11 (derive impls in verus!), 12 (macros), 13 (derive impls outside verus!).

Section 10 is labeled as "free fns" but should be unnumbered or integrated.
The filter_inner and reduce_inner free functions are correctly inside `verus!`.

**In/Out placement**: Clone for Exposed and NodeInner correctly inside `verus!`.
Clone for ParamBST uses `external_body` inside `verus!`. Debug is outside `verus!`.
Macro `ParamBSTLit!` is correctly outside `verus!`.

### BSTParaMtEph.rs

Sections present: 1, 2, 3, 4, 5, 8, 9, 11, 13.

The parallel inner functions (union_inner, etc.) are correctly placed outside `verus!`
since they use `ParaPair!` macro which is not Verus-compatible.

**In/Out placement**: Correct. Trait and trait impl inside `verus!`; parallel
implementations outside `verus!`; Clone impls inside `verus!` with assume workarounds;
Debug outside.
