<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap17 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-17
**Last mechanical audit:** 2026-02-19 — full review regeneration; proof holes log updated.
**Prose file:** `prompts/Chap17.txt`
**Source files:** `MathSeq.rs`

## Prose Inventory

Chapter 17 is the introduction to the Sequences ADT. Purely definitional —
no algorithms, no cost tables.

| # | Item | Type |
|---|------|------|
| 1 | Def 17.1: Sequence — mapping from N to alpha with dense domain {0..n-1} | Definition |
| 2 | Syntax 17.2: Angle-bracket notation, indexing a[i], subrange a[l..h] | Syntax |
| 3 | Syntax 17.3: Ordered pairs and strings | Syntax |
| 4 | Ex 17.1-17.3: Example sequences, pairs, strings | Examples |

## Code Inventory

| # | File | Lines | Parallel? | Proof holes | Notes |
|---|------|-------|-----------|-------------|-------|
| 1 | MathSeq.rs | 439 | No | 2 assume() (Clone, PartialEq), 1 bare_impl | Vec-backed sequence ADT. bare_impl: iter_mut — must stay outside verus! per style rule. |

## Prose-to-Code Mapping

| # | Prose Item | Code | Spec Fidelity |
|---|-----------|------|---------------|
| 1 | Def 17.1: Sequence (dense domain 0..n-1) | MathSeqS struct (Vec-backed) | Strong — View maps to Seq<T::V>, dense domain guaranteed by Vec |
| 2 | Syntax 17.2: Indexing a[i] | nth(index) | Strong — requires index < spec_len, ensures elem@ == self@[index] |
| 3 | Syntax 17.2: Subrange a[l..h] | subseq(start, length), subseq_copy(start, length) | Strong — subseq returns slice, subseq_copy returns owned MathSeqS |
| 4 | Syntax 17.3: Ordered pairs | Pair<T,U> from Types module | Exists elsewhere |
| 5 | Syntax 17.3: Strings | Rust String / char sequences | Native Rust |

## Cost Analysis

APAS provides no cost specs for Chap17 — this is the introductory definitions
chapter. Cost specifications come in a later chapter. All costs below are
Claude-Opus-4.6's analysis of the Vec-backed implementation.

| # | Function | Claude-Opus-4.6 | Notes |
|---|----------|-----------------|-------|
| 1 | new(length, init_value) | O(n) | Vec allocation + clone fill |
| 2 | set(index, value) | O(1) | Direct index write |
| 3 | length() | O(1) | Vec::len |
| 4 | nth(index) | O(1) | Direct index read |
| 5 | empty() | O(1) | Empty Vec allocation |
| 6 | singleton(item) | O(1) | Single-element Vec |
| 7 | add_last(value) | Amortized O(1) | Vec::push |
| 8 | delete_last() | O(1) | Vec::pop |
| 9 | is_empty() | O(1) | Length check |
| 10 | is_singleton() | O(1) | Length check |
| 11 | from_vec(data) | O(1) | Move, no copy |
| 12 | with_len(length, init) | O(n) | Delegates to new |
| 13 | subseq(start, length) | O(1) | Returns slice reference |
| 14 | subseq_copy(start, length) | O(length) | Copies subrange |
| 15 | domain() | O(n) | Builds Vec of indices |
| 16 | range() | O(n) expected | Hash set dedup, O(n) with good hash |
| 17 | multiset_range() | O(n) expected | Hash map counting, two passes |

## Parallelism Review

Not applicable — MathSeqS is a single-threaded data structure. No Mt variant
exists for Chap17. (Chap18 provides ArraySeq with Mt variants.)

## Runtime Test Review

| # | Source module | RTT file | PTT file | Status |
|---|-------------|----------|----------|--------|
| 1 | MathSeq | TestMathSeq.rs (26 tests) | ProveMathSeq.rs (6 tests), prove_MathSeq_iters.rs (3 tests) | Both exist |

### RTT Coverage

Excellent — 26 tests covering: new, set, length, nth, empty, singleton,
add_last, delete_last, is_empty, is_singleton, from_vec, with_len, subseq,
subseq_copy, domain, range, multiset_range, iter, iter_mut, into_iter
(by ref, by mut ref, by value), PartialEq, Display, Debug, and the
MathSeqSLit! macro (empty, repeat, list forms).

### PTT Coverage

ProveMathSeq.rs tests 6 loop forms:
- loop-borrow-iter, loop-borrow-into, loop-consume
- for-borrow-iter, for-borrow-into, for-consume

prove_MathSeq_iters.rs tests 3 additional patterns:
- consuming for loop, range-based for, nested for loops

All PTTs pass. `MathSeqIter<T>`, `iter_invariant`, and ghost iterator
infrastructure are now defined in `MathSeq.rs`, implementing the full
collection iterator standard (collection-iterators.mdc). PTTs registered
in `rust_verify_test/Cargo.toml`.

## Gap Analysis

**Prose items with no implementation:**

| # | Prose item | Notes |
|---|-----------|-------|
| 1 | Ex 17.1: Non-sequence (gap in domain) | Structural — Vec enforces density by construction |
| 2 | Lambda sequences | Not representable in this ADT (would need closure-backed variant) |

No real gaps — Chap17 is definitional and the code faithfully implements
Definition 17.1 as a dense Vec-backed sequence.

**Code with no prose counterpart:**

- `range()` — returns unique elements (set of values)
- `multiset_range()` — returns elements with counts
- `domain()` — returns index vector (trivially {0..n-1} for Vec)
- Iterator implementations (MathSeqIter, IntoIter, IterMut, ghost iter infrastructure)
- Clone, PartialEq, Eq, Display, Debug
- MathSeqSLit! macro
- `spec_clamp` helper

## Proof Holes

| # | File | Hole | Count | Justification |
|---|------|------|-------|---------------|
| 1 | MathSeq.rs | assume(cloned@ == self@) in Clone::clone | 1 | Vec::clone view equality — Verus cannot prove cloned@ == self@ through Vec's external specs. |
| 2 | MathSeq.rs | assume(r == (self@ == other@)) in PartialEq::eq | 1 | Leaf type wrapping Vec — Verus cannot resolve eq_spec through trait machinery. Standard pattern per partialeq-eq-pattern rule. |
| 3 | MathSeq.rs | bare_impl (iter_mut) | 1 | Exception: iter_mut returns &mut; must stay outside verus! per style rule. Not a violation. |

The bare_impl detector flags `impl MathSeqS` for iter_mut, but this is the
required exception — `&mut`-returning methods cannot be inside verus!.

## Style Notes

1. **Clone** — inside `verus!` with assume for view equality. Correct.
2. **PartialEq/Eq/PartialEqSpecImpl** — inside `verus!`. Correct.
3. **IntoIterator for &** and consuming **IntoIterator** — inside `verus!`. Correct.
4. **iter()** — inside `verus!` with ensures clause. Returns `MathSeqIter`.
5. **iter_mut()**, **IntoIterator for &mut** — outside `verus!`.
   Required: Verus does not support `&mut` in return position or trait impls.
6. **Debug/Display** — outside `verus!`. Correct per style rule.
7. **Iterator standard:** MathSeq now implements the full collection iterator
   standard (collection-iterators.mdc): `MathSeqIter` custom struct, `View`,
   `iter_invariant`, `Iterator::next` with ensures, `MathSeqGhostIter`,
   `ForLoopGhostIteratorNew`, `ForLoopGhostIterator` — all inside `verus!`.

## Summary

Chap17 introduces the Sequence ADT (Def 17.1). The code provides a faithful
Vec-backed implementation with strong specs on all operations:

- **MathSeqS<T>** — 15+ operations with ensures/requires, View maps to Seq<T::V>
- Excellent RTT coverage (26 tests) including iterators, macros, traits
- PTT coverage: 9 tests across 2 files, all passing
- Full collection iterator standard implemented (MathSeqIter, ghost iter, etc.)
- Two proof holes: `assume()` in Clone and PartialEq — standard leaf-type patterns
- No APAS cost specs to compare against (definitions-only chapter)

This is a well-implemented building block used extensively by Chap18 (ArraySeq)
and later chapters.
