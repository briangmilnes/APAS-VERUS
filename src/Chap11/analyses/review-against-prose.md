# Review Against Prose -- Chapter 11: Threads, Concurrency, and Parallelism

## Phase 1: Inventory

Source: `src/Chap11/analyses/veracity-review-module-fn-impls.md`

| # | Chap | File | Function | Kind | V! | SpecStr | Holes |
|---|------|------|----------|------|----|---------|-------|
| 1 | 11 | FibonacciStEph.rs | spec_fib | spec ML | Y | open | 0 |
| 2 | 11 | FibonacciStEph.rs | lemma_fib_bound | proof ML | Y | strong | 0 |
| 3 | 11 | FibonacciStEph.rs | lemma_fib_fits_u64 | proof ML | Y | strong | 0 |
| 4 | 11 | FibonacciStEph.rs | lemma_fib_sum_fits_u64 | proof ML | Y | strong | 0 |
| 5 | 11 | FibonacciStEph.rs | fib | exec ML | Y | strong | 0 |
| 6 | 11 | FibonacciStEph.rs | fib_recursive | exec ML | Y | strong | 0 |
| 7 | 11 | FibonacciMtEph2Threads.rs | fib_2threads | exec ML | Y | strong | 0 |
| 8 | 11 | FibonacciMtEphRecomputes.rs | fib_recomputes | exec ML | Y | strong | 0 |
| 9 | 11 | FibonacciMtPerAllThreads.rs | fib | exec ML | Y | strong | 0 |
| 10 | 11 | FibonacciMtPerTSM.rs | fib | exec ML | Y | strong | 0 |

Proof holes: 0. All modules clean. The accept() annotations on thread-join error arms
(6 total: 2 each in MtEph2Threads, MtEphRecomputes, MtPerTSM) are the standard
`accept(false); diverge()` pattern for unreachable thread-join error arms and are
classified as informational, not holes.

FibonacciMtPerAllThreads uses `ParaPair!` macro (no thread-join error arms) and has
zero annotations of any kind.

## Phase 2: Prose Inventory

Source: `prompts/Chap11.txt` (APAS Chapter 11)

### Definitions

| # | ID | Name | Covered |
|---|-----|------|---------|
| 1 | 11.1 | Thread (spawn, sync) | Yes -- Mt files implement spawn/sync |
| 2 | 11.2 | Thread Scheduler | N/A -- concept, not code |
| 3 | 11.3 | Concurrency | N/A -- concept |
| 4 | 11.4 | Parallelism | N/A -- concept |
| 5 | 11.5 | Sequential Elision | Yes -- FibonacciStEph.fib_recursive |
| 6 | 11.6 | Data Race | N/A -- concept |

### Examples (algorithmic content)

| # | ID | Description | Covered |
|---|-----|-------------|---------|
| 1 | 11.1 | Spawn two threads for fib(n), fib(2n) | Yes -- MtEph2Threads |
| 2 | 11.9 | Parallel Fibonacci with refs | Yes -- MtEph2Threads, MtEphRecomputes |
| 3 | 11.10 | Parallel Fibonacci in SPARC | Yes -- MtPerAllThreads, MtPerTSM |
| 4 | 11.11 | Sequential elision of fib | Yes -- FibonacciStEph.fib_recursive |

### Algorithms

| # | Algorithm | File(s) | Notes |
|---|-----------|---------|-------|
| 1 | Sequential iterative fib | FibonacciStEph.rs (fib) | Not in prose; project addition |
| 2 | Sequential recursive fib | FibonacciStEph.rs (fib_recursive) | Ex 11.11 sequential elision |
| 3 | 2-thread parallel fib | FibonacciMtEph2Threads.rs | Ex 11.1/11.2 pattern (top-level fork only) |
| 4 | Full recursive parallel fib (TSM) | FibonacciMtEphRecomputes.rs | Ex 11.9/11.10 |
| 5 | Full recursive parallel fib (ParaPair) | FibonacciMtPerAllThreads.rs | Ex 11.10 |
| 6 | Full recursive parallel fib (TSM v2) | FibonacciMtPerTSM.rs | Ex 11.10 |

### Cost Specs from Prose

The prose does not give explicit cost specifications for Fibonacci. The cost analysis
is implicit:
- Sequential recursive fib: Work = Span = Theta(phi^n) (exponential).
- Parallel recursive fib: Work Theta(phi^n), Span Theta(n).
- The chapter focuses on concepts (threads, concurrency, parallelism, races) rather
  than cost theorems.

### Theorems

No formal theorems in Chapter 11. The chapter is conceptual, introducing threads,
concurrency vs parallelism, mutable state, and data races through examples.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All exec functions already have cost annotations in the required format. Verified:

| # | Chap | File | Function | APAS Cost | Agent Cost | Status |
|---|------|------|----------|-----------|------------|--------|
| 1 | 11 | FibonacciStEph.rs | fib | W Theta(n), S Theta(n) | agrees | present |
| 2 | 11 | FibonacciStEph.rs | fib_recursive | W Theta(phi^n), S Theta(phi^n) | agrees | present |
| 3 | 11 | FibonacciMtEph2Threads.rs | fib_2threads | W Theta(phi^n), S Theta(n) | W Theta(n), S Theta(n) | present |
| 4 | 11 | FibonacciMtEphRecomputes.rs | fib_recomputes | W Theta(phi^n), S Theta(n) | agrees | present |
| 5 | 11 | FibonacciMtPerAllThreads.rs | fib | W Theta(phi^n), S Theta(n) | agrees | present |
| 6 | 11 | FibonacciMtPerTSM.rs | fib | W Theta(phi^n), S Theta(n) | agrees | present |

Row 3 note: The APAS annotation says "the recursive parallel version (Ex 11.10)" but
the implementation is NOT Ex 11.10. `fib_2threads` does a single top-level fork calling
iterative `fib` on each branch, yielding Work Theta(n) and Span Theta(n). The doc
comment already flags this discrepancy. This is a design choice (more efficient variant),
not a bug.

No new annotations needed.

### Phase 3b: Implementation Deviations

| # | Chap | File | Deviation | Severity |
|---|------|------|-----------|----------|
| 1 | 11 | FibonacciStEph.rs | `fib` is iterative (not in prose). Project addition for efficiency. | None -- beneficial |
| 2 | 11 | FibonacciMtEph2Threads.rs | Top-level fork only with iterative sub-calls; prose Ex 11.10 is fully recursive. | Low -- intentional variant |
| 3 | 11 | FibonacciMtEphRecomputes.rs | Uses TSM tokens for fork-join proof; prose has no verification mechanism. | None -- verification infrastructure |
| 4 | 11 | FibonacciMtPerAllThreads.rs | Uses ParaPair! macro; prose uses raw spawn/sync. | None -- macro abstracts spawn/sync |
| 5 | 11 | FibonacciMtPerTSM.rs | Uses TSM tokens (simpler variant: no expected_left/right constants). | None -- verification infrastructure |

### Phase 3c: Ensures vs Prose

All exec functions ensure `fibonacci == spec_fib(n as nat)`, which matches the prose
definition of Fibonacci exactly. The spec function `spec_fib` directly encodes
fib(0)=0, fib(1)=1, fib(n) = fib(n-1) + fib(n-2) for n>=2.

| # | Chap | File | Function | ensures | Prose Match |
|---|------|------|----------|---------|-------------|
| 1 | 11 | FibonacciStEph.rs | fib | fibonacci == spec_fib(n) | exact |
| 2 | 11 | FibonacciStEph.rs | fib_recursive | fibonacci == spec_fib(n) | exact |
| 3 | 11 | FibonacciMtEph2Threads.rs | fib_2threads | fibonacci == spec_fib(n) | exact |
| 4 | 11 | FibonacciMtEphRecomputes.rs | fib_recomputes | fibonacci == spec_fib(n) | exact |
| 5 | 11 | FibonacciMtPerAllThreads.rs | fib | fibonacci == spec_fib(n) | exact |
| 6 | 11 | FibonacciMtPerTSM.rs | fib | fibonacci == spec_fib(n) | exact |

## Phase 4: Parallelism Review

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|----------------|-------|
| 1 | 11 | FibonacciMtEph2Threads.rs | fib_2threads | parallel | 2 threads via vstd::thread::spawn, TSM proof |
| 2 | 11 | FibonacciMtEphRecomputes.rs | fib_recomputes | parallel | recursive 2-thread fork per level, TSM proof |
| 3 | 11 | FibonacciMtPerAllThreads.rs | fib | parallel | recursive ParaPair! fork per level |
| 4 | 11 | FibonacciMtPerTSM.rs | fib | parallel | recursive 2-thread fork per level, TSM proof |

All Mt files are genuinely parallel. No sequential workarounds. No sequentialization.

Verification approach differences:
- **MtEph2Threads**: TSM `FibPair` with `expected_left`/`expected_right` constant fields.
  Top-level fork only; each thread calls iterative `fib`.
- **MtEphRecomputes**: TSM `FibFork` (same structure as FibPair). Full recursive
  parallelism; each thread calls `fib_recomputes` recursively.
- **MtPerAllThreads**: Uses `ParaPair!` macro with named closures carrying explicit
  `requires`/`ensures`. Fully verified with no accept() annotations. The cleanest
  implementation.
- **MtPerTSM**: TSM `FibForkJoin` (simpler: no expected_left/right constants, just
  completion flags). Full recursive parallelism.

## Phase 5: Runtime Test Review

Source: `tests/Chap11/TestFibonacciMt.rs`, `tests/Chap11/TestParaPairs.rs`

### TestFibonacciMt.rs

| # | Test | What it covers |
|---|------|----------------|
| 1 | test_fib_base_cases | fib(0)=0, fib(1)=1 |
| 2 | test_fib_small | fib(2..10) |
| 3 | test_fib_medium | fib(15,20,25,30) |
| 4 | test_fib_large | fib(35,40,45,46) |
| 5 | test_fib_known_values | 12 known values, fib(0)..fib(46) |

Coverage: FibonacciStEph.fib is thoroughly tested. `fib_recursive` is not directly
tested but is verified to produce the same result as `fib` (both ensure
`fibonacci == spec_fib(n as nat)`).

The Mt variants (MtEph2Threads, MtEphRecomputes, MtPerAllThreads, MtPerTSM) cannot be
tested at runtime because they are gated behind `#![cfg(verus_keep_ghost)]` or
`#[cfg(verus_keep_ghost)]` -- they use TSM tokens and `Tracked` types that exist only
during Verus compilation. The test file documents this limitation.

### TestParaPairs.rs

| # | Test | What it covers |
|---|------|----------------|
| 1 | test_set_parallelism | HFScheduler configuration |
| 2 | test_para_pair_simple | ParaPair! with simple arithmetic |
| 3 | test_para_pair_strings | ParaPair! with String types |
| 4 | test_para_pair_compute | ParaPair! with fib(10), fib(15) |
| 5 | test_para_pair_different_types | ParaPair! with heterogeneous types |
| 6 | test_para_pair_nested | Nested ParaPair! calls |

This tests the ParaPair! infrastructure used by FibonacciMtPerAllThreads. The
`test_para_pair_compute` test exercises `fib` through the parallel path.

### Coverage Assessment

- FibonacciStEph.fib: strong coverage (5 tests, 12+ values).
- FibonacciStEph.fib_recursive: no direct RTT (verified equivalent to fib).
- FibonacciMtEph2Threads: no RTT (verus_keep_ghost gated).
- FibonacciMtEphRecomputes: no RTT (verus_keep_ghost gated).
- FibonacciMtPerAllThreads: no direct RTT (verus_keep_ghost gated), but ParaPair! is
  tested via TestParaPairs.rs.
- FibonacciMtPerTSM: no RTT (verus_keep_ghost gated).

Missing: A test for `fib_recursive` would be easy to add but is not critical given
verification ensures equivalence.

## Phase 6: PTT Review

No PTTs exist for Chap11. None are needed:
- The functions are simple (no complex requires that callers must satisfy).
- No iterators.
- The ParaPair! macro is tested via TestParaPairs.rs at runtime.

## Phase 7: Gap Analysis

### Proof Gaps

None. All 5 modules are clean with 0 proof holes.

### Spec Gaps

None. All exec functions have strong specs: `ensures fibonacci == spec_fib(n as nat)`.

### Prose Coverage Gaps

| # | Gap | Severity | Notes |
|---|-----|----------|-------|
| 1 | No data race demonstration | N/A | Chap11 Sec 3 discusses races conceptually; no algorithm to implement |
| 2 | No thread scheduler implementation | N/A | Def 11.2 is a concept; HFScheduler in Chap02 serves this role |
| 3 | No fib(n) + fib(2n) top-level example | Low | Ex 11.1 computes fib(n) and fib(2n); MtEph2Threads computes fib(n-1) and fib(n-2) instead |

### Style Warnings (from veracity-review-verus-style.log)

| # | Chap | File | Warning | Severity |
|---|------|------|---------|----------|
| 1 | 11 | FibonacciStEph.rs | [22] free spec fn spec_fib should be abstract signature in trait | Low |
| 2 | 11 | All 5 files | [24] copyright format | Cosmetic |

Warning 1 is notable: `spec_fib` is a free `open spec fn` rather than an abstract
trait signature with body in the impl. This is acceptable for a spec function shared
across multiple modules (it is imported by all Mt files). Moving it into a trait would
complicate cross-module imports without benefit.

### Structural Notes

- No trait-impl pattern for FibonacciStEph exec functions. The functions are free
  module-level functions, not trait methods. This is acceptable for standalone algorithm
  demonstrations that do not represent a data structure.
- FibonacciMtEph2Threads and FibonacciMtEphRecomputes use `#![cfg(verus_keep_ghost)]`
  (file-level). FibonacciMtPerAllThreads uses `#[cfg(verus_keep_ghost)]` (module-level).
  FibonacciMtPerTSM uses `#![cfg(verus_keep_ghost)]` (file-level). This inconsistency
  is cosmetic but could be unified.

## Phase 8: TOC Review

### FibonacciStEph.rs

Present TOC:
```
//  Table of Contents
//  1. module
//  6. spec fns
//  7. proof fns/broadcast groups
//  9. impls
```

Actual structure: module (line 15), imports (lines 16-24), spec fns (line 32),
proof fns (lines 48-81), exec fns (lines 89-142). Section 9 header says "impls" but
contains free exec functions, not impl blocks. Should be "9. impls" only if there were
trait impls; these are module-level functions. Minor labeling issue.

Missing section 2 (imports) in the TOC header.

### FibonacciMtEph2Threads.rs

No TOC present. Should have one (sections 1, 2, 6/TSM, 9).

### FibonacciMtEphRecomputes.rs

No TOC present. Should have one.

### FibonacciMtPerAllThreads.rs

No TOC present. Should have one.

### FibonacciMtPerTSM.rs

No TOC present. Should have one.

### TOC Summary

| # | Chap | File | TOC Present | Correct |
|---|------|------|-------------|---------|
| 1 | 11 | FibonacciStEph.rs | Yes | Minor: section 2 missing, section 9 label |
| 2 | 11 | FibonacciMtEph2Threads.rs | No | -- |
| 3 | 11 | FibonacciMtEphRecomputes.rs | No | -- |
| 4 | 11 | FibonacciMtPerAllThreads.rs | No | -- |
| 5 | 11 | FibonacciMtPerTSM.rs | No | -- |

## Summary

Chapter 11 is in excellent shape. All 5 modules verify cleanly with 0 proof holes.
Every exec function has a strong spec (`ensures fibonacci == spec_fib(n as nat)`).
Four different parallelization strategies are implemented, all genuinely parallel.
The ParaPair! variant (FibonacciMtPerAllThreads) is the cleanest, with zero accept()
annotations.

### Actionable Items

| # | Priority | Item |
|---|----------|------|
| 1 | Low | Add TOC headers to the 4 Mt files |
| 2 | Low | Add section 2 (imports) to FibonacciStEph.rs TOC |
| 3 | Low | Add a direct RTT for fib_recursive |
| 4 | Cosmetic | Unify cfg gating style across Mt files |

---

Date: 2026-03-15
Reviewer: Claude-Opus-4.6, Agent 1
