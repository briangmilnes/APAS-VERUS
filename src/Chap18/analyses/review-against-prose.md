<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap18 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-15
**Prose file:** `prompts/Chap18.txt`
**Source files:** ArraySeq.rs, ArraySeqStEph.rs, ArraySeqStPer.rs, ArraySeqMtEph.rs, ArraySeqMtPer.rs, LinkedListStEph.rs, LinkedListStPer.rs

## Prose Inventory

Chapter 18 defines the Sequence ADT (Data Type 18.1) with full semantics.
No explicit cost table — costs are implementation-dependent and appear in
later chapters (Chap26 for reduce/scan).

| # | Item | Type |
|---|------|------|
| 1 | Data Type 18.1: Sequence ADT — 18 operations | Definition |
| 2 | Def 18.3: length, nth | Definition |
| 3 | Def 18.4: empty, singleton | Definition |
| 4 | Def 18.5: isEmpty, isSingleton | Definition |
| 5 | Def 18.6: tabulate | Definition |
| 6 | Def 18.8: map | Definition |
| 7 | Def 18.10: filter | Definition |
| 8 | Def 18.12: subseq | Definition |
| 9 | Def 18.13: append | Definition |
| 10 | Def 18.14: flatten | Definition |
| 11 | Def 18.15: update | Definition |
| 12 | Def 18.16: inject | Definition |
| 13 | Def 18.17: ninject (nondeterministic inject) | Definition |
| 14 | Def 18.18: collect | Definition |
| 15 | Def 18.19: iterate, iteratePrefixes | Definition |
| 16 | Def 18.20: associative function | Definition |
| 17 | Def 18.21: reduce | Definition |
| 18 | Def 18.22: scan, scanI (inclusive) | Definition |
| 19 | Ex 18.1: Rightmost positive (iterate solution) | Exercise |
| 20 | Ex 18.2: iterate vs reduce difference | Exercise |
| 21 | Ex 18.3: Why use reduce over iterate | Exercise |

## Code Inventory

| # | File | Lines | Parallel? | Proof holes | Notes |
|---|------|-------|-----------|-------------|-------|
| 1 | ArraySeq.rs | 1633 | No | 3 (1 assume, 2 external) | Base trait + impl with 27+ exec fns, iterator, collect |
| 2 | ArraySeqStEph.rs | 933 | No | 1 (assume in PartialEq) | St ephemeral variant, full iterator standard |
| 3 | ArraySeqStPer.rs | 908 | No | 1 (assume in PartialEq) | St persistent variant, full iterator standard |
| 4 | ArraySeqMtEph.rs | 1151 | Yes | 1 (assume in PartialEq) | Mt ephemeral, map_par/filter_par/reduce_par |
| 5 | ArraySeqMtPer.rs | 1160 | Yes | 1 (assume in PartialEq) | Mt persistent, map_par/filter_par/reduce_par |
| 6 | LinkedListStEph.rs | 858 | No | 1 (assume in PartialEq) | LinkedList-backed sequence |
| 7 | LinkedListStPer.rs | 842 | No | 1 (assume in PartialEq) | LinkedList persistent variant |

## Prose-to-Code Mapping

| # | Prose Item | Code | Spec Fidelity |
|---|-----------|------|---------------|
| 1 | length | ArraySeqTrait::length | Strong |
| 2 | nth | ArraySeqTrait::nth | Strong |
| 3 | empty | ArraySeqTrait::empty | Strong |
| 4 | singleton | ArraySeqTrait::singleton | Strong |
| 5 | tabulate | module fn tabulate | Strong |
| 6 | map | module fn map | Strong |
| 7 | filter | ArraySeqTrait::filter | Strong |
| 8 | subseq | ArraySeqTrait::subseq, subseq_copy | Strong |
| 9 | append | ArraySeqTrait::append | Strong |
| 10 | flatten | module fn flatten | Strong |
| 11 | update | ArraySeqTrait::update | Strong |
| 12 | inject | ArraySeqTrait::inject | Strong — first-update-wins semantics |
| 13 | ninject | Not implemented | Gap — nondeterministic inject |
| 14 | collect | ArraySeqTrait::collect | Strong |
| 15 | iterate | ArraySeqTrait::iterate | Strong |
| 16 | iteratePrefixes | module fn iterate_prefixes | Strong |
| 17 | reduce | ArraySeqTrait::reduce | Strong |
| 18 | scan | ArraySeqTrait::scan | Strong |
| 19 | scanI | ArraySeqTrait::scan_inclusive | Strong |
| 20 | isEmpty | ArraySeqTrait::is_empty | Strong |
| 21 | isSingleton | ArraySeqTrait::is_singleton | Strong |

## Cost Analysis

APAS provides no explicit cost table for Chap18 — it defines semantics only.
The prose notes that tabulate, map, filter can apply f in parallel; iterate is
sequential; reduce has logarithmic span; scan matches reduce's span.

Cost annotations exist in ArraySeq.rs and the 4 ArraySeq variants (old
single-line `/// - Work` format). LinkedList files have no cost annotations.

**Format issue:** All cost annotations use old single-line format
(`/// - Work Θ(...), Span Θ(...)`) instead of the dual
APAS/Claude-Opus-4.6 format. Since APAS provides no explicit costs for
this chapter, the correct annotation is:

```
/// - APAS: no cost spec (semantics-only chapter).
/// - Claude-Opus-4.6: Work Θ(...), Span Θ(...).
```

**Action items:**
1. Convert 96 old-format annotations across 5 ArraySeq files to dual format
2. Add cost annotations to LinkedListStEph.rs and LinkedListStPer.rs

## Parallelism Review

| # | Function | Module | APAS Span | Actual | Parallel? | Notes |
|---|----------|--------|-----------|--------|-----------|-------|
| 1 | map_par | MtEph, MtPer | N/A | Θ(f cost) | Yes | Uses HFScheduler pool, fork per element |
| 2 | filter_par | MtEph, MtPer | N/A | Θ(pred cost) | Yes | Parallel predicate eval, sequential collect |
| 3 | reduce_par | MtEph, MtPer | N/A | Θ(log n × f cost) | Yes | Divide-and-conquer via pool |

APAS defines no cost specs for Chap18, so all APAS Span values are N/A.
The Mt modules provide genuine parallelism for map, filter, and reduce.
No parallel scan, inject, iterate, flatten, or collect in Mt variants.

## Runtime Test Review

| # | Source module | RTT file | Status |
|---|-------------|----------|--------|
| 1 | ArraySeq | TestArraySeq.rs (21 tests) | Pass |
| 2 | ArraySeqStEph | None | Missing |
| 3 | ArraySeqStPer | None | Missing |
| 4 | ArraySeqMtEph | TestArraySeqMtEph.rs (10 tests) | Pass |
| 5 | ArraySeqMtPer | TestArraySeqMtPer.rs (10 tests) | Pass |
| 6 | LinkedListStEph | None | Missing |
| 7 | LinkedListStPer | None | Missing |

RTT total: 41 tests, all pass. Missing RTTs for 4 modules. The base
ArraySeq test exercises the trait, so StEph/StPer get indirect coverage
through it. LinkedList variants have no RTT coverage at all.

## Proof-Time Test Review

| # | Source module | PTT file | Tests | Status |
|---|-------------|----------|-------|--------|
| 1 | ArraySeq | ProveArraySeq.rs | 10 | 5 pass, 5 fail |
| 2 | ArraySeqStEph | ProveArraySeqStEph.rs | 2 | Pass |
| 3 | ArraySeqStPer | ProveArraySeqStPer.rs | 2 | Pass |
| 4 | ArraySeqMtEph | ProveArraySeqMtEph.rs | 6 | Pass |
| 5 | ArraySeqMtPer | ProveArraySeqMtPer.rs | 2 | Pass |
| 6 | LinkedListStEph | ProveLinkedListStEph.rs | 2 | Pass |
| 7 | LinkedListStPer | ProveLinkedListStPer.rs | 2 | Pass |

PTT total: 21 tests (16 pass, 5 fail).

**ProveArraySeq failures (5):** `arrayseq_iterate`, `arrayseq_iterate_prefixes`,
`arrayseq_reduce`, `arrayseq_scan`, `arrayseq_scan_inclusive`. All fail on
the closure ensures precondition (`f.ensures((&a, &t), ret) <==> ret == spec_f(a, t)`).
This is likely a Verus version regression in closure specification handling.

## Gap Analysis

**Prose items with no implementation:**

| # | Prose item | Notes |
|---|-----------|-------|
| 1 | Def 18.17: ninject (nondeterministic inject) | Not implemented. Would need nondeterministic semantics. |
| 2 | Ex 18.1: Rightmost positive | Exercise — not implemented as standalone algorithm |
| 3 | Ex 18.2-18.3: iterate vs reduce exercises | Conceptual — no code expected |

**Code with no prose counterpart:**

- `set` — mutating index write (not in ADT definition)
- `subseq_copy` — owned copy of subsequence (prose only defines subseq)
- `remove`, `insert` — standard Vec operations not in ADT
- `from_vec` — construction helper
- `find_key` — collect helper
- `iter`, `iter_mut` — Rust iteration infrastructure
- Clone, PartialEq, Eq, Debug, Display
- All St/Mt variant modules (prose defines one abstract ADT; code provides 6 concrete implementations)

## Proof Holes

| # | File | Hole | Count | Justification |
|---|------|------|-------|---------------|
| 1 | ArraySeq.rs | assume(equal == (self@ == other@)) in PartialEq | 1 | Standard leaf-type pattern |
| 2 | ArraySeq.rs | #[verifier::external] on iter_mut impl | 1 | Verus &mut limitation |
| 3 | ArraySeq.rs | #[verifier::external] on IntoIterator for &mut | 1 | Verus &mut limitation |
| 4 | ArraySeqStEph.rs | assume in PartialEq | 1 | Standard pattern |
| 5 | ArraySeqStPer.rs | assume in PartialEq | 1 | Standard pattern |
| 6 | ArraySeqMtEph.rs | assume in PartialEq | 1 | Standard pattern |
| 7 | ArraySeqMtPer.rs | assume in PartialEq | 1 | Standard pattern |
| 8 | LinkedListStEph.rs | assume in PartialEq | 1 | Standard pattern |
| 9 | LinkedListStPer.rs | assume in PartialEq | 1 | Standard pattern |

Total: 9 holes (7 assume in PartialEq, 2 external for &mut). All justified.

## Table of Contents / In-Out Table

All 7 files have TOC comments and section headers. Sections are in standard order.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | ArraySeq | ✅ in | ✅ in | - | - | ✅ in | ✅ out | ✅ out | - | iter_mut, IntoIterator &mut: ✅ out (external) |
| 2 | ArraySeqStEph | ✅ in | ✅ in | - | - | ✅ in | ✅ out | ✅ out | - | - |
| 3 | ArraySeqStPer | ✅ in | ✅ in | - | - | ✅ in | ✅ out | ✅ out | - | - |
| 4 | ArraySeqMtEph | ✅ in | ✅ in | - | - | ✅ in | ✅ out | ✅ out | - | - |
| 5 | ArraySeqMtPer | ✅ in | ✅ in | - | - | ✅ in | ✅ out | ✅ out | - | - |
| 6 | LinkedListStEph | ✅ in | ✅ in | - | - | ✅ in | ✅ out | ✅ out | - | - |
| 7 | LinkedListStPer | ✅ in | ✅ in | - | - | ✅ in | ✅ out | ✅ out | - | - |

All placements correct. No action items.

## Summary

Chap18 defines the Sequence ADT with 18 operations. The code provides
7 implementations (1 base + 4 ArraySeq variants + 2 LinkedList variants)
with 6000+ lines total:

- **Spec fidelity:** 19 of 20 prose operations implemented. Only `ninject` missing.
- **Proof holes:** 9 total, all justified (7 PartialEq assume, 2 &mut external)
- **RTTs:** 41 tests pass, but 4 modules lack dedicated RTTs (StEph, StPer, both LinkedList)
- **PTTs:** 16 pass, 5 fail (closure spec regression in ProveArraySeq)
- **Cost annotations:** Exist in old single-line format; need dual APAS/Claude-Opus conversion.
  LinkedList files have no annotations at all.
- **Parallelism:** Mt modules provide genuine parallel map, filter, reduce via HFScheduler
- **TOC/in-out:** All correct

**Action items:**

| # | Priority | Item |
|---|----------|------|
| 1 | High | Fix 5 failing ProveArraySeq PTTs (closure ensures regression) |
| 2 | Medium | Convert 96 cost annotations to dual APAS/Claude-Opus format |
| 3 | Medium | Add cost annotations to LinkedListStEph.rs and LinkedListStPer.rs |
| 4 | Low | Add RTTs for StEph, StPer, LinkedListStEph, LinkedListStPer |
| 5 | Low | Consider implementing ninject |
