<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap18 Review Against Prose

**Reviewer:** Claude-Opus-4.6
**Date:** 2026-02-15 (updated 2026-02-16)
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

| # | File | Lines | Parallel? | Proof holes | Independent? | Notes |
|---|------|-------|-----------|-------------|--------------|-------|
| 1 | ArraySeq.rs | 1641 | No | 3 (1 assume, 2 external) | N/A (base) | Base trait + impl, iterator, collect |
| 2 | ArraySeqStEph.rs | 1062 | No | 1 (assume in PartialEq) | Yes | St ephemeral variant |
| 3 | ArraySeqStPer.rs | 1036 | No | 1 (assume in PartialEq) | Yes | St persistent variant |
| 4 | ArraySeqMtEph.rs | 1574 | Yes | 8 (7 ninject_par + 1 PartialEq) | Yes | Mt ephemeral, map/filter/reduce/ninject_par |
| 5 | ArraySeqMtPer.rs | 1195 | Yes | 1 (assume in PartialEq) | Yes | Mt persistent, map/filter/reduce_par |
| 6 | LinkedListStEph.rs | 898 | No | 1 (assume in PartialEq) | Yes | LinkedList-backed sequence |
| 7 | LinkedListStPer.rs | 881 | No | 1 (assume in PartialEq) | Yes | LinkedList persistent variant |

Total: 8287 lines across 7 files. All 6 variant modules are independent of ArraySeq.rs — each
defines its own `spec_iterate`, `spec_inject` (where needed), and imports `spec_monoid` from
`vstdplus/monoid.rs`.

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
| 13 | ninject | ArraySeqMtEphTrait::ninject, ninject_par | Strong — spec_ninject allows any update to win; ninject_par uses Arc\<RwLock\> + HF join for real thread-racing nondeterminism |
| 14 | collect | ArraySeqTrait::collect | Strong |
| 15 | iterate | ArraySeqTrait::iterate | Strong |
| 16 | iteratePrefixes | module fn iterate_prefixes | Strong |
| 17 | reduce | ArraySeqTrait::reduce | Strong |
| 18 | scan | ArraySeqTrait::scan | Strong |
| 19 | scanI | ArraySeqTrait::scan_inclusive | Strong |
| 20 | isEmpty | ArraySeqTrait::is_empty | Strong |
| 21 | isSingleton | ArraySeqTrait::is_singleton | Strong |

All 20 implementable prose items are now implemented. 20 of 20 strong spec fidelity.

## Cost Analysis

APAS provides no explicit cost table for Chap18 — it defines semantics only.
The prose notes that tabulate, map, filter can apply f in parallel; iterate is
sequential; reduce has logarithmic span; scan matches reduce's span.

All 7 source files now carry dual APAS/Claude-Opus-4.6 cost annotations. Since APAS
provides no explicit cost table for Chapter 18 (semantics-only chapter), every operation
is annotated as:

```
/// - APAS: no cost spec (semantics-only chapter).
/// - Claude-Opus-4.6: Work Θ(...), Span Θ(...).
```

Implementation-only utilities (`set`, `subseq_copy`, `from_vec`, etc.) use
`/// - APAS: N/A — implementation utility, not in prose.`

## Parallelism Review

| # | Function | Module | APAS Span | Actual | Parallel? | Notes |
|---|----------|--------|-----------|--------|-----------|-------|
| 1 | map_par | MtEph, MtPer | N/A | Θ(f cost) | Yes | Uses HFScheduler pool, fork per element |
| 2 | filter_par | MtEph, MtPer | N/A | Θ(pred cost) | Yes | Parallel predicate eval, sequential collect |
| 3 | reduce_par | MtEph, MtPer | N/A | Θ(log n × f cost) | Yes | Divide-and-conquer via pool |
| 4 | ninject_par | MtEph | N/A | Θ(\|updates\|) | Yes | Arc\<RwLock\> + HF join; two threads race for single lock |

APAS defines no cost specs for Chap18, so all APAS Span values are N/A.
The Mt modules provide genuine parallelism for map, filter, reduce, and ninject.
No parallel scan, inject, iterate, flatten, or collect in Mt variants.

### ninject_par Architecture

The parallel ninject uses a single `vstd::rwlock::RwLock` protecting the result
buffer, wrapped in `Arc` for sharing across threads. Two threads forked via
`HFSchedulerMtEph::join` each acquire write access, apply their half of updates,
and release. Thread scheduling determines which writer goes last at each
conflicting position — that is the source of nondeterminism.

The lock invariant (`NinjectInv`) guarantees that at every acquire/release
boundary, the buffer satisfies `spec_ninject`: each element is either the
original or came from some update. The invariant proof is fully verified;
the assumes are limited to clone-view preservation and Arc pred opacity.

## Module Independence

All 6 variant modules are independent of `ArraySeq.rs`. Each defines its own
copy of the spec functions it needs:

| # | Module | Own spec_iterate | Own spec_inject | Imports from ArraySeq |
|---|--------|:----------------:|:---------------:|:---------------------:|
| 1 | ArraySeqStEph | Yes | Yes | None |
| 2 | ArraySeqStPer | Yes | Yes | None |
| 3 | ArraySeqMtEph | Yes | Yes | None |
| 4 | ArraySeqMtPer | Yes | No | None |
| 5 | LinkedListStEph | Yes | No | None |
| 6 | LinkedListStPer | Yes | No | None |

`spec_monoid` lives in `src/vstdplus/monoid.rs` and is imported by all modules
that need it (ArraySeq, all 6 variants, Chap19, Chap26).

## Runtime Test Review

| # | Source module | RTT file | Tests | Status |
|---|-------------|----------|-------|--------|
| 1 | ArraySeq | TestArraySeq.rs | 21 | Pass |
| 2 | ArraySeqStEph | TestArraySeqStEph.rs | 15 | Pass |
| 3 | ArraySeqStPer | TestArraySeqStPer.rs | 14 | Pass |
| 4 | ArraySeqMtEph | TestArraySeqMtEph.rs | 10 | Pass |
| 5 | ArraySeqMtPer | TestArraySeqMtPer.rs | 10 | Pass |
| 6 | LinkedListStEph | TestLinkedListStEph.rs | 15 | Pass |
| 7 | LinkedListStPer | TestLinkedListStPer.rs | 14 | Pass |

RTT total: 99 tests, all pass. Every source module has dedicated RTTs covering
construction, access, mutation, equality, cloning, display, iteration, and
higher-order operations.

## Proof-Time Test Review

| # | Source module | PTT file | Tests | Status |
|---|-------------|----------|-------|--------|
| 1 | ArraySeq | ProveArraySeq.rs | 10 | Pass |
| 2 | ArraySeqStEph | ProveArraySeqStEph.rs | 6 | Pass |
| 3 | ArraySeqStPer | ProveArraySeqStPer.rs | 2 | Pass |
| 4 | ArraySeqMtEph | ProveArraySeqMtEph.rs | 2 | Pass |
| 5 | ArraySeqMtPer | ProveArraySeqMtPer.rs | 2 | Pass |
| 6 | LinkedListStEph | ProveLinkedListStEph.rs | 2 | Pass |
| 7 | LinkedListStPer | ProveLinkedListStPer.rs | 2 | Pass |

PTT total: 26 tests, all pass. ProveArraySeq required adding
`use apas_verus::vstdplus::monoid::monoid::*` to 3 tests after
`spec_monoid` moved to vstdplus.

## Gap Analysis

**Prose items with no implementation:**

| # | Prose item | Notes |
|---|-----------|-------|
| 1 | Ex 18.1: Rightmost positive | Exercise — not implemented as standalone algorithm |
| 2 | Ex 18.2-18.3: iterate vs reduce exercises | Conceptual — no code expected |

**Code with no prose counterpart:**

- `set` — mutating index write (not in ADT definition)
- `subseq_copy` — owned copy of subsequence (prose only defines subseq)
- `remove`, `insert` — standard Vec operations not in ADT
- `from_vec` — construction helper
- `find_key` — collect helper
- `iter`, `iter_mut` — Rust iteration infrastructure
- Clone, PartialEq, Eq, Debug, Display
- All St/Mt variant modules (prose defines one abstract ADT; code provides 6 concrete implementations)
- `NinjectInv`, `apply_ninject_updates` — RwLock infrastructure for parallel ninject

## Proof Holes

| # | File | Hole | Count | Justification |
|---|------|------|-------|---------------|
| 1 | ArraySeq.rs | assume in PartialEq | 1 | Standard leaf-type pattern |
| 2 | ArraySeq.rs | #[verifier::external] on iter_mut impl | 1 | Verus &mut limitation |
| 3 | ArraySeq.rs | #[verifier::external] on IntoIterator for &mut | 1 | Verus &mut limitation |
| 4 | ArraySeqStEph.rs | assume in PartialEq | 1 | Standard pattern |
| 5 | ArraySeqStPer.rs | assume in PartialEq | 1 | Standard pattern |
| 6 | ArraySeqMtEph.rs | assume in PartialEq | 1 | Standard pattern |
| 7 | ArraySeqMtEph.rs | assume(val == updates@[i].1) in apply_ninject_updates | 1 | Clone preserves value |
| 8 | ArraySeqMtEph.rs | assume(buf@ =~= a.seq@) in ninject_par | 1 | Vec::clone preserves view |
| 9 | ArraySeqMtEph.rs | assume(lock.pred() == pred) in ninject_par | 1 | Arc::new preserves RwLock pred |
| 10 | ArraySeqMtEph.rs | assume((pos, val) == updates@[k]) in ninject_par | 1 | Tuple clone preserves value |
| 11 | ArraySeqMtEph.rs | assume(lock1.pred() == pred) in ninject_par | 1 | Arc::clone preserves RwLock pred |
| 12 | ArraySeqMtEph.rs | assume(lock2.pred() == pred) in ninject_par | 1 | Arc::clone preserves RwLock pred |
| 13 | ArraySeqMtEph.rs | assume(r@ =~= result_vec@) in ninject_par | 1 | Vec::clone preserves view |
| 14 | ArraySeqMtPer.rs | assume in PartialEq | 1 | Standard pattern |
| 15 | LinkedListStEph.rs | assume in PartialEq | 1 | Standard pattern |
| 16 | LinkedListStPer.rs | assume in PartialEq | 1 | Standard pattern |

Total: 16 holes (14 assume, 2 external).

- 7 PartialEq assumes: standard leaf-type pattern, justified.
- 2 external: Verus &mut limitation, justified.
- 7 ninject_par assumes: all for clone-view preservation and Arc opacity. The concurrent
  lock-invariant proof itself is fully verified. These could be closed with better Verus
  support for generic Clone specs and Arc transparency.

## Table of Contents / In-Out Table

All 7 files have TOC comments and section headers. Sections are in standard order.

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Other |
|---|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|-------|
| 1 | ArraySeq | in | in | - | - | in | out | out | iter_mut, IntoIterator &mut: out (external) |
| 2 | ArraySeqStEph | in | in | - | - | in | out | out | - |
| 3 | ArraySeqStPer | in | in | - | - | in | out | out | - |
| 4 | ArraySeqMtEph | in | in | - | - | in | out | out | NinjectInv + apply_ninject_updates (in) |
| 5 | ArraySeqMtPer | in | in | - | - | in | out | out | - |
| 6 | LinkedListStEph | in | in | - | - | in | out | out | - |
| 7 | LinkedListStPer | in | in | - | - | in | out | out | - |

All placements correct.

## Summary

Chap18 defines the Sequence ADT with 18 operations. The code provides
7 implementations (1 base + 4 ArraySeq variants + 2 LinkedList variants)
with 8287 lines total:

- **Spec fidelity:** 20 of 20 implementable prose operations implemented. All strong.
- **Module independence:** All 6 variants are independent of ArraySeq.rs.
- **Proof holes:** 16 total (14 assume, 2 external). 7 PartialEq + 2 &mut external are
  standard justified patterns. 7 ninject_par assumes are clone/Arc opacity — the
  concurrent invariant proof is fully verified.
- **RTTs:** 99 tests across 7 files, all pass. Every module has dedicated RTTs.
- **PTTs:** 26 tests across 7 files, all pass.
- **Cost annotations:** All 7 files carry dual APAS/Claude-Opus-4.6 annotations.
- **Parallelism:** Mt modules provide genuine parallel map, filter, reduce via HFScheduler.
  MtEph also provides ninject_par via Arc\<RwLock\> + HF join.
- **TOC/in-out:** All correct.

**Action items:**

| # | Priority | Item |
|---|----------|------|
| 1 | Low | Close 7 ninject_par clone/Arc assumes when Verus support improves |
