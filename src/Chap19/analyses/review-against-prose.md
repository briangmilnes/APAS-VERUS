<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap19 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-17
**Prose files:** `prompts/Chap19.txt`, `prompts/Chap19Scan.txt`
**Source files:** ArraySeqStEph.rs, ArraySeqStPer.rs, ArraySeqMtEph.rs

## Prose Inventory

Chapter 19 defines a "Parametric Implementation" of the Sequence ADT. The key
idea is that most operations can be built from a small set of primitives (nth,
length, subseq, tabulate, flatten, inject, ninject). The prose provides concrete
algorithms for each derived operation.

| # | Item | Type |
|---|------|------|
| 1 | Algorithm 19.1: empty via tabulate | Algorithm |
| 2 | Algorithm 19.2: singleton via tabulate | Algorithm |
| 3 | Algorithm 19.3: map via tabulate + nth | Algorithm |
| 4 | Algorithm 19.4: append via flatten (or tabulate + select) | Algorithm |
| 5 | Algorithm 19.5: filter via map + deflate + flatten | Algorithm |
| 6 | Algorithm 19.5: deflate helper | Algorithm |
| 7 | Algorithm 19.6: update via tabulate | Algorithm |
| 8 | Algorithm 19.7: isEmpty, isSingleton via length | Algorithm |
| 9 | Algorithm 19.8: iterate (recursive left fold) | Algorithm |
| 10 | Algorithm 19.9: reduce (divide-and-conquer) | Algorithm |
| 11 | Algorithm 19.10: scan (contraction, power-of-two) | Algorithm |
| 12 | Primitive functions: nth, length, subseq, tabulate, flatten | Primitives |
| 13 | Primitive functions: inject, ninject | Primitives |

## Code Inventory

| # | File | Lines | Parallel? | Proof holes | Independent? | Notes |
|---|------|-------|-----------|-------------|--------------|-------|
| 1 | ArraySeqStEph.rs | 1112 | No | 1 (assume in PartialEq) | Yes | St ephemeral variant |
| 2 | ArraySeqStPer.rs | 1122 | No | 1 (assume in PartialEq) | Yes | St persistent variant |
| 3 | ArraySeqMtEph.rs | 1434 | Yes | 1 (assume in PartialEq) | Yes | Mt ephemeral, map/filter/reduce_par |

Total: 3668 lines across 3 files. All modules are independent — no imports from
Chap18 or cross-Chap19 dependencies. Each module defines its own trait and
implementations.

## Prose-to-Code Mapping

| # | Prose Item | Code | Spec Fidelity | Notes |
|---|-----------|------|---------------|-------|
| 1 | Alg 19.1: empty | ArraySeqTrait::empty | Strong | Implemented directly (Vec::new), prose builds via tabulate |
| 2 | Alg 19.2: singleton | ArraySeqTrait::singleton | Strong | Implemented directly (Vec with one elem), prose builds via tabulate |
| 3 | Alg 19.3: map | ArraySeqTrait::map | Strong | Uses tabulate internally, matching prose |
| 4 | Alg 19.4: append | ArraySeqTrait::append | Strong | Direct concatenation; prose uses tabulate+select or flatten |
| 5 | Alg 19.5: filter | ArraySeqTrait::filter + deflate | Strong | filter builds on map(deflate)+flatten per prose |
| 6 | Alg 19.5: deflate | ArraySeqTrait::deflate | Strong | Returns singleton or empty based on predicate |
| 7 | Alg 19.6: update | ArraySeqTrait::update | Strong | Ephemeral: clone+set; Persistent: clone+modify; Prose uses tabulate |
| 8 | Alg 19.7: isEmpty | ArraySeqTrait::is_empty | Strong | Checks length == 0 |
| 9 | Alg 19.7: isSingleton | ArraySeqTrait::is_singleton | Strong | Checks length == 1 |
| 10 | Alg 19.8: iterate | iterate_iter (iterative) + iterate (recursive) | Strong | Both iterative and recursive forms provided |
| 11 | Alg 19.9: reduce | reduce_iter (iterative) + reduce (recursive D&C) | Strong | Both iterative and recursive forms provided |
| 12 | Alg 19.10: scan | ArraySeqTrait::scan | Partial | Iterative scan only (left fold). Prose defines contraction-based scan. Sequential O(n) scan is correct but not parallel. |
| 13 | Primitives: nth, length, subseq, tabulate, flatten | All implemented | Strong | Direct implementations |
| 14 | Primitives: inject | ArraySeqTrait::inject | Strong | Spec function spec_inject + verified implementation in all 3 modules |
| 15 | Primitives: ninject | ArraySeqMtEphTrait::ninject | Strong | spec_ninject + proof lemmas in MtEph; delegates to inject |

16 of 17 prose items implemented. 14 strong, 1 partial (scan), 0 not implemented.
ninject currently in MtEph only (delegates to inject); StEph/StPer omit ninject
since they have no thread model to exhibit non-determinism.

## Key Observations

### 1. Dual iterate/reduce implementations

Each file provides both an iterative form (`iterate_iter`, `reduce_iter`) and a
recursive form (`iterate`, `reduce`). The iterative forms use a while loop; the
recursive forms match the prose algorithms. This is a teaching choice — showing
both approaches for comparison.

### 2. Scan is sequential only

The prose Algorithm 19.10 presents scan using contraction (pair-and-recurse).
The code implements only an iterative left-fold scan with O(n) work and O(n) span.
The contraction-based scan would have O(n lg n) work but O(lg² n) span — better
parallelism. This is acceptable since Chapter 26 covers scan in detail with
divide-and-conquer implementations (ScanDCStPer, ScanDCMtPer).

### 3. inject and ninject now implemented

inject is implemented in all 3 modules with full spec_inject spec function and
verified implementation. MtEph additionally provides ninject (non-deterministic
inject) with spec_ninject, lemma_spec_inject_element, and lemma_spec_inject_len.
The ninject implementation delegates to inject — a valid choice since inject
produces a result that satisfies spec_ninject (every element is either the
original or came from some update).

### 4. into_iter ensures clauses fixed

StEph and StPer `into_iter` implementations lacked `ensures` clauses, causing
PTT failures for borrow_into and consume iterator patterns. Fixed by adding
ensures matching the MtEph pattern.

### 5. Missing iteratePrefixes, scan_inclusive, collect

These Chap18 ADT operations are not present in any Chap19 file. The prose does not
cover them in Chapter 19, so this is not a gap against the prose.

## Cost Analysis

All cost annotations are in dual APAS/Claude-Opus-4.6 format. Each function carries
two lines:

```
/// - APAS: Algorithm 19.X — description.
/// - Claude-Opus-4.6: Work Θ(...), Span Θ(...).
```

For primitives: `APAS: primitive (Section 19.2).`
For implementation utilities: `APAS: N/A — implementation utility, not in prose.`
Parallel variants: `APAS: parallel variant of Algorithm 19.X.`

## Parallelism Review

| # | Function | Module | APAS | Actual | Parallel? | Notes |
|---|----------|--------|------|--------|-----------|-------|
| 1 | map_par | MtEph | parallel 19.3 | Θ(lg\|a\|) span | Yes | HFScheduler fork per element |
| 2 | filter_par | MtEph | parallel 19.5 | Θ(lg²\|a\|) span | Yes | Parallel predicate eval, D&C collect |
| 3 | reduce_par | MtEph | parallel 19.9 | Θ(lg\|a\|) span | Yes | D&C via HFScheduler pool |

The Mt module provides genuine parallelism for map, filter, and reduce.
No parallel scan in Chap19 (covered in Chap26).

## Module Independence

All 3 modules are fully independent:

| # | Module | Imports from Chap18 | Cross-Chap19 deps | vstdplus deps |
|---|--------|:-------------------:|:------------------:|--------------|
| 1 | ArraySeqStEph | None | None | feq, multiset |
| 2 | ArraySeqStPer | None | None | feq, multiset |
| 3 | ArraySeqMtEph | None | None | clone_plus, feq, monoid, multiset |

## Runtime Test Review

| # | Source module | RTT file | Tests | Status |
|---|-------------|----------|-------|--------|
| 1 | ArraySeqStEph | TestArraySeqStEph.rs | 15 | Pass |
| 2 | ArraySeqStPer | TestArraySeqStPer.rs | 14 | Pass |
| 3 | ArraySeqMtEph | TestArraySeqMtEph.rs | 18 | Pass |

RTT total: 47 tests, all passing. Every source module has dedicated RTTs covering
basic operations, mutation, higher-order functions, parallel operations (MtEph),
equality, cloning, formatting, and iterators.

## Proof-Time Test Review

| # | Source module | PTT file | Tests | Registered? | Status |
|---|-------------|----------|-------|-------------|--------|
| 1 | ArraySeqStEph | ProveArraySeqStEph.rs | 6 | Yes | Pass |
| 2 | ArraySeqStPer | ProveArraySeqStPer.rs | 6 | Yes | Pass |
| 3 | ArraySeqMtEph | ProveArraySeqMtEph.rs | 6 | Yes | Pass |

PTT total: 18 tests, all passing. All registered in `rust_verify_test/Cargo.toml`.
PTTs were fixed: imports changed from Chap18 iterator types to Chap19 glob imports,
and StEph/StPer `into_iter` ensures clauses added to make borrow_into/consume
patterns verifiable.

## Proof Holes

| # | File | Hole | Count | Justification |
|---|------|------|-------|---------------|
| 1 | ArraySeqStEph.rs | assume in PartialEq | 1 | Standard leaf-type pattern |
| 2 | ArraySeqStPer.rs | assume in PartialEq | 1 | Standard leaf-type pattern |
| 3 | ArraySeqMtEph.rs | assume in PartialEq | 1 | Standard leaf-type pattern |

Total: 3 holes (all assume in PartialEq). Clean — all are the standard justified pattern.
12 clean proof functions (inject proofs, ninject proofs, inject lemmas).

## Gap Analysis

**Prose items partially implemented:**

| # | Prose item | Notes |
|---|-----------|-------|
| 1 | Alg 19.10: scan | Sequential only; contraction-based parallel scan deferred to Chap26 |

**Code with no prose counterpart:**

- `set` — mutating index write (ephemeral only, not in prose)
- `subseq_copy` — owned-copy variant of subseq
- `from_vec` — construction helper
- `iterate_iter`, `reduce_iter` — iterative variants of prose recursive algorithms
- `deflate` — extracted as trait method; prose defines it inline within filter
- `iter`, Clone, PartialEq, Eq, Debug, Display — Rust infrastructure
- `map_par`, `filter_par`, `reduce_par` — parallel variants (MtEph only)

## Table of Contents / In-Out Table

All 3 files have TOC comments. Sections follow standard order.

| # | File | Clone | PartialEq/Eq | Iterator | Debug | Display |
|---|------|:-----:|:------------:|:--------:|:-----:|:-------:|
| 1 | ArraySeqStEph | in | in | in | out | out |
| 2 | ArraySeqStPer | in | in | in | out | out |
| 3 | ArraySeqMtEph | in | in | in | out | out |

All placements correct. MtEph had Debug/Display behind `#[cfg(verus_keep_ghost)]`
which was removed (they belong outside verus but accessible to cargo tests).

## Summary

Chapter 19 provides a "parametric implementation" of the Sequence ADT where
derived operations are built from primitives. The code provides 3 implementations
(StEph, StPer, MtEph) with 3668 lines total:

- **Spec fidelity:** 16 of 17 prose items implemented. 14 strong, 1 partial (scan).
- **Module independence:** All 3 modules are fully independent.
- **Proof holes:** 3 total, all standard PartialEq assumes. 12 clean proof functions.
- **RTTs:** 47 tests, all passing. Every module has dedicated RTTs.
- **PTTs:** 18 tests, all passing. All registered and running.
- **Cost annotations:** All in dual APAS/Claude-Opus-4.6 format.
- **Parallelism:** MtEph provides parallel map, filter, reduce via HFScheduler.
- **TOC/in-out:** All correct.
- **Verification:** 1485 verified, 0 errors.

**Action items:**

| # | Priority | Item |
|---|----------|------|
| 1 | Low | Consider adding contraction-based parallel scan per Algorithm 19.10 (Chap26 already covers this) |
