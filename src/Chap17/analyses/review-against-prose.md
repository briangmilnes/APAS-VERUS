# Review Against Prose: Chapter 17 (Introduction / Mathematical Sequences)

## Phase 1: Inventory

### Source Files

| # | Chap | File | Fns(Tr) | Fns(IT) | Fns(IBI) | Fns(ML) | Holes |
|---|------|------|---------|---------|----------|---------|-------|
| 1 | 17 | MathSeq.rs | 18 | 20 | 1 | 0 | 0 |

Total: 21 exec functions, 0 proof holes, 2 info-only accepts (Clone/PartialEq eq workaround).

### RTT Files

| # | Chap | File | Tests |
|---|------|------|-------|
| 1 | 17 | TestMathSeq.rs | 25 |

### PTT Files

| # | Chap | File | Patterns |
|---|------|------|----------|
| 1 | 17 | ProveMathSeq.rs | (basic construction/iteration) |
| 2 | 17 | prove_MathSeq_iters.rs | 6 loop patterns |


## Phase 2: Prose Inventory

Chapter 17 is a definitions chapter. It defines what sequences are mathematically but
does not define any algorithms, cost specifications, or theorems requiring proof.

### Definitions Extracted

| # | Prose Ref | Name | Status |
|---|-----------|------|--------|
| 1 | Def 17.1 | Sequence as mapping from N to alpha, domain {0..n-1} | Implemented |
| 2 | Syntax 17.2 | Indexing a[i], subsequence a[l..h] | Implemented (nth, subseq) |
| 3 | Syntax 17.3 | Ordered pairs, strings | N/A (Rust native) |

### Algorithms Extracted

None. Chapter 17 defines no algorithms.

### Cost Specs Extracted

None. Chapter 17 defines no cost specifications.

### Theorems Extracted

None. Chapter 17 defines no theorems.


## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All exec functions in `MathSeq.rs` already have cost annotations in the form:
```
/// - APAS: no cost spec (definitions chapter).
/// - Claude-Opus-4.6: O(n) -- Vec allocation + clone fill.
```

No new cost annotations needed. All 18 trait functions and the `iter_mut` bare impl
function already carry cost comments.

### Phase 3b: Implementation Deviations

| # | Chap | File | Function | Deviation |
|---|------|------|----------|-----------|
| 1 | 17 | MathSeq.rs | domain() | Not in prose. Implementation utility returning indices. |
| 2 | 17 | MathSeq.rs | range() | Not in prose. Returns distinct values via hash set. |
| 3 | 17 | MathSeq.rs | multiset_range() | Not in prose. Returns (count, value) pairs via hash map. |
| 4 | 17 | MathSeq.rs | add_last() | Not in prose. Growable-vector utility. |
| 5 | 17 | MathSeq.rs | delete_last() | Not in prose. Growable-vector utility. |
| 6 | 17 | MathSeq.rs | set() | Not in prose. Mutable index write utility. |
| 7 | 17 | MathSeq.rs | with_len() | Not in prose. Delegates to new(). |
| 8 | 17 | MathSeq.rs | iter_mut() | Not in prose. Mutable iterator utility. |

All deviations are justified: MathSeq is the foundation type for the project's sequence
infrastructure. These utility functions support downstream chapters.

### Phase 3c: Spec Fidelity

| # | Chap | File | Function | Prose Spec | Code Ensures | Match? |
|---|------|------|----------|------------|--------------|--------|
| 1 | 17 | MathSeq.rs | new | N/A (no prose) | len == length, elements cloned from init | N/A |
| 2 | 17 | MathSeq.rs | nth | Def 17.1: a[i] | elem@ == self@[index] | Strong |
| 3 | 17 | MathSeq.rs | length | Def 17.1: |a| | len == spec_len | Strong |
| 4 | 17 | MathSeq.rs | empty | Implicit: |empty| == 0 | spec_len == 0 | Strong |
| 5 | 17 | MathSeq.rs | singleton | Implicit: |s|==1, s[0]==x | spec_len==1, @[0]==item@ | Strong |
| 6 | 17 | MathSeq.rs | subseq | Def 17.2: a[l..h] | subrange semantics | Strong |
| 7 | 17 | MathSeq.rs | subseq_copy | Def 17.2: a[l..h] | subrange semantics + copy | Strong |
| 8 | 17 | MathSeq.rs | is_empty | Implicit | emptiness == (len==0) | Strong |
| 9 | 17 | MathSeq.rs | is_singleton | Implicit | singularity == (len==1) | Strong |
| 10 | 17 | MathSeq.rs | from_vec | N/A | spec_seq == data@ | Strong |


## Phase 4: Parallelism Review

Not applicable. Chapter 17 has no Mt modules.


## Phase 5: Runtime Test Review

| # | Chap | File | Function | Tested? |
|---|------|------|----------|---------|
| 1 | 17 | MathSeq.rs | new | Yes (test_new) |
| 2 | 17 | MathSeq.rs | set | Yes (test_set) |
| 3 | 17 | MathSeq.rs | length | Yes (test_length) |
| 4 | 17 | MathSeq.rs | nth | Yes (test_nth) |
| 5 | 17 | MathSeq.rs | empty | Yes (test_empty) |
| 6 | 17 | MathSeq.rs | singleton | Yes (test_singleton) |
| 7 | 17 | MathSeq.rs | add_last | Yes (test_add_last) |
| 8 | 17 | MathSeq.rs | delete_last | Yes (test_delete_last) |
| 9 | 17 | MathSeq.rs | is_empty | Yes (test_is_empty) |
| 10 | 17 | MathSeq.rs | is_singleton | Yes (test_is_singleton) |
| 11 | 17 | MathSeq.rs | from_vec | Yes (test_from_vec) |
| 12 | 17 | MathSeq.rs | with_len | Yes (test_with_len) |
| 13 | 17 | MathSeq.rs | subseq | Yes (test_subseq) |
| 14 | 17 | MathSeq.rs | subseq_copy | Yes (test_subseq_copy) |
| 15 | 17 | MathSeq.rs | domain | Yes (test_domain) |
| 16 | 17 | MathSeq.rs | range | Yes (test_range) |
| 17 | 17 | MathSeq.rs | multiset_range | Yes (test_multiset_range) |
| 18 | 17 | MathSeq.rs | iter | Yes (test_iter) |
| 19 | 17 | MathSeq.rs | iter_mut | Yes (test_iter_mut) |
| 20 | 17 | MathSeq.rs | PartialEq | Yes (test_partial_eq) |
| 21 | 17 | MathSeq.rs | Clone | Implicit (in PartialEq test) |
| 22 | 17 | MathSeq.rs | Display | Yes (test_display) |
| 23 | 17 | MathSeq.rs | Debug | Yes (test_debug) |
| 24 | 17 | MathSeq.rs | IntoIter (ref) | Yes (test_into_iterator_by_ref) |
| 25 | 17 | MathSeq.rs | IntoIter (mut) | Yes (test_into_iterator_by_mut_ref) |
| 26 | 17 | MathSeq.rs | IntoIter (val) | Yes (test_into_iterator_by_value) |
| 27 | 17 | MathSeq.rs | MathSeqSLit! | Yes (test_macro_empty/repeat/list) |

Coverage: Complete. All functions and macros are tested.


## Phase 6: PTT Review

### ProveMathSeq.rs

Basic construction tests (not shown in detail, standard pattern).

### prove_MathSeq_iters.rs

| # | Pattern | Name | Status |
|---|---------|------|--------|
| 1 | loop-borrow-iter | mathseq_loop_borrow_iter | Covered |
| 2 | loop-borrow-into | mathseq_loop_borrow_into | Covered |
| 3 | loop-consume | mathseq_loop_consume | Covered |
| 4 | for-borrow-iter | mathseq_for_borrow_iter | Covered |
| 5 | for-borrow-into | mathseq_for_borrow_into | Covered |
| 6 | for-consume | mathseq_for_consume | Covered |

Coverage: All 6 iterator loop patterns are tested. Complete.


## Phase 7: Gap Analysis

| # | Gap | Severity | Notes |
|---|-----|----------|-------|
| 1 | No gaps found | N/A | Chapter 17 is a definitions chapter with no algorithms to verify. |

Chapter 17 is fully verified (0 holes). All functions have specs. All are tested (RTT + PTT).
The 2 info-level accepts are Clone/PartialEq workaround pattern (approved project pattern).


## Phase 8: TOC Review

The file has a Table of Contents at the top of the file:

```
//  Table of Contents
//  1. module
//  3. broadcast use
//  4. type definitions
//  5. view impls
//  6. spec fns
//  8. traits
//  9. impls
//  10. iterators
//  11. derive impls in verus!
//  12. macros
//  13. derive impls outside verus!
```

Observations:
- Section 2 (imports) is omitted from the TOC but imports appear inside verus! (use statements at top of module). Minor inconsistency.
- Section 7 (proof fns/broadcast groups) correctly omitted (no proof fns in this module).
- Duplicate TOC comment blocks inside verus! (lines 52-61 repeat the TOC). Minor style issue.
- Sections 12 and 13 ordering is inverted in the outside-verus! region (macros appear after Debug/Display). The TOC says section 12 (macros) is before section 13 (derive impls outside verus!), but in the actual code the macros section appears after Debug/Display impls.

These are cosmetic issues only. No functional impact.

---

Date: 2026-03-15
Reviewer: Claude-Opus-4.6, Agent 1
