# Chapter 23 — Tree Sequences: Review Against Prose

Reviewer: Claude-Opus-4.6 (agent2)
Date: 2026-03-15
Prose source: prompts/Chap23.txt (Chapter 23: Tree Sequences)

## Phase 2: Prose Inventory

### Definitions and Data Types

| # | Chap | Prose Item | Kind | Description |
|---|------|-----------|------|-------------|
| 1 | 23 | Data Type 23.1 | type | Primitive tree sequence type S-alpha: Zero, One(x), Two(L,R) |
| 2 | 23 | Data Type 23.1 | fn | length : S-alpha -> N |
| 3 | 23 | Data Type 23.1 | fn | expose : S-alpha -> T-alpha |
| 4 | 23 | Data Type 23.1 | fn | join : T-alpha -> S-alpha |

### Cost Specifications

| # | Chap | Prose Item | Kind | Description |
|---|------|-----------|------|-------------|
| 5 | 23 | Cost Spec 23.2 | cost | length a: Work 1, Span 1 |
| 6 | 23 | Cost Spec 23.2 | cost | expose a: Work 1, Span 1 |
| 7 | 23 | Cost Spec 23.2 | cost | join(Zero) or join(One(x)): Work 1, Span 1 |
| 8 | 23 | Cost Spec 23.2 | cost | join(Two(L,R)): Work 1+abs(r(L)-r(R)), Span same |

### Algorithms

| # | Chap | Prose Item | Kind | Description |
|---|------|-----------|------|-------------|
| 9 | 23 | Algorithm 23.3 | fn | empty = join(Zero) |
| 10 | 23 | Algorithm 23.3 | fn | singleton x = join(One(x)) |
| 11 | 23 | Algorithm 23.3 | fn | append(a,b) = join(Two(a,b)) |
| 12 | 23 | Algorithm 23.3 | fn | nth S i (recursive on expose) |
| 13 | 23 | Algorithm 23.3 | fn | map f S (parallel on expose) |
| 14 | 23 | Algorithm 23.3 | fn | tabulate f n (parallel divide-and-conquer) |
| 15 | 23 | Algorithm 23.3 | fn | filter f S (parallel on expose) |
| 16 | 23 | Algorithm 23.3 | fn | drop S n (recursive on expose) |
| 17 | 23 | Algorithm 23.3 | fn | update S (i,v) (recursive on expose) |
| 18 | 23 | Algorithm 23.3 | fn | subseq S (a,n) = take(drop(S,a),n) |
| 19 | 23 | Algorithm 23.3 | fn | flatten S = reduce append empty S |

### Rank Properties (not directly implemented)

| # | Chap | Prose Item | Kind | Description |
|---|------|-----------|------|-------------|
| 20 | 23 | Rank property 1 | prop | r(T) in O(log abs(T)) |
| 21 | 23 | Rank property 2 | prop | For expose(T)=Two(L,R): r(L)+1 <= r(T) <= r(L)+2 |
| 22 | 23 | Rank property 3 | prop | r(join(Two(L,R))) <= max(r(L),r(R))+1 |

## Phase 3a: Cost Annotation Summary

All exec functions in both files now have APAS/Claude cost annotations.

### Cost Disagreements (implementation vs. prose)

The implementation (`PrimTreeSeqStPer.rs`) uses a Vec-backed representation rather
than a tree-based representation. This means several operations have different
asymptotic costs from the tree-based prose specification:

| # | Chap | File | Function | APAS Cost (Work/Span) | Impl Cost (Work/Span) | Reason |
|---|------|------|----------|-----------------------|-----------------------|--------|
| 1 | 23 | PrimTreeSeqStPer.rs | nth | Theta(log n) / Theta(log n) | Theta(1) / Theta(1) | Vec direct index vs tree traversal |
| 2 | 23 | PrimTreeSeqStPer.rs | expose | Theta(1) / Theta(1) | Theta(n) / Theta(n) | Vec clone-split vs tree root inspect |
| 3 | 23 | PrimTreeSeqStPer.rs | join(Two) | Theta(1+abs(r(L)-r(R))) | Theta(abs(L)+abs(R)) | Vec append vs tree rebalance |
| 4 | 23 | PrimTreeSeqStPer.rs | append | Theta(1+abs(r(a)-r(b))) | Theta(abs(a)+abs(b)) | Sequential clone loops |
| 5 | 23 | PrimTreeSeqStPer.rs | update | Theta(log^2 n) | Theta(n) | Copies entire Vec |
| 6 | 23 | PrimTreeSeqStPer.rs | map | Theta(n) / Theta(log n) | Theta(n) / Theta(n) | Sequential loop vs parallel tree |
| 7 | 23 | PrimTreeSeqStPer.rs | tabulate | Theta(n) / Theta(log n) | Theta(n) / Theta(n) | Sequential loop vs parallel tree |
| 8 | 23 | PrimTreeSeqStPer.rs | filter | Theta(n) / Theta(log^2 n) | Theta(n) / Theta(n) | Sequential loop vs parallel tree |
| 9 | 23 | PrimTreeSeqStPer.rs | drop | Theta(log^2 n) | Theta(abs(a)-n) | Delegates to subseq |

These disagreements are expected and intentional. The `PrimTreeSeqStPer` file
implements a "primitive" Vec-backed tree sequence as a foundation. The textbook's
tree-based cost bounds (Cost Spec 23.2) apply to a tree representation that would
be provided in later chapters (e.g., AVL trees, red-black trees, weight-balanced
trees). The Vec-backed implementation satisfies the same functional contracts
(correctness specs) even though its asymptotic costs differ.

### Functions with no cost disagreement

| # | Chap | File | Function | Cost (Work/Span) |
|---|------|------|----------|-------------------|
| 1 | 23 | PrimTreeSeqStPer.rs | empty | Theta(1) / Theta(1) |
| 2 | 23 | PrimTreeSeqStPer.rs | singleton | Theta(1) / Theta(1) |
| 3 | 23 | PrimTreeSeqStPer.rs | length | Theta(1) / Theta(1) |
| 4 | 23 | PrimTreeSeqStPer.rs | join(Zero) | Theta(1) / Theta(1) |
| 5 | 23 | PrimTreeSeqStPer.rs | join(One) | Theta(1) / Theta(1) |

### BalBinTreeStEph cost annotations

BalBinTreeStEph is not directly from the APAS prose; it is utility infrastructure
for balanced binary trees. Costs are all Theta(n) work and span for traversals
(size, height, in_order, pre_order, post_order) and Theta(1) for constructors
(leaf, node, is_leaf).

## Phase 3b: Implementation Fidelity

### PrimTreeSeqStPer.rs

| # | Chap | File | Prose Item | Status | Notes |
|---|------|------|-----------|--------|-------|
| 1 | 23 | PrimTreeSeqStPer.rs | Data Type 23.1 (type) | Faithful | Zero/One/Two enum present |
| 2 | 23 | PrimTreeSeqStPer.rs | length | Faithful | Returns Vec::len |
| 3 | 23 | PrimTreeSeqStPer.rs | expose | Faithful | Splits at midpoint, returns Zero/One/Two |
| 4 | 23 | PrimTreeSeqStPer.rs | join | Faithful | Reassembles from Zero/One/Two |
| 5 | 23 | PrimTreeSeqStPer.rs | empty (Alg 23.3) | Faithful | Creates empty Vec |
| 6 | 23 | PrimTreeSeqStPer.rs | singleton (Alg 23.3) | Faithful | Creates single-element Vec |
| 7 | 23 | PrimTreeSeqStPer.rs | append (Alg 23.3) | Deviation | Does not use join(Two(a,b)) — copies elements directly instead |
| 8 | 23 | PrimTreeSeqStPer.rs | nth (Alg 23.3) | Deviation | Direct Vec index rather than recursive expose-based |
| 9 | 23 | PrimTreeSeqStPer.rs | map (Alg 23.3) | Deviation | Sequential loop rather than recursive parallel expose |
| 10 | 23 | PrimTreeSeqStPer.rs | tabulate (Alg 23.3) | Deviation | Sequential loop rather than recursive parallel divide |
| 11 | 23 | PrimTreeSeqStPer.rs | filter (Alg 23.3) | Deviation | Sequential loop rather than recursive parallel expose |
| 12 | 23 | PrimTreeSeqStPer.rs | drop (Alg 23.3) | Deviation | Delegates to subseq rather than recursive expose |
| 13 | 23 | PrimTreeSeqStPer.rs | update (Alg 23.3) | Deviation | Copies entire Vec rather than recursive expose |
| 14 | 23 | PrimTreeSeqStPer.rs | subseq (Alg 23.3) | Faithful | take(drop(S,a),n) per prose |
| 15 | 23 | PrimTreeSeqStPer.rs | flatten (Alg 23.3) | Deviation | Nested loops rather than reduce append empty |

All deviations are justified by the Vec-backed representation. The prose algorithms
are written for a tree representation; a Vec-backed primitive implementation naturally
uses sequential iteration rather than recursive tree decomposition.

### BalBinTreeStEph.rs

BalBinTreeStEph is Verus infrastructure for balanced binary trees, not directly
from the Chapter 23 prose. The binary tree type (Leaf/Node) is a general utility,
not the tree sequence type (Zero/One/Two) from Data Type 23.1.

## Phase 3c: Spec Fidelity

### PrimTreeSeqStPer.rs spec strength

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|---------------|-------|
| 1 | 23 | PrimTreeSeqStPer.rs | empty | Strong | Ensures len==0 |
| 2 | 23 | PrimTreeSeqStPer.rs | singleton | Strong | Ensures len==1, index(0)==value |
| 3 | 23 | PrimTreeSeqStPer.rs | from_vec | Strong | Ensures len and index match Vec |
| 4 | 23 | PrimTreeSeqStPer.rs | length | Strong | Ensures len==spec_len |
| 5 | 23 | PrimTreeSeqStPer.rs | nth | Strong | Ensures dereferenced == spec_index |
| 6 | 23 | PrimTreeSeqStPer.rs | expose | Strong | Full Zero/One/Two characterization |
| 7 | 23 | PrimTreeSeqStPer.rs | join | Strong | Full Zero/One/Two reconstruction |
| 8 | 23 | PrimTreeSeqStPer.rs | append | Strong | Pointwise element preservation |
| 9 | 23 | PrimTreeSeqStPer.rs | subseq | Strong | Pointwise element preservation |
| 10 | 23 | PrimTreeSeqStPer.rs | update | Strong | Pointwise + updated index |
| 11 | 23 | PrimTreeSeqStPer.rs | map | Strong | Pointwise f.ensures |
| 12 | 23 | PrimTreeSeqStPer.rs | tabulate | Strong | Pointwise f.ensures |
| 13 | 23 | PrimTreeSeqStPer.rs | filter | Strong | Length, multiset, element preservation |
| 14 | 23 | PrimTreeSeqStPer.rs | drop | Strong | Pointwise element preservation |
| 15 | 23 | PrimTreeSeqStPer.rs | flatten | Strong | map_values + flatten on Seq |
| 16 | 23 | PrimTreeSeqStPer.rs | as_slice | Strong | Slice view matches spec |
| 17 | 23 | PrimTreeSeqStPer.rs | into_vec | Strong | Vec view matches spec |

All functional specs are strong. No weakenings detected relative to the textbook's
functional contracts.

### BalBinTreeStEph.rs spec strength

| # | Chap | File | Function | Spec Strength | Notes |
|---|------|------|----------|---------------|-------|
| 1 | 23 | BalBinTreeStEph.rs | leaf | Strong | Size=0, height=0, traversals empty |
| 2 | 23 | BalBinTreeStEph.rs | node | Strong | Size, height, all 3 traversals |
| 3 | 23 | BalBinTreeStEph.rs | is_leaf | Strong | Iff size==0 |
| 4 | 23 | BalBinTreeStEph.rs | size | Strong | Equals spec_size |
| 5 | 23 | BalBinTreeStEph.rs | height | Strong | Equals spec_height |
| 6 | 23 | BalBinTreeStEph.rs | in_order | Strong | Matches spec_in_order |
| 7 | 23 | BalBinTreeStEph.rs | pre_order | Strong | Matches spec_pre_order |
| 8 | 23 | BalBinTreeStEph.rs | post_order | Strong | Matches spec_post_order |

## Phase 4: Parallelism Review

Both files are single-threaded (StEph and StPer suffixes). No Mt variants exist
for Chapter 23 in the codebase. The prose specifies parallel map, tabulate, and
filter (using `||` notation for parallel evaluation), but these are implemented
sequentially as appropriate for the St variant. This is consistent with the project
pattern of providing St implementations first; Mt variants would be separate files.

Not applicable for this review.

## Phase 5: RTT Coverage

### BalBinTreeStEph RTT (TestBalBinTreeStEph.rs)

| # | Chap | File | Function | Tested | Test Count |
|---|------|------|----------|--------|------------|
| 1 | 23 | BalBinTreeStEph.rs | leaf | Yes | 2 |
| 2 | 23 | BalBinTreeStEph.rs | node | Yes | 5+ |
| 3 | 23 | BalBinTreeStEph.rs | is_leaf | Yes | 1 |
| 4 | 23 | BalBinTreeStEph.rs | size | Yes | 3 |
| 5 | 23 | BalBinTreeStEph.rs | height | Yes | 2 |
| 6 | 23 | BalBinTreeStEph.rs | in_order | Yes | 6 |
| 7 | 23 | BalBinTreeStEph.rs | pre_order | Yes | 5 |
| 8 | 23 | BalBinTreeStEph.rs | post_order | No | 0 |
| 9 | 23 | BalBinTreeStEph.rs | iter_in_order | Yes | 5 |
| 10 | 23 | BalBinTreeStEph.rs | iter_pre_order | Yes | 4 |
| 11 | 23 | BalBinTreeStEph.rs | iter_post_order | No | 0 |
| 12 | 23 | BalBinTreeStEph.rs | clone | Yes | 1 |
| 13 | 23 | BalBinTreeStEph.rs | eq | Yes | 1 (implicit in assert_eq) |
| 14 | 23 | BalBinTreeStEph.rs | Debug::fmt | Yes | 1 |

RTT gaps: `post_order` traversal has no direct RTT. `iter_post_order` has no RTT.
Both are covered at the PTT level.

### PrimTreeSeqStPer RTT (TestPrimTreeSeqStPer.rs)

| # | Chap | File | Function | Tested | Test Count |
|---|------|------|----------|--------|------------|
| 1 | 23 | PrimTreeSeqStPer.rs | empty | Yes | 3 |
| 2 | 23 | PrimTreeSeqStPer.rs | singleton | Yes | 3 |
| 3 | 23 | PrimTreeSeqStPer.rs | from_vec | Yes | 3 |
| 4 | 23 | PrimTreeSeqStPer.rs | length | Yes | 3 |
| 5 | 23 | PrimTreeSeqStPer.rs | nth | Yes | 1 |
| 6 | 23 | PrimTreeSeqStPer.rs | expose | Yes | 5 |
| 7 | 23 | PrimTreeSeqStPer.rs | join | Yes | 5 |
| 8 | 23 | PrimTreeSeqStPer.rs | append | Yes | 3 |
| 9 | 23 | PrimTreeSeqStPer.rs | subseq | Yes | 3 |
| 10 | 23 | PrimTreeSeqStPer.rs | update | Yes | 3 |
| 11 | 23 | PrimTreeSeqStPer.rs | map | Yes | 2 |
| 12 | 23 | PrimTreeSeqStPer.rs | tabulate | Yes | 2 |
| 13 | 23 | PrimTreeSeqStPer.rs | filter | Yes | 3 |
| 14 | 23 | PrimTreeSeqStPer.rs | drop | Yes | 3 |
| 15 | 23 | PrimTreeSeqStPer.rs | flatten | Yes | 3 |
| 16 | 23 | PrimTreeSeqStPer.rs | as_slice | Yes | 3 |
| 17 | 23 | PrimTreeSeqStPer.rs | into_vec | Yes | 2 |
| 18 | 23 | PrimTreeSeqStPer.rs | clone | Yes | 1 |
| 19 | 23 | PrimTreeSeqStPer.rs | eq | Yes | 1 |
| 20 | 23 | PrimTreeSeqStPer.rs | Debug::fmt | Yes | 1 |
| 21 | 23 | PrimTreeSeqStPer.rs | iter (generic) | Yes | 3 |

RTT gaps: None significant. All algorithmic functions are well-covered.

## Phase 6: PTT Review

### ProveBalBinTreeStEph.rs

Covers 6 iterator loop patterns + 5 spec-match tests:
- loop-in-order (manual next loop)
- loop-pre-order (manual next loop)
- loop-post-order (manual next loop)
- for-in-order (ForLoopGhostIterator)
- for-pre-order (ForLoopGhostIterator)
- for-post-order (ForLoopGhostIterator)
- postorder-spec-match
- inorder-spec-match
- preorder-spec-match
- leaf-traversals-empty
- leaf-iter-exhausted

All 11 PTTs pass. Coverage is thorough: all three traversal orders, both loop
forms (manual loop + for-in), spec matching, and edge cases (leaf).

### ProvePrimTreeSeqStPer.rs

Covers all 6 standard iterator loop patterns:
- loop-borrow-iter
- loop-borrow-into
- loop-consume
- for-borrow-iter
- for-borrow-into
- for-consume

All 6 PTTs pass. Full iterator coverage per the iterator standard.

## Phase 7: Gap Analysis

### Prose items with no code

| # | Chap | Prose Item | Gap Description |
|---|------|-----------|-----------------|
| 1 | 23 | Rank properties (1-3) | Not modeled in code. The rank concept is part of the tree-based cost analysis; the Vec-backed implementation does not need rank properties. Future tree-backed implementations (AVL, RB, WB) would need to prove these. |

### Code items with no prose

| # | Chap | File | Code Item | Description |
|---|------|------|-----------|-------------|
| 1 | 23 | BalBinTreeStEph.rs | BalBinTree type | General binary tree utility; not from Chapter 23 prose |
| 2 | 23 | BalBinTreeStEph.rs | All traversals | Binary tree traversals are general utilities |
| 3 | 23 | BalBinTreeStEph.rs | lemma_in_order_pre_order_permutation | Proof that traversals are permutations |
| 4 | 23 | BalBinTreeStEph.rs | lemma_pre_order_post_order_permutation | Proof that traversals are permutations |
| 5 | 23 | PrimTreeSeqStPer.rs | from_vec | Vec-specific constructor not in prose |
| 6 | 23 | PrimTreeSeqStPer.rs | as_slice | Utility accessor not in prose |
| 7 | 23 | PrimTreeSeqStPer.rs | into_vec | Utility accessor not in prose |
| 8 | 23 | PrimTreeSeqStPer.rs | iter | Iterator infrastructure not in prose |

## Phase 8: TOC and In/Out Table

### BalBinTreeStEph.rs TOC

Present and correct. Sections: 1 (module), 2 (imports), 3 (broadcast use),
4 (type definitions), 6 (spec fns), 7 (proof fns), 8 (traits), 9 (impls),
10 (iterators), 11 (derive impls in verus!), 13 (derive impls outside verus!).
Sections 5 and 12 correctly omitted (no View impls, no macros).

Style review notes 30 warnings, mostly about iterator struct ordering (structs
defined after impls in section 10 rather than in section 4) and missing
Display/Debug on iterator types. These are cosmetic.

### PrimTreeSeqStPer.rs TOC

Present and correct. Sections: 1 (module), 2 (imports), 3 (broadcast use),
4 (type definitions), 5 (view impls), 6 (spec fns), 8 (traits), 9 (impls),
10 (iterators), 11 (derive impls in verus!), 13 (derive impls outside verus!).
Sections 7 and 12 correctly omitted.

Style review notes 13 warnings, similar iterator struct ordering and missing
Display/Debug issues.

### In/Out Table

| # | Chap | File | Item | Expected | Actual |
|---|------|------|------|----------|--------|
| 1 | 23 | BalBinTreeStEph.rs | Clone impl | in verus! | in verus! |
| 2 | 23 | BalBinTreeStEph.rs | PartialEq impl | in verus! | in verus! |
| 3 | 23 | BalBinTreeStEph.rs | Eq impl | in verus! | in verus! |
| 4 | 23 | BalBinTreeStEph.rs | Debug impl | outside verus! | outside verus! |
| 5 | 23 | BalBinTreeStEph.rs | Iterator impls | in verus! | in verus! |
| 6 | 23 | PrimTreeSeqStPer.rs | Clone impl | in verus! | in verus! |
| 7 | 23 | PrimTreeSeqStPer.rs | PartialEq impl | in verus! | in verus! |
| 8 | 23 | PrimTreeSeqStPer.rs | Eq impl | in verus! | in verus! |
| 9 | 23 | PrimTreeSeqStPer.rs | Debug impl | outside verus! | outside verus! |
| 10 | 23 | PrimTreeSeqStPer.rs | Iterator impls | in verus! | in verus! |

All items correctly placed.

## Proof Holes Summary

| # | Chap | File | Holes | Accept | Assume | External Body |
|---|------|------|-------|--------|--------|---------------|
| 1 | 23 | BalBinTreeStEph.rs | 0 | 4 | 0 | 0 |
| 2 | 23 | PrimTreeSeqStPer.rs | 0 | 5 | 0 | 0 |
| | | **Total** | **0** | **9** | **0** | **0** |

All 9 `accept()` calls are in Clone::clone or PartialEq::eq bodies, which is the
approved pattern per the project rules. No algorithmic assumes, no external_body
on algorithmic logic. Chapter 23 is fully verified.

## Overall Assessment

Chapter 23 is in excellent shape:
- 0 proof holes across both files.
- All APAS Algorithm 23.3 operations implemented with strong specs.
- Cost disagreements are expected (Vec-backed vs tree-backed representation).
- Comprehensive RTT and PTT coverage.
- Clean code structure following project conventions.
- No missing prose operations (rank properties are deferred to tree implementations).
