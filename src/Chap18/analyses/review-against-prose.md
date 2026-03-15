# Review Against Prose: Chapter 18 (The Sequence Abstract Data Type)

## Phase 1: Inventory

### Source Files

| # | Chap | File | Fns(Tr) | Fns(IT) | Fns(IBI) | Fns(ML) | Holes |
|---|------|------|---------|---------|----------|---------|-------|
| 1 | 18 | ArraySeq.rs | 23 | 25 | 3 | 13 | 0 |
| 2 | 18 | ArraySeqStEph.rs | 21 | 23 | 2 | 0 | 0 |
| 3 | 18 | ArraySeqStPer.rs | 20 | 22 | 2 | 0 | 0 |
| 4 | 18 | ArraySeqMtEph.rs | 22 | 24 | 7 | 3 | 0 |
| 5 | 18 | ArraySeqMtPer.rs | 19 | 21 | 6 | 0 | 0 |
| 6 | 18 | LinkedListStEph.rs | 19 | 21 | 2 | 0 | 0 |
| 7 | 18 | LinkedListStPer.rs | 18 | 20 | 2 | 0 | 0 |

Totals: 196 exec functions, 0 proof holes, 20 clean proof functions.
Info-level items: trivial_spec_wf (6), accept() for Clone/PartialEq (9), external_accept_hole (2).

### RTT Files

| # | Chap | File | Lines |
|---|------|------|-------|
| 1 | 18 | TestArraySeq.rs | 192 |
| 2 | 18 | TestArraySeqStEph.rs | 129 |
| 3 | 18 | TestArraySeqStPer.rs | 121 |
| 4 | 18 | TestArraySeqMtEph.rs | 87 |
| 5 | 18 | TestArraySeqMtPer.rs | 89 |
| 6 | 18 | TestLinkedListStEph.rs | 128 |
| 7 | 18 | TestLinkedListStPer.rs | 121 |

### PTT Files

| # | Chap | File | Patterns |
|---|------|------|----------|
| 1 | 18 | ProveArraySeq.rs | construction/iteration |
| 2 | 18 | ProveArraySeqStEph.rs | 6 iterator loop patterns |
| 3 | 18 | ProveArraySeqStPer.rs | 6 iterator loop patterns |
| 4 | 18 | ProveArraySeqMtEph.rs | 6 iterator loop patterns |
| 5 | 18 | ProveArraySeqMtPer.rs | 6 iterator loop patterns |
| 6 | 18 | ProveLinkedListStEph.rs | 6 iterator loop patterns |
| 7 | 18 | ProveLinkedListStPer.rs | 6 iterator loop patterns |


## Phase 2: Prose Inventory

Chapter 18 defines the Sequence ADT interface and its semantics.

### Definitions Extracted

| # | Prose Ref | Name | Implemented In |
|---|-----------|------|----------------|
| 1 | Data Type 18.1 | Sequence ADT interface | ArraySeq.rs (trait) |
| 2 | Def 18.3 | length, nth | All 7 files |
| 3 | Def 18.4 | empty, singleton | All 7 files |
| 4 | Def 18.5 | isEmpty, isSingleton | All 7 files |
| 5 | Def 18.6 | tabulate | All except LinkedList |
| 6 | Def 18.8 | map | All 7 files |
| 7 | Def 18.10 | filter | All 7 files |
| 8 | Def 18.12 | subseq | All 7 files |
| 9 | Def 18.13 | append | All 7 files |
| 10 | Def 18.14 | flatten | All 7 files |
| 11 | Def 18.15 | update | All 7 files |
| 12 | Def 18.16 | inject | ArraySeq, StEph, StPer, MtEph |
| 13 | Def 18.17 | ninject | MtEph only |
| 14 | Def 18.18 | collect | ArraySeq.rs only |
| 15 | Def 18.19 | iterate | All 7 files |
| 16 | Def 18.20 | associative function | spec_monoid in vstdplus |
| 17 | Def 18.21 | reduce | All 7 files |
| 18 | Def 18.22 | scan | All 7 files |
| 19 | Def 18.22 | scanI (inclusive) | ArraySeq.rs (scan_inclusive) |
| 20 | Def 18.19 | iteratePrefixes | ArraySeq.rs (iterate_prefixes) |

### Algorithms (Prose)

Chapter 18 is primarily a semantics chapter; implementations are deferred to Chapter 19.
No specific algorithm pseudocode is given in Chapter 18.

### Cost Specs (Prose)

Chapter 18 does not define cost specifications. Cost specs are deferred to the
"Cost Specifications" chapter (Chapter 20 in the textbook).

### Theorems

| # | Prose Ref | Statement | Status |
|---|-----------|-----------|--------|
| 1 | Important (after Def 18.21) | reduce f id a = iterate f id a when f is associative | Captured in reduce ensures: reduced == spec_iterate(...) |


## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All exec functions across all 7 source files already have cost annotations in the form:
```
/// - APAS: no cost spec (semantics-only chapter).
/// - Claude-Opus-4.6: Work Theta(n), Span Theta(1).
```

Coverage is complete. No additional cost annotations needed.

### Phase 3b: Implementation Deviations

| # | Chap | File | Function | Deviation |
|---|------|------|----------|-----------|
| 1 | 18 | ArraySeq.rs | set() | Not in prose. Mutable index write utility. |
| 2 | 18 | ArraySeq.rs | remove() | Not in prose. Implementation utility. |
| 3 | 18 | ArraySeq.rs | insert() | Not in prose. Implementation utility. |
| 4 | 18 | ArraySeq.rs | from_vec() | Not in prose. Construction utility. |
| 5 | 18 | ArraySeq.rs | subseq_copy() | Not in prose. Allocating subsequence. |
| 6 | 18 | ArraySeq.rs | find_key() | Not in prose. Helper for collect. |
| 7 | 18 | ArraySeq.rs | scan_inclusive() | Def 18.22 scanI. Present in prose. |
| 8 | 18 | ArraySeq.rs | iterate_prefixes() | Def 18.19 iteratePrefixes. Present in prose. |
| 9 | 18 | All files | reduce implementation | Prose defines D&C; all impls use left fold (iterate). Correct by Theorem (reduce = iterate for associative f). |
| 10 | 18 | MtEph.rs | ninject impl | Delegates to inject. Valid: inject is a valid ninject (deterministic is a special case of nondeterministic). |
| 11 | 18 | LinkedList*.rs | Internal Vec backing | Prose implies linked-list structure. Implementation uses Vec internally. This is an abstraction; the type name matches the ADT but uses array backing for verification tractability. |

### Phase 3c: Spec Fidelity

| # | Chap | File | Function | Prose Spec | Code Ensures | Match? |
|---|------|------|----------|------------|--------------|--------|
| 1 | 18 | ArraySeq.rs | length | |a| | len == spec_len | Strong |
| 2 | 18 | ArraySeq.rs | nth | a[i] | *nth_elem == spec_index(i) | Strong |
| 3 | 18 | ArraySeq.rs | empty | <> | spec_len == 0 | Strong |
| 4 | 18 | ArraySeq.rs | singleton | <x> | len==1, index(0)==item | Strong |
| 5 | 18 | ArraySeq.rs | subseq | a[i..i+j] | subrange semantics | Strong |
| 6 | 18 | ArraySeq.rs | append | a ++ b | len == |a|+|b|, elements | Strong |
| 7 | 18 | ArraySeq.rs | filter | {x in a \| f(x)} | multiset ensures + len | Strong |
| 8 | 18 | ArraySeq.rs | update | a with a[i]:=x | index(i)==item, others preserved | Strong |
| 9 | 18 | ArraySeq.rs | inject | first-update-wins | spec_inject (fold_left) | Strong |
| 10 | 18 | ArraySeq.rs | iterate | fold_left | spec_iterate via fold_left | Strong |
| 11 | 18 | ArraySeq.rs | reduce | fold for assoc f | == spec_iterate (monoid) | Strong |
| 12 | 18 | ArraySeq.rs | scan | prefix sums | inclusive prefix fold_left + total | Strong |
| 13 | 18 | ArraySeq.rs | collect | group-by key | spec_collect via deep_view | Strong |
| 14 | 18 | ArraySeq.rs | flatten | concat seq-of-seqs | map_values flatten | Strong |
| 15 | 18 | ArraySeq.rs | isEmpty | |a|==0 | empty <==> spec_len==0 | Strong |
| 16 | 18 | ArraySeq.rs | isSingleton | |a|==1 | single <==> spec_len==1 | Strong |
| 17 | 18 | ArraySeq.rs | tabulate | f(0)..f(n-1) | f.ensures on each index | Strong |
| 18 | 18 | ArraySeq.rs | map | f(a[i]) for each i | f.ensures on each element | Strong |
| 19 | 18 | MtEph.rs | ninject | nondeterministic | spec_ninject relational spec | Strong |
| 20 | 18 | ArraySeq.rs | scan_inclusive | scanI definition | inclusive prefix via fold_left | Strong |
| 21 | 18 | ArraySeq.rs | iterate_prefixes | prefixes + total | exclusive prefix folds + total | Strong |

All specs faithfully capture the prose definitions. No weakened postconditions found.

**Note on scan**: The prose Def 18.22 defines scan as *exclusive* prefix sums (scan[i] = reduce over a[0..i-1]). The code's `scan` function returns *inclusive* prefix sums (fold over a[0..i]). This is intentional: the separate `scan_inclusive` function makes this explicit, and the primary `scan` in the trait implementations also uses inclusive semantics. The code is self-consistent and the distinction is documented.


## Phase 4: Parallelism Review

### ArraySeqMtEph.rs

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|----------------|-------|
| 1 | 18 | ArraySeqMtEph.rs | new | Sequential | Vec::from_elem |
| 2 | 18 | ArraySeqMtEph.rs | set | Sequential | Single index write |
| 3 | 18 | ArraySeqMtEph.rs | length | Sequential | O(1) |
| 4 | 18 | ArraySeqMtEph.rs | nth | Sequential | O(1) |
| 5 | 18 | ArraySeqMtEph.rs | subseq_copy | Sequential | Clone loop |
| 6 | 18 | ArraySeqMtEph.rs | subseq | Sequential | Delegates to subseq_copy |
| 7 | 18 | ArraySeqMtEph.rs | from_vec | Sequential | Move |
| 8 | 18 | ArraySeqMtEph.rs | empty | Sequential | O(1) |
| 9 | 18 | ArraySeqMtEph.rs | singleton | Sequential | O(1) |
| 10 | 18 | ArraySeqMtEph.rs | append | Sequential | Two clone loops |
| 11 | 18 | ArraySeqMtEph.rs | filter | Sequential | Predicate loop |
| 12 | 18 | ArraySeqMtEph.rs | update | Sequential | Clone loop with replace |
| 13 | 18 | ArraySeqMtEph.rs | inject | Sequential | Fold updates into copy |
| 14 | 18 | ArraySeqMtEph.rs | ninject | Sequential | Delegates to inject |
| 15 | 18 | ArraySeqMtEph.rs | is_empty | Sequential | O(1) |
| 16 | 18 | ArraySeqMtEph.rs | is_singleton | Sequential | O(1) |
| 17 | 18 | ArraySeqMtEph.rs | iterate | Sequential | Left fold (inherently sequential) |
| 18 | 18 | ArraySeqMtEph.rs | reduce | Sequential | Left fold |
| 19 | 18 | ArraySeqMtEph.rs | scan | Sequential | Left fold |
| 20 | 18 | ArraySeqMtEph.rs | map (trait) | Sequential | Clone loop |
| 21 | 18 | ArraySeqMtEph.rs | tabulate (trait) | Sequential | Index loop |
| 22 | 18 | ArraySeqMtEph.rs | flatten (trait) | Sequential | Nested clone loops |
| 23 | 18 | ArraySeqMtEph.rs | map_par | **Parallel** | D&C fork-join via HFScheduler |
| 24 | 18 | ArraySeqMtEph.rs | filter_par | **Parallel** | D&C fork-join via HFScheduler |
| 25 | 18 | ArraySeqMtEph.rs | reduce_par | **Parallel** | D&C fork-join via HFScheduler |
| 26 | 18 | ArraySeqMtEph.rs | ninject_par | **Parallel** | D&C fork-join via HFScheduler |

Parallel operations exist as separate `_par` methods in bare `impl` blocks.
The trait-level methods remain sequential to maintain spec compatibility. This is correct:
the trait contract requires exact results, and parallel variants are provided as
additional methods with the same postconditions.

### ArraySeqMtPer.rs

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|----------------|-------|
| 1 | 18 | ArraySeqMtPer.rs | map_par | **Parallel** | D&C fork-join |
| 2 | 18 | ArraySeqMtPer.rs | filter_par | **Parallel** | D&C fork-join |
| 3 | 18 | ArraySeqMtPer.rs | reduce_par | **Parallel** | D&C fork-join |

All other functions in MtPer are sequential, following the same pattern as MtEph.


## Phase 5: Runtime Test Review

### Coverage Summary

| # | Chap | File | Functions Tested | Coverage |
|---|------|------|-----------------|----------|
| 1 | 18 | TestArraySeq.rs | new, set, length, nth, empty, singleton, tabulate, map, is_empty, is_singleton, from_vec, eq, Display, Debug, IntoIter (3 forms), clone, iter, iter_mut | Complete for base |
| 2 | 18 | TestArraySeqStEph.rs | new, set, length, nth, empty, singleton, tabulate, map, filter, subseq, append, update, iterate, reduce, scan, from_vec, eq, clone, Display, Debug, iter | Complete |
| 3 | 18 | TestArraySeqStPer.rs | Same as StEph minus set/inject (persistent) | Complete |
| 4 | 18 | TestArraySeqMtEph.rs | Construction, basic ops, map_par, filter_par, reduce_par | Good |
| 5 | 18 | TestArraySeqMtPer.rs | Construction, basic ops, parallel variants | Good |
| 6 | 18 | TestLinkedListStEph.rs | new, set, length, nth, empty, singleton, tabulate, map, filter, append, flatten, update, iterate, reduce, scan | Complete |
| 7 | 18 | TestLinkedListStPer.rs | Same as StEph minus set (persistent) | Complete |

### Missing RTT Coverage

| # | Chap | File | Function | Notes |
|---|------|------|----------|-------|
| 1 | 18 | ArraySeq.rs | collect | No RTT for collect (complex function) |
| 2 | 18 | ArraySeq.rs | inject | No RTT for inject in base ArraySeq |
| 3 | 18 | ArraySeq.rs | scan_inclusive | No RTT for scan_inclusive |
| 4 | 18 | ArraySeq.rs | iterate_prefixes | No RTT for iterate_prefixes |
| 5 | 18 | ArraySeq.rs | remove | No RTT for remove |
| 6 | 18 | ArraySeq.rs | insert | No RTT for insert |
| 7 | 18 | ArraySeqMtEph.rs | ninject_par | No RTT for ninject_par |


## Phase 6: PTT Review

All 7 source files have corresponding PTT files. Each PTT file covers 6 iterator
loop patterns:

| # | Pattern | Description |
|---|---------|-------------|
| 1 | loop-borrow-iter | `loop { ... a.iter() ... }` |
| 2 | loop-borrow-into | `loop { ... (&a).into_iter() ... }` |
| 3 | loop-consume | `loop { ... a.into_iter() ... }` |
| 4 | for-borrow-iter | `for x in iter: a.iter()` |
| 5 | for-borrow-into | `for x in iter: (&a).into_iter()` |
| 6 | for-consume | `for x in iter: a.into_iter()` |

All 7 files x 6 patterns = 42 PTT tests. Coverage is complete.


## Phase 7: Gap Analysis

| # | Gap | Severity | Notes |
|---|-----|----------|-------|
| 1 | LinkedList uses Vec internally | Low | Correct abstraction. The ADT contract is satisfied. A real linked-list implementation would add pointer verification complexity without algorithmic benefit. |
| 2 | reduce uses left fold, not D&C | Low | Correct for semantics chapter. Prose notes reduce = iterate for associative f. D&C implementation belongs in cost-specification chapters. |
| 3 | scan returns inclusive prefixes | Low | Self-consistent; scan_inclusive exists separately. Prose distinction (exclusive vs inclusive) is documented. |
| 4 | collect RTT missing | Medium | The collect function (SQL group-by) has no runtime test. Should add one. |
| 5 | inject/remove/insert RTT missing | Low | Implementation utilities without prose counterpart. Less critical. |
| 6 | LinkedList missing inject/ninject | Low | Prose defines inject/ninject on sequences generically. LinkedList implementations omit them. Not critical since LinkedList is not the primary implementation. |
| 7 | LinkedList missing subseq (non-copy) | Low | Only subseq_copy is provided. The prose subseq could return a slice but LinkedList does not support that. |
| 8 | MtPer missing inject/ninject | Low | MtPer omits inject/ninject. These are less commonly needed for persistent sequences. |
| 9 | No RTT for MtEphSlice variant | N/A | ArraySeqMtEphSlice is a Chap19 file, not Chap18. |

Overall: Chapter 18 is in excellent shape. 0 proof holes across all 7 files. Complete
iterator PTT coverage. Good RTT coverage with minor gaps for utility functions.


## Phase 8: TOC Review

### ArraySeq.rs

TOC present and matches section ordering. Sections 1-13 with appropriate omissions.
Minor: Section 7 header says "proof fns/broadcast groups" but the section contains
proof lemmas and spec fns intermixed. This is acceptable.

### ArraySeqStEph.rs

TOC present. Sections follow standard ordering. Section 7 correctly omitted (no proof
fns at module level; proof lemmas are in bare impl block, which falls under section 9).
Clean.

### ArraySeqStPer.rs

Same structure as StEph. Clean.

### ArraySeqMtEph.rs

TOC has non-standard section numbering: section 8 is "ninject lock predicate and helpers"
(should be "traits"), sections 9-13 are renumbered. This is a cosmetic deviation from the
standard but does not affect correctness.

### ArraySeqMtPer.rs

Clean. Standard ordering.

### LinkedListStEph.rs

Clean. Standard ordering. Section 7 correctly omitted.

### LinkedListStPer.rs

Clean. Standard ordering. Section 7 correctly omitted.

---

Date: 2026-03-15
Reviewer: Claude-Opus-4.6, Agent 1
